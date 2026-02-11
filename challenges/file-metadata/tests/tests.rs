use file_metadata::*;
use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

// Helper to create a temporary test directory
fn setup_test_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("file_metadata_test_{}", name));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

// Helper to cleanup test directory
fn cleanup_test_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

// ==================== get_file_size tests ====================

#[test]
fn test_get_file_size_basic() {
    let dir = setup_test_dir("size_basic");
    let file_path = dir.join("test.txt");
    let content = "Hello, World!";
    File::create(&file_path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();

    let size = get_file_size(&file_path).unwrap();
    assert_eq!(size, content.len() as u64);

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_file_size_empty_file() {
    let dir = setup_test_dir("size_empty");
    let file_path = dir.join("empty.txt");
    File::create(&file_path).unwrap();

    let size = get_file_size(&file_path).unwrap();
    assert_eq!(size, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_file_size_larger_file() {
    let dir = setup_test_dir("size_large");
    let file_path = dir.join("large.txt");
    let content = "x".repeat(10000);
    File::create(&file_path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();

    let size = get_file_size(&file_path).unwrap();
    assert_eq!(size, 10000);

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_file_size_nonexistent() {
    let result = get_file_size(Path::new("/nonexistent/path/file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_get_file_size_binary_content() {
    let dir = setup_test_dir("size_binary");
    let file_path = dir.join("binary.bin");
    let content: Vec<u8> = (0..=255).collect();
    File::create(&file_path).unwrap().write_all(&content).unwrap();

    let size = get_file_size(&file_path).unwrap();
    assert_eq!(size, 256);

    cleanup_test_dir(&dir);
}

// ==================== get_file_type tests ====================

#[test]
fn test_get_file_type_regular_file() {
    let dir = setup_test_dir("type_file");
    let file_path = dir.join("regular.txt");
    File::create(&file_path).unwrap();

    let file_type = get_file_type(&file_path).unwrap();
    assert_eq!(file_type, "file");

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_file_type_directory() {
    let dir = setup_test_dir("type_dir");
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    let file_type = get_file_type(&subdir).unwrap();
    assert_eq!(file_type, "directory");

    cleanup_test_dir(&dir);
}

#[test]
#[cfg(unix)]
fn test_get_file_type_symlink() {
    use std::os::unix::fs::symlink;

    let dir = setup_test_dir("type_symlink");
    let file_path = dir.join("original.txt");
    let link_path = dir.join("link.txt");
    File::create(&file_path).unwrap();
    symlink(&file_path, &link_path).unwrap();

    let file_type = get_file_type(&link_path).unwrap();
    assert_eq!(file_type, "symlink");

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_file_type_nonexistent() {
    let result = get_file_type(Path::new("/nonexistent/path"));
    assert!(result.is_err());
}

#[test]
fn test_get_file_type_nested_directory() {
    let dir = setup_test_dir("type_nested");
    let nested = dir.join("a").join("b").join("c");
    fs::create_dir_all(&nested).unwrap();

    let file_type = get_file_type(&nested).unwrap();
    assert_eq!(file_type, "directory");

    cleanup_test_dir(&dir);
}

// ==================== is_readonly tests ====================

#[test]
fn test_is_readonly_writable_file() {
    let dir = setup_test_dir("readonly_writable");
    let file_path = dir.join("writable.txt");
    File::create(&file_path).unwrap();

    let readonly = is_readonly(&file_path).unwrap();
    assert!(!readonly);

    cleanup_test_dir(&dir);
}

#[test]
fn test_is_readonly_readonly_file() {
    let dir = setup_test_dir("readonly_readonly");
    let file_path = dir.join("readonly.txt");
    File::create(&file_path).unwrap();

    // Set file to read-only
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&file_path, perms).unwrap();

    let readonly = is_readonly(&file_path).unwrap();
    assert!(readonly);

    // Restore write permissions for cleanup
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(&file_path, perms).unwrap();

    cleanup_test_dir(&dir);
}

#[test]
fn test_is_readonly_nonexistent() {
    let result = is_readonly(Path::new("/nonexistent/path/file.txt"));
    assert!(result.is_err());
}

// ==================== get_modified_time tests ====================

#[test]
fn test_get_modified_time_exists() {
    let dir = setup_test_dir("modified_exists");
    let file_path = dir.join("test.txt");
    File::create(&file_path).unwrap();

    let result = get_modified_time(&file_path);
    assert!(result.is_ok());

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_modified_time_recent() {
    let dir = setup_test_dir("modified_recent");
    let file_path = dir.join("recent.txt");
    File::create(&file_path).unwrap();

    let modified = get_modified_time(&file_path).unwrap();
    let now = std::time::SystemTime::now();
    let elapsed = now.duration_since(modified).unwrap();

    // File should have been modified within the last few seconds
    assert!(elapsed.as_secs() < 10);

    cleanup_test_dir(&dir);
}

#[test]
fn test_get_modified_time_nonexistent() {
    let result = get_modified_time(Path::new("/nonexistent/path/file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_get_modified_time_after_write() {
    let dir = setup_test_dir("modified_after_write");
    let file_path = dir.join("test.txt");
    File::create(&file_path).unwrap();

    let time1 = get_modified_time(&file_path).unwrap();

    // Small delay to ensure different timestamp
    thread::sleep(Duration::from_millis(100));

    // Modify the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(&file_path)
        .unwrap();
    file.write_all(b"new content").unwrap();
    drop(file);

    let time2 = get_modified_time(&file_path).unwrap();

    // time2 should be >= time1 (could be equal on low-resolution filesystems)
    assert!(time2 >= time1);

    cleanup_test_dir(&dir);
}

// ==================== was_modified_within tests ====================

#[test]
fn test_was_modified_within_recent() {
    let dir = setup_test_dir("within_recent");
    let file_path = dir.join("recent.txt");
    File::create(&file_path).unwrap();

    // File was just created, should be within the last hour
    let result = was_modified_within(&file_path, Duration::from_secs(3600)).unwrap();
    assert!(result);

    cleanup_test_dir(&dir);
}

#[test]
fn test_was_modified_within_very_short_duration() {
    let dir = setup_test_dir("within_short");
    let file_path = dir.join("test.txt");
    File::create(&file_path).unwrap();

    // Wait a bit
    thread::sleep(Duration::from_millis(200));

    // Check with a very short duration (less than the wait)
    let result = was_modified_within(&file_path, Duration::from_millis(50)).unwrap();
    assert!(!result);

    cleanup_test_dir(&dir);
}

#[test]
fn test_was_modified_within_exact_timing() {
    let dir = setup_test_dir("within_exact");
    let file_path = dir.join("test.txt");
    File::create(&file_path).unwrap();

    // File was just created, should be within the last minute
    let result = was_modified_within(&file_path, Duration::from_secs(60)).unwrap();
    assert!(result);

    cleanup_test_dir(&dir);
}

#[test]
fn test_was_modified_within_nonexistent() {
    let result = was_modified_within(Path::new("/nonexistent/file.txt"), Duration::from_secs(60));
    assert!(result.is_err());
}

// ==================== is_executable tests ====================

#[test]
#[cfg(unix)]
fn test_is_executable_regular_file() {
    let dir = setup_test_dir("exec_regular");
    let file_path = dir.join("regular.txt");
    File::create(&file_path).unwrap();

    // Regular files typically aren't executable by default
    let result = is_executable(&file_path).unwrap();
    assert!(!result);

    cleanup_test_dir(&dir);
}

#[test]
#[cfg(unix)]
fn test_is_executable_with_execute_permission() {
    use std::os::unix::fs::PermissionsExt;

    let dir = setup_test_dir("exec_permission");
    let file_path = dir.join("script.sh");
    File::create(&file_path)
        .unwrap()
        .write_all(b"#!/bin/bash\necho hello")
        .unwrap();

    // Set execute permission
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = is_executable(&file_path).unwrap();
    assert!(result);

    cleanup_test_dir(&dir);
}

#[test]
#[cfg(unix)]
fn test_is_executable_owner_only() {
    use std::os::unix::fs::PermissionsExt;

    let dir = setup_test_dir("exec_owner");
    let file_path = dir.join("script.sh");
    File::create(&file_path).unwrap();

    // Set execute permission for owner only
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = is_executable(&file_path).unwrap();
    assert!(result);

    cleanup_test_dir(&dir);
}

#[test]
#[cfg(unix)]
fn test_is_executable_directory() {
    let dir = setup_test_dir("exec_dir");
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    // Directories aren't "executable" in the program sense
    let result = is_executable(&subdir).unwrap();
    assert!(!result);

    cleanup_test_dir(&dir);
}

#[test]
fn test_is_executable_nonexistent() {
    let result = is_executable(Path::new("/nonexistent/path/script.sh"));
    assert!(result.is_err());
}

// ==================== compare_modified_times tests ====================

#[test]
fn test_compare_modified_times_same_time() {
    let dir = setup_test_dir("compare_same");
    let file_path = dir.join("test.txt");
    File::create(&file_path).unwrap();

    // Compare file with itself
    let result = compare_modified_times(&file_path, &file_path).unwrap();
    assert_eq!(result, Ordering::Equal);

    cleanup_test_dir(&dir);
}

#[test]
fn test_compare_modified_times_different_files() {
    let dir = setup_test_dir("compare_different");
    let file1 = dir.join("first.txt");
    File::create(&file1).unwrap();

    // Small delay to ensure different timestamp
    thread::sleep(Duration::from_millis(100));

    let file2 = dir.join("second.txt");
    File::create(&file2).unwrap();

    let result = compare_modified_times(&file1, &file2).unwrap();
    // file1 was created first, so it should be Less (older)
    assert_eq!(result, Ordering::Less);

    // Reverse comparison
    let result_reverse = compare_modified_times(&file2, &file1).unwrap();
    assert_eq!(result_reverse, Ordering::Greater);

    cleanup_test_dir(&dir);
}

#[test]
fn test_compare_modified_times_after_modification() {
    let dir = setup_test_dir("compare_modified");
    let file1 = dir.join("file1.txt");
    let file2 = dir.join("file2.txt");
    File::create(&file1).unwrap();
    File::create(&file2).unwrap();

    thread::sleep(Duration::from_millis(100));

    // Modify file1 to make it newer
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(&file1)
        .unwrap();
    file.write_all(b"modified").unwrap();
    drop(file);

    let result = compare_modified_times(&file1, &file2).unwrap();
    // file1 was modified more recently
    assert_eq!(result, Ordering::Greater);

    cleanup_test_dir(&dir);
}

#[test]
fn test_compare_modified_times_first_nonexistent() {
    let dir = setup_test_dir("compare_noexist1");
    let file_path = dir.join("exists.txt");
    File::create(&file_path).unwrap();

    let result = compare_modified_times(Path::new("/nonexistent"), &file_path);
    assert!(result.is_err());

    cleanup_test_dir(&dir);
}

#[test]
fn test_compare_modified_times_second_nonexistent() {
    let dir = setup_test_dir("compare_noexist2");
    let file_path = dir.join("exists.txt");
    File::create(&file_path).unwrap();

    let result = compare_modified_times(&file_path, Path::new("/nonexistent"));
    assert!(result.is_err());

    cleanup_test_dir(&dir);
}

// ==================== Integration tests ====================

#[test]
fn test_integration_file_lifecycle() {
    let dir = setup_test_dir("integration_lifecycle");
    let file_path = dir.join("lifecycle.txt");

    // Create file
    let content = "Initial content";
    File::create(&file_path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();

    // Check initial state
    assert_eq!(get_file_size(&file_path).unwrap(), content.len() as u64);
    assert_eq!(get_file_type(&file_path).unwrap(), "file");
    assert!(!is_readonly(&file_path).unwrap());
    assert!(was_modified_within(&file_path, Duration::from_secs(60)).unwrap());

    // Modify file
    thread::sleep(Duration::from_millis(100));
    let new_content = "Modified content with more data";
    fs::write(&file_path, new_content).unwrap();

    // Check after modification
    assert_eq!(get_file_size(&file_path).unwrap(), new_content.len() as u64);
    assert!(was_modified_within(&file_path, Duration::from_secs(60)).unwrap());

    cleanup_test_dir(&dir);
}

#[test]
fn test_integration_directory_structure() {
    let dir = setup_test_dir("integration_dir_structure");

    // Create files and directories
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();

    let file1 = dir.join("file1.txt");
    let file2 = subdir.join("file2.txt");

    File::create(&file1).unwrap().write_all(b"content1").unwrap();
    thread::sleep(Duration::from_millis(100));
    File::create(&file2).unwrap().write_all(b"content2").unwrap();

    // Check types
    assert_eq!(get_file_type(&dir).unwrap(), "directory");
    assert_eq!(get_file_type(&subdir).unwrap(), "directory");
    assert_eq!(get_file_type(&file1).unwrap(), "file");
    assert_eq!(get_file_type(&file2).unwrap(), "file");

    // Compare times
    let ordering = compare_modified_times(&file1, &file2).unwrap();
    assert_eq!(ordering, Ordering::Less);

    cleanup_test_dir(&dir);
}

#[test]
fn test_integration_multiple_file_comparison() {
    let dir = setup_test_dir("integration_multi");

    let files: Vec<_> = (0..5)
        .map(|i| {
            let path = dir.join(format!("file{}.txt", i));
            File::create(&path).unwrap();
            thread::sleep(Duration::from_millis(50));
            path
        })
        .collect();

    // Files should be in chronological order
    for i in 0..files.len() - 1 {
        let ordering = compare_modified_times(&files[i], &files[i + 1]).unwrap();
        assert_eq!(ordering, Ordering::Less, "File {} should be older than file {}", i, i + 1);
    }

    cleanup_test_dir(&dir);
}

#[test]
#[cfg(unix)]
fn test_integration_permissions_workflow() {
    use std::os::unix::fs::PermissionsExt;

    let dir = setup_test_dir("integration_perms");
    let file_path = dir.join("script.sh");

    // Create non-executable file
    File::create(&file_path)
        .unwrap()
        .write_all(b"#!/bin/bash\necho hello")
        .unwrap();

    assert!(!is_executable(&file_path).unwrap());
    assert!(!is_readonly(&file_path).unwrap());

    // Make executable
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&file_path, perms).unwrap();

    assert!(is_executable(&file_path).unwrap());

    // Make read-only
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&file_path, perms).unwrap();

    assert!(is_readonly(&file_path).unwrap());

    // Restore for cleanup
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(&file_path, perms).unwrap();

    cleanup_test_dir(&dir);
}
