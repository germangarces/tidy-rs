use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Directory to organize
    directory: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    extensions: HashMap<String, String>,
}

fn folder_for_extension(ext: &str) -> &str {
    match ext {
        "png" | "jpg" | "jpeg" => "images",
        "mp3" | "wav" | "m4a" => "music",
        "pdf" | "doc" | "docx" | "txt" => "documents",
        _ => "other",
    }
}

fn move_file_to_folder(path: &Path) -> Result<(), Box<dyn Error>> {
    let extension = path
        .extension()
        .unwrap_or(OsStr::new("txt"))
        .to_str()
        .unwrap()
        .to_lowercase();
    let folder = folder_for_extension(&extension);

    let mut new_path = path.parent().unwrap().to_path_buf();
    new_path.push(folder);
    fs::create_dir_all(&new_path)?;
    new_path.push(path.file_name().unwrap());
    fs::rename(path, &new_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    // Scan all files in the directory
    for entry in fs::read_dir(args.directory)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_file() {
            if let Err(e) = move_file_to_folder(&path) {
                eprintln!("Error moving {:?} {}", path, e)
            }
        }
    }

    Ok(())
}
