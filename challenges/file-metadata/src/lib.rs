use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Returns the size of a file in bytes.
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// A `Result` containing the file size in bytes, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::get_file_size;
/// use std::path::Path;
///
/// let size = get_file_size(Path::new("data.txt")).unwrap();
/// println!("File size: {} bytes", size);
/// ```
pub fn get_file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Returns the type of a filesystem entry as a string.
///
/// Returns "file", "directory", or "symlink".
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// A `Result` containing the type string, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::get_file_type;
/// use std::path::Path;
///
/// let file_type = get_file_type(Path::new("./src")).unwrap();
/// assert_eq!(file_type, "directory");
/// ```
pub fn get_file_type(path: &Path) -> io::Result<String> {
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();

    if file_type.is_symlink() {
        Ok("symlink".to_string())
    } else if file_type.is_dir() {
        Ok("directory".to_string())
    } else {
        Ok("file".to_string())
    }
}

/// Checks if a file is read-only.
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// A `Result` containing `true` if the file is read-only, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::is_readonly;
/// use std::path::Path;
///
/// let readonly = is_readonly(Path::new("config.txt")).unwrap();
/// if readonly {
///     println!("File is read-only");
/// }
/// ```
pub fn is_readonly(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().readonly())
}

/// Returns when a file was last modified.
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// A `Result` containing the modification `SystemTime`, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::get_modified_time;
/// use std::path::Path;
///
/// let modified = get_modified_time(Path::new("file.txt")).unwrap();
/// println!("Last modified: {:?}", modified);
/// ```
pub fn get_modified_time(path: &Path) -> io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    metadata.modified()
}

/// Checks if a file was modified within a given duration from now.
///
/// # Arguments
///
/// * `path` - The path to the file
/// * `duration` - The time window to check
///
/// # Returns
///
/// A `Result` containing `true` if the file was modified within the duration, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::was_modified_within;
/// use std::path::Path;
/// use std::time::Duration;
///
/// // Check if modified in the last hour
/// let recent = was_modified_within(
///     Path::new("log.txt"),
///     Duration::from_secs(3600)
/// ).unwrap();
/// ```
pub fn was_modified_within(path: &Path, duration: Duration) -> io::Result<bool> {
    let modified = get_modified_time(path)?;
    let now = SystemTime::now();

    // Calculate elapsed time since modification
    match now.duration_since(modified) {
        Ok(elapsed) => Ok(elapsed <= duration),
        Err(_) => {
            // File modification time is in the future (clock skew)
            // Consider it as "recently modified"
            Ok(true)
        }
    }
}

/// Checks if a file has execute permissions.
///
/// On Unix, this checks the execute bits in the file mode.
/// On Windows, this checks for executable extensions (.exe, .bat, .cmd, .com).
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// A `Result` containing `true` if the file is executable, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::is_executable;
/// use std::path::Path;
///
/// let executable = is_executable(Path::new("script.sh")).unwrap();
/// if executable {
///     println!("File is executable");
/// }
/// ```
pub fn is_executable(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;

    // Only files can be executable in the traditional sense
    if !metadata.is_file() {
        return Ok(false);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        // Check if any execute bit is set (owner, group, or other)
        Ok(mode & 0o111 != 0)
    }

    #[cfg(windows)]
    {
        // On Windows, check for executable extensions
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            Ok(matches!(ext_lower.as_str(), "exe" | "bat" | "cmd" | "com"))
        } else {
            Ok(false)
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        // For other platforms, we can't determine executability
        Ok(false)
    }
}

/// Compares the modification times of two files.
///
/// Returns `Ordering::Less` if path1 was modified before path2,
/// `Ordering::Greater` if path1 was modified after path2,
/// and `Ordering::Equal` if they have the same modification time.
///
/// # Arguments
///
/// * `path1` - The first file path
/// * `path2` - The second file path
///
/// # Returns
///
/// A `Result` containing the `Ordering`, or an `io::Error`
///
/// # Examples
///
/// ```no_run
/// use file_metadata::compare_modified_times;
/// use std::path::Path;
/// use std::cmp::Ordering;
///
/// let ordering = compare_modified_times(
///     Path::new("file1.txt"),
///     Path::new("file2.txt")
/// ).unwrap();
///
/// match ordering {
///     Ordering::Less => println!("file1 is older"),
///     Ordering::Greater => println!("file1 is newer"),
///     Ordering::Equal => println!("Same modification time"),
/// }
/// ```
pub fn compare_modified_times(path1: &Path, path2: &Path) -> io::Result<Ordering> {
    let time1 = get_modified_time(path1)?;
    let time2 = get_modified_time(path2)?;

    // SystemTime implements PartialOrd, but we need to handle the comparison
    // carefully since it returns Option<Ordering>
    Ok(time1.cmp(&time2))
}

pub fn main() {
    use std::fs::File;
    use std::io::Write;

    // Create test files
    let test_dir = Path::new("metadata_test");
    let _ = fs::create_dir_all(test_dir);

    let file_path = test_dir.join("test_file.txt");
    let _ = File::create(&file_path).and_then(|mut f| f.write_all(b"Hello, World!"));

    println!("=== File Metadata Examples ===\n");

    // Get file size
    if let Ok(size) = get_file_size(&file_path) {
        println!("File size: {} bytes", size);
    }

    // Get file type
    if let Ok(ft) = get_file_type(&file_path) {
        println!("File type: {}", ft);
    }

    if let Ok(ft) = get_file_type(test_dir) {
        println!("Directory type: {}", ft);
    }

    // Check if read-only
    if let Ok(readonly) = is_readonly(&file_path) {
        println!("Is read-only: {}", readonly);
    }

    // Get modification time
    if let Ok(modified) = get_modified_time(&file_path) {
        println!("Modified: {:?}", modified);
    }

    // Check if recently modified
    if let Ok(recent) = was_modified_within(&file_path, Duration::from_secs(60)) {
        println!("Modified within last minute: {}", recent);
    }

    // Check if executable
    if let Ok(exec) = is_executable(&file_path) {
        println!("Is executable: {}", exec);
    }

    // Create another file for comparison
    let file_path2 = test_dir.join("test_file2.txt");
    let _ = File::create(&file_path2).and_then(|mut f| f.write_all(b"Second file"));

    // Compare modification times
    if let Ok(ordering) = compare_modified_times(&file_path, &file_path2) {
        match ordering {
            Ordering::Less => println!("First file is older"),
            Ordering::Greater => println!("First file is newer"),
            Ordering::Equal => println!("Same modification time"),
        }
    }

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
    println!("\nTest directory cleaned up.");
}
