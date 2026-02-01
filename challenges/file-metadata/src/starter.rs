use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Returns the size of a file in bytes.
///
/// Use `std::fs::metadata()` to get file information.
pub fn get_file_size(path: &Path) -> io::Result<u64> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns the type of a filesystem entry as a string.
///
/// Returns "file", "directory", or "symlink".
/// Use `std::fs::symlink_metadata()` to detect symlinks properly.
pub fn get_file_type(path: &Path) -> io::Result<String> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file is read-only.
///
/// Use the permissions() method on Metadata.
pub fn is_readonly(path: &Path) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Returns when a file was last modified.
///
/// Returns the modification time as a SystemTime.
pub fn get_modified_time(path: &Path) -> io::Result<SystemTime> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file was modified within a given duration from now.
///
/// Compare the modification time with the current time.
pub fn was_modified_within(path: &Path, duration: Duration) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Checks if a file has execute permissions.
///
/// On Unix, check the execute bits using PermissionsExt.
/// On Windows, check for executable extensions (.exe, .bat, .cmd, .com).
pub fn is_executable(path: &Path) -> io::Result<bool> {
    // TODO: Implement this function
    unimplemented!()
}

/// Compares the modification times of two files.
///
/// Returns Ordering::Less if path1 was modified before path2,
/// Ordering::Greater if path1 was modified after path2,
/// Ordering::Equal if they have the same modification time.
pub fn compare_modified_times(path1: &Path, path2: &Path) -> io::Result<Ordering> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    use std::fs::File;
    use std::io::Write;

    // Create a test file
    let test_dir = Path::new("metadata_test");
    let _ = fs::create_dir_all(test_dir);

    let file_path = test_dir.join("test_file.txt");
    let _ = File::create(&file_path).and_then(|mut f| f.write_all(b"Hello, World!"));

    println!("=== File Metadata Examples ===\n");

    // Test your implementations here
    match get_file_size(&file_path) {
        Ok(size) => println!("File size: {} bytes", size),
        Err(e) => println!("Error getting size: {}", e),
    }

    match get_file_type(&file_path) {
        Ok(ft) => println!("File type: {}", ft),
        Err(e) => println!("Error getting type: {}", e),
    }

    match is_readonly(&file_path) {
        Ok(ro) => println!("Is read-only: {}", ro),
        Err(e) => println!("Error checking readonly: {}", e),
    }

    match get_modified_time(&file_path) {
        Ok(time) => println!("Modified time: {:?}", time),
        Err(e) => println!("Error getting modified time: {}", e),
    }

    match was_modified_within(&file_path, Duration::from_secs(60)) {
        Ok(recent) => println!("Modified within last minute: {}", recent),
        Err(e) => println!("Error checking recency: {}", e),
    }

    match is_executable(&file_path) {
        Ok(exec) => println!("Is executable: {}", exec),
        Err(e) => println!("Error checking executable: {}", e),
    }

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
    println!("\nTest directory cleaned up.");
}
