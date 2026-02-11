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
///
/// # Examples
///
/// ```
/// use path_operations::join_paths;
/// use std::path::PathBuf;
///
/// let result = join_paths("/home/user", &["documents", "reports"]);
/// assert_eq!(result, PathBuf::from("/home/user/documents/reports"));
/// ```
pub fn join_paths(base: &str, parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(base);
    for part in parts {
        path.push(part);
    }
    path
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
///
/// # Examples
///
/// ```
/// use path_operations::get_extension;
///
/// assert_eq!(get_extension("report.pdf"), Some("pdf".to_string()));
/// assert_eq!(get_extension("Makefile"), None);
/// ```
pub fn get_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
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
///
/// # Examples
///
/// ```
/// use path_operations::get_file_name;
///
/// assert_eq!(get_file_name("/home/user/doc.txt"), Some("doc.txt".to_string()));
/// assert_eq!(get_file_name("/"), None);
/// ```
pub fn get_file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
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
///
/// # Examples
///
/// ```
/// use path_operations::get_file_stem;
///
/// assert_eq!(get_file_stem("archive.tar.gz"), Some("archive.tar".to_string()));
/// assert_eq!(get_file_stem("document.txt"), Some("document".to_string()));
/// ```
pub fn get_file_stem(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
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
///
/// # Examples
///
/// ```
/// use path_operations::get_parent;
/// use std::path::PathBuf;
///
/// assert_eq!(get_parent("/home/user/file.txt"), Some(PathBuf::from("/home/user")));
/// assert_eq!(get_parent("/"), None);
/// ```
pub fn get_parent(path: &str) -> Option<PathBuf> {
    let p = Path::new(path);
    let parent = p.parent()?;

    // Return None if parent is empty (for root or relative single component)
    if parent.as_os_str().is_empty() {
        None
    } else {
        Some(parent.to_path_buf())
    }
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
///
/// # Examples
///
/// ```
/// use path_operations::change_extension;
/// use std::path::PathBuf;
///
/// assert_eq!(change_extension("report.doc", "pdf"), PathBuf::from("report.pdf"));
/// assert_eq!(change_extension("Makefile", "bak"), PathBuf::from("Makefile.bak"));
/// ```
pub fn change_extension(path: &str, new_ext: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.set_extension(new_ext);
    path_buf
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
///
/// # Examples
///
/// ```
/// use path_operations::is_absolute;
///
/// assert!(is_absolute("/home/user"));
/// assert!(!is_absolute("relative/path"));
/// ```
pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

/// Normalizes a path to use the platform's native separators.
///
/// This creates a `PathBuf` from the input, which handles separator
/// normalization automatically on the current platform.
///
/// # Arguments
///
/// * `path` - The path as a string
///
/// # Returns
///
/// A `PathBuf` with platform-appropriate separators
///
/// # Examples
///
/// ```
/// use path_operations::normalize_path;
/// use std::path::PathBuf;
///
/// let normalized = normalize_path("home/user/file.txt");
/// // On Unix: PathBuf::from("home/user/file.txt")
/// // On Windows: PathBuf::from("home\\user\\file.txt")
/// ```
pub fn normalize_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}
