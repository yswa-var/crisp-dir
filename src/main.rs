use clap::{Parser, Subcommand};
use std::fs;
use std::env;
use std::io;
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// rename files by replacing spaces "_" and removing "()"
    #[command(name = "clean-names")]
    CleanNames,

    /// list all file types in the current directory and their counts
    #[command(name = "list-file-types")]
    ListFileTypes,

    /// organize files into directories based on their file types
    #[command(name = "organize-files")]
    OrganizeFiles,
}

fn organize_files() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let image_exts = ["jpg", "jpeg", "gif", "png"];
    let document_exts = ["pdf", "epub", "txt", "csv", "md", "torrent", "xlsx", "xls", "docs", "docx", "doc"];
    let video_exts = ["mp4", "avi", "mov", "mkv", "webm", "m4v"]; // Add more as needed
    
    let image_dir = current_dir.join("images");
    let document_dir = current_dir.join("documents");
    let video_dir = current_dir.join("videos");
    let other_dir = current_dir.join("others");
    fs::create_dir_all(&image_dir)?;
    fs::create_dir_all(&document_dir)?;
    fs::create_dir_all(&video_dir)?;
    fs::create_dir_all(&other_dir)?;
    
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                let target_dir = if image_exts.contains(&extension.to_lowercase().as_str()) {
                    &image_dir
                } else if document_exts.contains(&extension.to_lowercase().as_str()) {
                    &document_dir
                } else if video_exts.contains(&extension.to_lowercase().as_str()) {
                    &video_dir
                } else {
                    &other_dir
                };

                let new_path = target_dir.join(path.file_name().unwrap());
                fs::rename(&path, &new_path)?;
            }
        }
    }

    println!("Files have been organized.");
    Ok(())
}

/// list all file types in the current directory
fn list_file_types() -> io::Result<()> {
    let mut file_types = HashMap::new();

    let current_dir = env::current_dir()?;
    println!("scanning files in directory: {}", current_dir.display());

    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(_file_name_original) = path.file_name() {
                let file_type = path.extension();
                if let Some(file_type) = file_type {
                    file_types.entry(file_type.to_string_lossy().to_string()).and_modify(|count| *count += 1).or_insert(1);
                }
            }
        }
    }

    for (file_type, count) in file_types {
        println!("{}: {}", file_type, count);
    }

    Ok(())
}

/// rename files in the current directory
fn clean_file_names() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("scanning files in directory: {}", current_dir.display());

    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name_original) = path.file_name() {
                if let Some(file_name) = file_name_original.to_str() {

                    // create the new, clean file name
                    let new_file_name = file_name
                        .replace(" ", "_")
                        .replace("(", "")
                        .replace(")", "");

                    // skip if the new file name is the same as the original
                    if new_file_name != file_name {
                        let new_path = path.with_file_name(new_file_name);
                        println!("renaming {} to {}", path.display(), new_path.display());
                        fs::rename(path, new_path)?;
                    }
                }
            }
        }
    }
    println!("done!");
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CleanNames => {
            if let Err(e) = clean_file_names() {
                eprintln!("Error: {}", e);
            }
        },
        Commands::ListFileTypes => {
            if let Err(e) = list_file_types() {
                eprintln!("Error: {}", e);
            }
        },
        Commands::OrganizeFiles => {
            if let Err(e) = organize_files() {
                eprintln!("Error: {}", e);
            }
        },
    }
}