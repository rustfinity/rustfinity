use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Lists all files (not directories) in a directory.
/// This is non-recursive - only immediate children are listed.
///
/// # Arguments
///
/// * `dir` - The directory path to list files from
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of file paths, or an `io::Error`
pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    // TODO: Use fs::read_dir() to iterate over directory entries
    // TODO: Filter to include only files (not directories)
    // TODO: Collect the paths into a Vec
    unimplemented!()
}

/// Lists all subdirectories in a directory.
/// This is non-recursive - only immediate children are listed.
///
/// # Arguments
///
/// * `dir` - The directory path to list subdirectories from
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of directory paths, or an `io::Error`
pub fn list_directories(dir: &Path) -> io::Result<Vec<PathBuf>> {
    // TODO: Use fs::read_dir() to iterate over directory entries
    // TODO: Filter to include only directories (not files)
    // TODO: Collect the paths into a Vec
    unimplemented!()
}

/// Recursively lists all files and directories in a directory tree.
///
/// # Arguments
///
/// * `dir` - The root directory to traverse
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of all paths (files and directories), or an `io::Error`
pub fn list_all_recursive(dir: &Path) -> io::Result<Vec<PathBuf>> {
    // TODO: Iterate over directory entries
    // TODO: Add each entry's path to results
    // TODO: If entry is a directory, recursively call this function
    // TODO: Append recursive results to the main results
    unimplemented!()
}

/// Finds all files with a given extension recursively.
///
/// # Arguments
///
/// * `dir` - The root directory to search
/// * `ext` - The file extension to match (without the leading dot)
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of matching file paths, or an `io::Error`
pub fn find_by_extension(dir: &Path, ext: &str) -> io::Result<Vec<PathBuf>> {
    // TODO: Recursively search through directories
    // TODO: Check if each file's extension matches using path.extension()
    // TODO: Collect matching files
    unimplemented!()
}

/// Finds all files or directories matching a name exactly (recursive).
///
/// # Arguments
///
/// * `dir` - The root directory to search
/// * `name` - The exact file or directory name to match
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of matching paths, or an `io::Error`
pub fn find_by_name(dir: &Path, name: &str) -> io::Result<Vec<PathBuf>> {
    // TODO: Recursively search through directories
    // TODO: Check if each entry's file_name() matches the target name
    // TODO: Collect matching paths (both files and directories)
    unimplemented!()
}

/// Calculates the total size of all files in a directory tree.
///
/// # Arguments
///
/// * `dir` - The root directory to calculate size for
///
/// # Returns
///
/// A `Result` containing the total size in bytes, or an `io::Error`
pub fn calculate_dir_size(dir: &Path) -> io::Result<u64> {
    // TODO: Recursively traverse the directory tree
    // TODO: For files, add their size (using entry.metadata()?.len())
    // TODO: For directories, recursively calculate their size
    // TODO: Return the total
    unimplemented!()
}

/// Counts all files (not directories) in a directory tree recursively.
///
/// # Arguments
///
/// * `dir` - The root directory to count files in
///
/// # Returns
///
/// A `Result` containing the file count, or an `io::Error`
pub fn count_files_recursive(dir: &Path) -> io::Result<usize> {
    // TODO: Recursively traverse the directory tree
    // TODO: Count files (not directories)
    // TODO: Return the total count
    unimplemented!()
}

pub fn main() {
    // Example usage - this will work once you implement the functions
    let test_dir = Path::new(".");

    println!("Listing files in current directory:");
    match list_files(test_dir) {
        Ok(files) => {
            for file in files {
                println!("  File: {}", file.display());
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nListing directories:");
    match list_directories(test_dir) {
        Ok(dirs) => {
            for dir in dirs {
                println!("  Dir: {}", dir.display());
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nFinding .rs files:");
    match find_by_extension(test_dir, "rs") {
        Ok(files) => {
            for file in files {
                println!("  {}", file.display());
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nCounting files recursively:");
    match count_files_recursive(test_dir) {
        Ok(count) => println!("  Total files: {}", count),
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nCalculating directory size:");
    match calculate_dir_size(test_dir) {
        Ok(size) => println!("  Total size: {} bytes", size),
        Err(e) => println!("  Error: {}", e),
    }
}
