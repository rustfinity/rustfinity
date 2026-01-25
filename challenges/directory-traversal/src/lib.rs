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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::list_files;
/// use std::path::Path;
///
/// let files = list_files(Path::new("./src")).unwrap();
/// for file in files {
///     println!("{}", file.display());
/// }
/// ```
pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::list_directories;
/// use std::path::Path;
///
/// let dirs = list_directories(Path::new("./project")).unwrap();
/// for dir in dirs {
///     println!("{}", dir.display());
/// }
/// ```
pub fn list_directories(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut directories = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            directories.push(path);
        }
    }
    Ok(directories)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::list_all_recursive;
/// use std::path::Path;
///
/// let all_paths = list_all_recursive(Path::new("./src")).unwrap();
/// for path in all_paths {
///     println!("{}", path.display());
/// }
/// ```
pub fn list_all_recursive(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        results.push(path.clone());
        if path.is_dir() {
            let mut sub_results = list_all_recursive(&path)?;
            results.append(&mut sub_results);
        }
    }
    Ok(results)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::find_by_extension;
/// use std::path::Path;
///
/// let rust_files = find_by_extension(Path::new("./"), "rs").unwrap();
/// for file in rust_files {
///     println!("Found Rust file: {}", file.display());
/// }
/// ```
pub fn find_by_extension(dir: &Path, ext: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut sub_results = find_by_extension(&path, ext)?;
            results.append(&mut sub_results);
        } else if path.extension().map_or(false, |e| e == ext) {
            results.push(path);
        }
    }
    Ok(results)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::find_by_name;
/// use std::path::Path;
///
/// let cargo_files = find_by_name(Path::new("./"), "Cargo.toml").unwrap();
/// for file in cargo_files {
///     println!("Found: {}", file.display());
/// }
/// ```
pub fn find_by_name(dir: &Path, name: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().map_or(false, |n| n == name) {
            results.push(path.clone());
        }
        if path.is_dir() {
            let mut sub_results = find_by_name(&path, name)?;
            results.append(&mut sub_results);
        }
    }
    Ok(results)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::calculate_dir_size;
/// use std::path::Path;
///
/// let size = calculate_dir_size(Path::new("./src")).unwrap();
/// println!("Total size: {} bytes", size);
/// ```
pub fn calculate_dir_size(dir: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            total_size += calculate_dir_size(&path)?;
        } else {
            total_size += entry.metadata()?.len();
        }
    }
    Ok(total_size)
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
///
/// # Examples
///
/// ```no_run
/// use directory_traversal::count_files_recursive;
/// use std::path::Path;
///
/// let count = count_files_recursive(Path::new("./src")).unwrap();
/// println!("Found {} files", count);
/// ```
pub fn count_files_recursive(dir: &Path) -> io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            count += count_files_recursive(&path)?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

pub fn main() {
    use std::fs::{self, File};
    use std::io::Write;

    // Create a temporary test directory structure
    let test_dir = Path::new("test_traversal");

    // Create test structure
    let _ = fs::create_dir_all(test_dir.join("subdir1"));
    let _ = fs::create_dir_all(test_dir.join("subdir2"));

    // Create test files
    let _ = File::create(test_dir.join("file1.txt")).and_then(|mut f| f.write_all(b"Hello"));
    let _ = File::create(test_dir.join("file2.rs")).and_then(|mut f| f.write_all(b"fn main() {}"));
    let _ = File::create(test_dir.join("subdir1").join("nested.txt")).and_then(|mut f| f.write_all(b"Nested content"));
    let _ = File::create(test_dir.join("subdir1").join("code.rs")).and_then(|mut f| f.write_all(b"// code"));

    println!("=== Directory Traversal Examples ===\n");

    // List files (non-recursive)
    println!("Files in test_traversal:");
    if let Ok(files) = list_files(test_dir) {
        for f in &files {
            println!("  {}", f.display());
        }
    }

    // List directories
    println!("\nDirectories in test_traversal:");
    if let Ok(dirs) = list_directories(test_dir) {
        for d in &dirs {
            println!("  {}", d.display());
        }
    }

    // List all recursively
    println!("\nAll paths (recursive):");
    if let Ok(all) = list_all_recursive(test_dir) {
        for p in &all {
            println!("  {}", p.display());
        }
    }

    // Find by extension
    println!("\nRust files (.rs):");
    if let Ok(rs_files) = find_by_extension(test_dir, "rs") {
        for f in &rs_files {
            println!("  {}", f.display());
        }
    }

    // Calculate size
    if let Ok(size) = calculate_dir_size(test_dir) {
        println!("\nTotal size: {} bytes", size);
    }

    // Count files
    if let Ok(count) = count_files_recursive(test_dir) {
        println!("Total files: {}", count);
    }

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
    println!("\nTest directory cleaned up.");
}
