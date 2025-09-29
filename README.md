# 🗂️ Tidy-rs: CLI File Organizer

A command-line tool that automatically organizes files in a given directory into subfolders based on rules (e.g., by file extension, by date, or by custom patterns).

## Features (Progressive Difficulty)

### Basic
- Take a directory path as input
- Scan all files in the directory
- Move `.jpg` and `.png` into an `images/` folder, `.mp3` into `music/`, etc.

### Intermediate
- Allow the user to configure rules in a `.toml` file (e.g., "pdf → documents")
- Handle errors gracefully (e.g., when a file can't be moved)

### Advanced
- Add a `--dry-run` option that shows what would happen without moving files
- Add logging with levels (info, warn, error)
- Implement concurrency (use Rayon or async) to process large directories faster

## Why This Project?

This project helps you practice:
- Ownership and borrowing with file paths
- Working with the `std::fs` API (and maybe `walkdir` crate)
- Using structs and traits to represent rules
- Error handling with `Result` and `?`
- Making it as simple or advanced as you like
