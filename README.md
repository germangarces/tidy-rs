# 🗂️ Tidy-rs: CLI File Organizer

A fast, concurrent command-line tool that automatically organizes files into subfolders by extension or date.

## Usage

```bash
# Get help
tidy-rs --help

# Organize files by extension (default)
tidy-rs /path/to/messy/directory

# Organize files by creation date into year folders
tidy-rs --by-date /path/to/directory

# Preview what would happen (dry run)
tidy-rs --dry-run /path/to/directory

# Combine options: organize by date with preview
tidy-rs --by-date --dry-run /path/to/directory

# Use custom extension rules from config file
tidy-rs --config rules.toml /path/to/directory
```

## Installation

```bash
git clone <repo-url>
cd tidy-rs
cargo build --release
./target/release/tidy-rs --help
```

## Organization Modes

### By Extension (Default)
- **Images**: `png`, `jpg`, `jpeg` → `images/`
- **Music**: `mp3`, `wav`, `m4a` → `music/`  
- **Documents**: `pdf`, `doc`, `docx`, `txt` → `documents/`
- **Other**: Everything else → `other/`

### By Date (`--by-date`)
- **Files** → Organized into year folders: `2023/`, `2024/`, `2025/`
- Uses file creation date (or modification date if older)
- Ignores file extensions and config files

## Custom Configuration

Create a `rules.toml` file for extension-based organization:

```toml
[folders]
images = ["png", "jpg", "jpeg", "gif", "webp"]
code = ["rs", "py", "js", "ts", "go"]
archives = ["zip", "tar", "gz", "rar"]
```

**Note**: Custom config only works with extension-based organization (not with `--by-date`).

## Features

- **Cross-platform** file system operations
- **Robust error handling** for edge cases
- **Dry run mode** to preview changes before moving files
- **Custom rules** via TOML configuration (extension mode only)
- **Structured logging** with timestamps and log levels
- **Smart date handling** - uses older of creation/modification date
- **Cross-platform** file system operations
- **Robust error handling** for edge cases
