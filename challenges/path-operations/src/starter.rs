use std::path::{Path, PathBuf};

/// Joins a base path with multiple path components.
///
/// # Arguments
///
/// * `base` - The base path as a string
/// * `parts` - A slice of path components to join
///
/// # Returns
///
/// A `PathBuf` containing the joined path
pub fn join_paths(base: &str, parts: &[&str]) -> PathBuf {
    // TODO: Create a PathBuf from base and join all parts
    // Hint: Use PathBuf::from() and the .push() method
    unimplemented!()
}

/// Extracts the file extension from a path (without the leading dot).
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the extension, or `None` if no extension exists
pub fn get_extension(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .extension() method
    // Remember to convert OsStr to String
    unimplemented!()
}

/// Extracts the file name (including extension) from a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the file name, or `None` if the path ends with `..`
pub fn get_file_name(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .file_name() method
    unimplemented!()
}

/// Extracts the file stem (file name without extension) from a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(String)` containing the file stem, or `None` if no file name exists
pub fn get_file_stem(path: &str) -> Option<String> {
    // TODO: Use Path::new() and the .file_stem() method
    unimplemented!()
}

/// Gets the parent directory of a path.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `Some(PathBuf)` containing the parent directory, or `None` if the path has no parent
pub fn get_parent(path: &str) -> Option<PathBuf> {
    // TODO: Use Path::new() and the .parent() method
    // Consider: what should happen for "/" or "file.txt"?
    unimplemented!()
}

/// Changes the extension of a path.
///
/// # Arguments
///
/// * `path` - The original path as a string
/// * `new_ext` - The new extension (without the leading dot)
///
/// # Returns
///
/// A `PathBuf` with the new extension
pub fn change_extension(path: &str, new_ext: &str) -> PathBuf {
    // TODO: Create a PathBuf and use .set_extension()
    unimplemented!()
}

/// Checks if a path is absolute.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// `true` if the path is absolute, `false` otherwise
pub fn is_absolute(path: &str) -> bool {
    // TODO: Use Path::new() and the .is_absolute() method
    unimplemented!()
}

/// Normalizes a path to use the platform's native separators.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// A `PathBuf` with platform-appropriate separators
pub fn normalize_path(path: &str) -> PathBuf {
    // TODO: PathBuf::from() handles normalization automatically
    unimplemented!()
}

// Example usage
pub fn main() {
    // Join paths
    let full_path = join_paths("/home/user", &["documents", "reports"]);
    println!("Joined path: {:?}", full_path);

    // Get extension
    let ext = get_extension("document.pdf");
    println!("Extension: {:?}", ext);

    // Get file name
    let name = get_file_name("/home/user/document.pdf");
    println!("File name: {:?}", name);

    // Get parent
    let parent = get_parent("/home/user/document.pdf");
    println!("Parent: {:?}", parent);

    // Change extension
    let new_path = change_extension("report.doc", "pdf");
    println!("New extension: {:?}", new_path);

    // Check if absolute
    println!("Is absolute '/home': {}", is_absolute("/home"));
    println!("Is absolute 'relative': {}", is_absolute("relative"));
}
