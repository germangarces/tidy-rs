use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Deserialize)]
struct Config {
    folders: HashMap<String, Vec<String>>,
}

fn folder_for_extension<'a>(ext: &str, config: Option<&'a Config>) -> &'a str {
    match config {
        Some(config) => {
            for (folder, extensions) in &config.folders {
                if extensions.iter().any(|e| e == ext) {
                    return folder;
                }
            }
            "other"
        }
        None => match ext {
            "png" | "jpg" | "jpeg" => "images",
            "mp3" | "wav" | "m4a" => "music",
            "pdf" | "doc" | "docx" | "txt" => "documents",
            _ => "other",
        },
    }
}

fn load_config(config_file: &Path) -> anyhow::Result<Config> {
    let content = fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn move_file_to_folder(path: &Path, config: Option<&Config>, dry_run: bool) -> anyhow::Result<()> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "txt".to_string());

    let folder = folder_for_extension(&extension, config);

    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("File has no parent directory"))?;

    let mut new_path = parent.to_path_buf();
    new_path.push(folder);
    println!("Moving {:?} to {:?}", path, new_path);
    if !dry_run {
        fs::create_dir_all(&new_path)?;

        new_path.push(
            path.file_name()
                .ok_or_else(|| anyhow::anyhow!("File has no name"))?,
        );

        fs::rename(path, &new_path)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.dry_run {
        println!("Dry run. Would move files without actually moving them.");
    }

    let config = match args.config {
        Some(config_path) => Some(load_config(&config_path)?),
        None => None,
    };

    for entry in fs::read_dir(args.directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Err(e) = move_file_to_folder(&path, config.as_ref(), args.dry_run) {
                eprintln!("Error moving {:?}: {}", path, e);
            }
        }
    }

    Ok(())
}
