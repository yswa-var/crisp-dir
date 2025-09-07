# crisp-dir

A Rust CLI tool for directory management and file organization.

## Features

- **Clean file names**: Replace spaces with underscores and remove parentheses from filenames
- **List file types**: Scan directory and display file extension counts
- **Organize files**: Automatically sort files into categorized directories (images, documents, videos, others)

## Usage

```bash
# Clean file names (replace spaces with _ and remove parentheses)
cargo run -- clean-names

# List all file types and their counts
cargo run -- list-file-types

# Organize files into directories by type
cargo run -- organize-files
```

## Installation

```bash
git clone <repository-url>
cd crisp-dir
cargo build --release
```

## Requirements

- Rust 1.70+
- clap (for CLI argument parsing)