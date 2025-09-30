use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to organize
    directory: PathBuf,

    /// Config file with rules. Toml format.
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Dry run. Show what would happen without moving files.
    #[arg(short, long)]
    dry_run: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    folders: HashMap<String, Vec<String>>,
}

fn folder_for_extension<'a>(ext: &str, config: Option<&'a Config>) -> &'a str {
    if let Some(config) = config {
        for (folder, extensions) in &config.folders {
            if extensions.iter().any(|e| e == ext) {
                return folder;
            }
        }
    }

    // Default mappings
    match ext {
        "png" | "jpg" | "jpeg" => "images",
        "mp3" | "wav" | "m4a" => "music",
        "pdf" | "doc" | "docx" | "txt" => "documents",
        _ => "other",
    }
}

async fn load_config(config_file: &Path) -> anyhow::Result<Config> {
    let content = tokio::fs::read_to_string(config_file).await?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

async fn move_file_to_folder(
    path: &Path,
    config: Option<Arc<Config>>,
    dry_run: bool,
) -> anyhow::Result<()> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "no_ext".to_string());

    let folder = folder_for_extension(&extension, config.as_deref());

    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("File has no parent directory"))?;

    let mut new_path = parent.to_path_buf();
    new_path.push(folder);
    info!("Moving {:?} to {:?}", path, new_path);
    if !dry_run {
        tokio::fs::create_dir_all(&new_path).await?;

        new_path.push(
            path.file_name()
                .ok_or_else(|| anyhow::anyhow!("File has no name"))?,
        );

        tokio::fs::rename(path, &new_path).await?;
    }
    Ok(())
}

async fn organize_directory(
    directory: PathBuf,
    config: Option<Arc<Config>>,
    dry_run: bool,
) -> anyhow::Result<()> {
    let mut dir = tokio::fs::read_dir(directory).await?;
    let mut tasks = vec![];

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            let config = config.clone(); // Arc makes it cheap to clone
            let dry_run = dry_run;

            tasks.push(task::spawn(async move {
                if let Err(e) = move_file_to_folder(&path, config, dry_run).await {
                    error!("Error moving {:?}: {}", path, e);
                }
            }));
        }
    }

    // Wait for all tasks
    for task in tasks {
        task.await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber with custom formatting
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    if args.dry_run {
        info!("Dry run. Would move files without actually moving them.");
    }

    let config = match args.config {
        Some(config_path) => {
            let cfg = load_config(&config_path).await?;
            Some(Arc::new(cfg))
        }
        None => None,
    };

    organize_directory(args.directory, config, args.dry_run).await?;

    Ok(())
}
