# 🗂️ Tidy-rs: CLI File Organizer

A fast, concurrent command-line tool that automatically organizes files into subfolders by extension.

## Usage

```bash
# Get help
tidy-rs --help

# Organize files in a directory
tidy-rs /path/to/messy/directory

# Preview what would happen (dry run)
tidy-rs --dry-run /path/to/directory

# Use custom rules from config file
tidy-rs --config rules.toml /path/to/directory
```

## Installation

```bash
git clone <repo-url>
cd tidy-rs
cargo build --release
./target/release/tidy-rs --help
```

## Default File Organization

- **Images**: `png`, `jpg`, `jpeg` → `images/`
- **Music**: `mp3`, `wav`, `m4a` → `music/`  
- **Documents**: `pdf`, `doc`, `docx`, `txt` → `documents/`
- **Other**: Everything else → `other/`

## Custom Configuration

Create a `rules.toml` file:

```toml
[folders]
images = ["png", "jpg", "jpeg", "gif", "webp"]
code = ["rs", "py", "js", "ts", "go"]
archives = ["zip", "tar", "gz", "rar"]
```

## Features

- ✅ **Concurrent processing** for fast organization
- ✅ **Dry run mode** to preview changes
- ✅ **Custom rules** via TOML configuration
- ✅ **Structured logging** with timestamps
- ✅ **Error handling** for edge cases
