extern crate colored;
// extern crate uuid;
use colored::*;
use log::info;
use normpath::PathExt;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process;

// This function's input called "path", is an input directory 'string'.
// This function's output called "files", is a 'vector' containing all the files paths as 'PathBufs' (owned, mutable paths).
pub fn get_files_in_directory(path: &str) -> Vec<PathBuf> {
    // Creates an empty mutable 'vector' of 'PathBufs' called "files"
    let mut files: Vec<PathBuf> = Vec::new();
    // Creates an iterator called "directory" over the entries in "path" with files returned as a 'DirEntry'.
    let directory = PathBuf::from(path).read_dir();
    // Checks "directory" if there was an error creating the iterator. Prints error. Exits function true.
    if directory.is_err() {
        eprintln!("{} {}", "Directory not found:".red().bold(), path);
        process::exit(1);
    }
    // Loop through "directory" for each 'DirEntry', get the full path, and push into the "files" vector.
    for entry in directory.unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        files.push(path);
    }
    // Return the "files" vector.
    files
}

// This function's input called "path", is a 'PathBuf'.
// This function's outputs a 'PathBuf'.
pub fn normalize_and_create_if_needed(path: PathBuf) -> PathBuf {
    // Creates a mutable clone() of "path" normalized called "normalized_path". If it fails it will create the directory and return it normalized. 
    let mut normalized_path = match path.clone().normalize() {
        Ok(path) => path,
        Err(_) => {
            info!(
                "{} directory does not exist, attempting to create it now...",
                path.to_string_lossy().blue().bold()
            );
            let path = path;
            let normalized_path = create_dir(path.clone().to_path_buf()).normalize().unwrap();
            normalized_path
        }
    };
    // Checks if the "normalized_path" exists. Returns message that it's exists. Check one more time that that path is normalized. 
    if normalized_path.exists() {
        info!(
            "{} directory exists, using it...",
            normalized_path.as_path().to_string_lossy().blue().bold()
        );
        normalized_path = normalized_path
            .clone()
            .normalize()
            .expect("Could not canonicalize output dir path");
    }
    // Return "normalized_path" as a 'PathBuf'.
    normalized_path.into_path_buf()
}

// This function's input called "path", is a 'PathBuf'.
// This function's outputs a 'PathBuf'.
pub fn create_dir(path: PathBuf) -> PathBuf {
    // Creates all of the parent directories if needed to create the output folder. 
    create_dir_all(path.clone()).expect("Failed to create dir");
    // Returns the path it created.
    path
}
