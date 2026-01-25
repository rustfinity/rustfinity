use std::fs;
use tempfile_operations::*;

// ==================== get_temp_dir tests ====================

#[test]
fn test_get_temp_dir_exists() {
    let temp_dir = get_temp_dir();
    assert!(temp_dir.exists(), "Temp directory should exist");
}

#[test]
fn test_get_temp_dir_is_directory() {
    let temp_dir = get_temp_dir();
    assert!(temp_dir.is_dir(), "Temp directory should be a directory");
}

// ==================== create_temp_file tests ====================

#[test]
fn test_create_temp_file_basic() {
    let path = create_temp_file("test_basic", ".tmp").unwrap();
    assert!(path.exists(), "Created file should exist");
    assert!(path.is_file(), "Should be a file, not a directory");
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_contains_prefix() {
    let path = create_temp_file("uniqueprefix", ".tmp").unwrap();
    let filename = path.file_name().unwrap().to_string_lossy();
    assert!(filename.contains("uniqueprefix"), "Filename should contain prefix");
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_contains_suffix() {
    let path = create_temp_file("test", ".myext").unwrap();
    let filename = path.file_name().unwrap().to_string_lossy();
    assert!(filename.ends_with(".myext"), "Filename should end with suffix");
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_unique_names() {
    let path1 = create_temp_file("unique", ".tmp").unwrap();
    let path2 = create_temp_file("unique", ".tmp").unwrap();
    assert_ne!(path1, path2, "Each call should create a unique filename");
    fs::remove_file(&path1).unwrap();
    fs::remove_file(&path2).unwrap();
}

#[test]
fn test_create_temp_file_in_temp_dir() {
    let temp_dir = get_temp_dir();
    let path = create_temp_file("location", ".tmp").unwrap();
    assert!(
        path.starts_with(&temp_dir),
        "File should be in temp directory"
    );
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_empty_prefix() {
    let path = create_temp_file("", ".tmp").unwrap();
    assert!(path.exists());
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_empty_suffix() {
    let path = create_temp_file("nosuffix", "").unwrap();
    assert!(path.exists());
    fs::remove_file(&path).unwrap();
}

// ==================== create_temp_file_with_content tests ====================

#[test]
fn test_create_temp_file_with_content_basic() {
    let content = "Hello, World!";
    let path = create_temp_file_with_content("content", ".txt", content).unwrap();
    assert!(path.exists());
    let read_content = fs::read_to_string(&path).unwrap();
    assert_eq!(read_content, content);
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_with_content_empty() {
    let path = create_temp_file_with_content("empty", ".txt", "").unwrap();
    assert!(path.exists());
    let read_content = fs::read_to_string(&path).unwrap();
    assert_eq!(read_content, "");
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_with_content_multiline() {
    let content = "Line 1\nLine 2\nLine 3";
    let path = create_temp_file_with_content("multiline", ".txt", content).unwrap();
    let read_content = fs::read_to_string(&path).unwrap();
    assert_eq!(read_content, content);
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_with_content_unicode() {
    let content = "Hello, 世界! 🌍 Привет!";
    let path = create_temp_file_with_content("unicode", ".txt", content).unwrap();
    let read_content = fs::read_to_string(&path).unwrap();
    assert_eq!(read_content, content);
    fs::remove_file(&path).unwrap();
}

#[test]
fn test_create_temp_file_with_content_large() {
    let content = "x".repeat(10_000);
    let path = create_temp_file_with_content("large", ".txt", &content).unwrap();
    let read_content = fs::read_to_string(&path).unwrap();
    assert_eq!(read_content.len(), 10_000);
    fs::remove_file(&path).unwrap();
}

// ==================== TempFile tests ====================

#[test]
fn test_tempfile_new_creates_file() {
    let temp = TempFile::new("tfnew", ".tmp").unwrap();
    assert!(temp.path().exists());
}

#[test]
fn test_tempfile_path_contains_prefix() {
    let temp = TempFile::new("mypfx", ".tmp").unwrap();
    let filename = temp.path().file_name().unwrap().to_string_lossy();
    assert!(filename.contains("mypfx"));
}

#[test]
fn test_tempfile_write_and_read() {
    let temp = TempFile::new("rw", ".tmp").unwrap();
    temp.write("Test content").unwrap();
    let content = temp.read().unwrap();
    assert_eq!(content, "Test content");
}

#[test]
fn test_tempfile_write_overwrites() {
    let temp = TempFile::new("overwrite", ".tmp").unwrap();
    temp.write("First").unwrap();
    temp.write("Second").unwrap();
    let content = temp.read().unwrap();
    assert_eq!(content, "Second");
}

#[test]
fn test_tempfile_read_empty() {
    let temp = TempFile::new("empty", ".tmp").unwrap();
    let content = temp.read().unwrap();
    assert_eq!(content, "");
}

#[test]
fn test_tempfile_drop_deletes_file() {
    let path: std::path::PathBuf;
    {
        let temp = TempFile::new("droptest", ".tmp").unwrap();
        path = temp.path().to_path_buf();
        assert!(path.exists(), "File should exist before drop");
    }
    assert!(!path.exists(), "File should be deleted after drop");
}

#[test]
fn test_tempfile_unicode_content() {
    let temp = TempFile::new("tfunicode", ".tmp").unwrap();
    let unicode_content = "こんにちは世界 🎉";
    temp.write(unicode_content).unwrap();
    assert_eq!(temp.read().unwrap(), unicode_content);
}

#[test]
fn test_tempfile_multiline_content() {
    let temp = TempFile::new("tfmulti", ".tmp").unwrap();
    let content = "Line 1\nLine 2\nLine 3\n";
    temp.write(content).unwrap();
    assert_eq!(temp.read().unwrap(), content);
}

// ==================== create_temp_dir tests ====================

#[test]
fn test_create_temp_dir_basic() {
    let dir = create_temp_dir("testdir").unwrap();
    assert!(dir.exists());
    assert!(dir.is_dir());
    fs::remove_dir(&dir).unwrap();
}

#[test]
fn test_create_temp_dir_contains_prefix() {
    let dir = create_temp_dir("dirprefix").unwrap();
    let dirname = dir.file_name().unwrap().to_string_lossy();
    assert!(dirname.contains("dirprefix"));
    fs::remove_dir(&dir).unwrap();
}

#[test]
fn test_create_temp_dir_unique_names() {
    let dir1 = create_temp_dir("unique").unwrap();
    let dir2 = create_temp_dir("unique").unwrap();
    assert_ne!(dir1, dir2);
    fs::remove_dir(&dir1).unwrap();
    fs::remove_dir(&dir2).unwrap();
}

#[test]
fn test_create_temp_dir_in_temp_dir() {
    let temp_dir = get_temp_dir();
    let dir = create_temp_dir("location").unwrap();
    assert!(dir.starts_with(&temp_dir));
    fs::remove_dir(&dir).unwrap();
}

#[test]
fn test_create_temp_dir_can_create_files_inside() {
    let dir = create_temp_dir("withfiles").unwrap();
    let file_path = dir.join("test.txt");
    fs::write(&file_path, "content").unwrap();
    assert!(file_path.exists());
    fs::remove_file(&file_path).unwrap();
    fs::remove_dir(&dir).unwrap();
}

// ==================== cleanup_temp_files tests ====================

#[test]
fn test_cleanup_temp_files_basic() {
    let prefix = format!("cleanup_basic_{}", std::process::id());

    // Create some test files
    let path1 = create_temp_file_with_content(&prefix, ".tmp", "test1").unwrap();
    let path2 = create_temp_file_with_content(&prefix, ".tmp", "test2").unwrap();

    assert!(path1.exists());
    assert!(path2.exists());

    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
    assert!(deleted >= 2);

    assert!(!path1.exists());
    assert!(!path2.exists());
}

#[test]
fn test_cleanup_temp_files_only_matching_prefix() {
    let prefix1 = format!("cleanup_match_{}", std::process::id());
    let prefix2 = format!("cleanup_other_{}", std::process::id());

    let path1 = create_temp_file_with_content(&prefix1, ".tmp", "match").unwrap();
    let path2 = create_temp_file_with_content(&prefix2, ".tmp", "other").unwrap();

    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix1).unwrap();
    assert!(deleted >= 1);

    assert!(!path1.exists(), "Matching file should be deleted");
    assert!(path2.exists(), "Non-matching file should remain");

    fs::remove_file(&path2).unwrap();
}

#[test]
fn test_cleanup_temp_files_no_matches() {
    let prefix = format!("nonexistent_prefix_{}", std::process::id());
    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
    assert_eq!(deleted, 0);
}

#[test]
fn test_cleanup_temp_files_returns_count() {
    let prefix = format!("cleanup_count_{}", std::process::id());

    create_temp_file_with_content(&prefix, ".tmp", "1").unwrap();
    create_temp_file_with_content(&prefix, ".tmp", "2").unwrap();
    create_temp_file_with_content(&prefix, ".tmp", "3").unwrap();

    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
    assert!(deleted >= 3);
}

#[test]
fn test_cleanup_temp_files_ignores_directories() {
    let prefix = format!("cleanup_dir_{}", std::process::id());

    // Create a directory with the matching prefix
    let dir = create_temp_dir(&prefix).unwrap();
    // Create a file with the matching prefix
    let file = create_temp_file_with_content(&prefix, ".tmp", "test").unwrap();

    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
    assert!(deleted >= 1, "Should delete at least the file");

    // File should be deleted, directory should remain
    assert!(!file.exists());
    assert!(dir.exists(), "Directory should not be deleted by cleanup_temp_files");

    fs::remove_dir(&dir).unwrap();
}

// ==================== Integration tests ====================

#[test]
fn test_integration_temp_workflow() {
    // Simulate a typical temp file workflow
    let prefix = format!("workflow_{}", std::process::id());

    // Step 1: Create a temp directory for work
    let work_dir = create_temp_dir(&prefix).unwrap();

    // Step 2: Create some temp files in the work directory
    let file1_path = work_dir.join("data1.tmp");
    let file2_path = work_dir.join("data2.tmp");
    fs::write(&file1_path, "Data 1").unwrap();
    fs::write(&file2_path, "Data 2").unwrap();

    // Step 3: Read and verify
    assert_eq!(fs::read_to_string(&file1_path).unwrap(), "Data 1");
    assert_eq!(fs::read_to_string(&file2_path).unwrap(), "Data 2");

    // Step 4: Cleanup
    fs::remove_file(&file1_path).unwrap();
    fs::remove_file(&file2_path).unwrap();
    fs::remove_dir(&work_dir).unwrap();
}

#[test]
fn test_integration_tempfile_raii() {
    // Test RAII pattern with TempFile
    let paths: Vec<std::path::PathBuf>;

    {
        let mut temps = Vec::new();
        for i in 0..3 {
            let temp = TempFile::new("raii", ".tmp").unwrap();
            temp.write(&format!("Content {}", i)).unwrap();
            temps.push(temp);
        }

        // All TempFiles are in scope, files should exist
        paths = temps.iter().map(|t| t.path().to_path_buf()).collect();
        for path in &paths {
            assert!(path.exists(), "File should exist while TempFile is in scope");
        }
        // temps goes out of scope here, triggering Drop
    }

    // TempFiles dropped, all files should be deleted
    for path in &paths {
        assert!(!path.exists(), "File should be deleted after TempFile is dropped");
    }
}

#[test]
fn test_integration_processing_pipeline() {
    // Simulate a data processing pipeline using temp files
    let temp = TempFile::new("pipeline", ".txt").unwrap();

    // Step 1: Write initial data
    temp.write("hello world").unwrap();

    // Step 2: Read, transform, write back
    let content = temp.read().unwrap();
    let transformed = content.to_uppercase();
    temp.write(&transformed).unwrap();

    // Step 3: Verify transformation
    assert_eq!(temp.read().unwrap(), "HELLO WORLD");
}

#[test]
fn test_integration_batch_file_creation() {
    let prefix = format!("batch_{}", std::process::id());
    let mut paths = Vec::new();

    // Create multiple files
    for i in 0..5 {
        let path = create_temp_file_with_content(&prefix, ".txt", &format!("File {}", i)).unwrap();
        paths.push(path);
    }

    // Verify all exist
    for path in &paths {
        assert!(path.exists());
    }

    // Cleanup all at once
    let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
    assert!(deleted >= 5);

    // Verify all deleted
    for path in &paths {
        assert!(!path.exists());
    }
}

#[test]
fn test_integration_nested_temp_dirs() {
    let prefix = format!("nested_{}", std::process::id());

    // Create outer temp dir
    let outer = create_temp_dir(&prefix).unwrap();

    // Create nested structure
    let inner = outer.join("inner");
    fs::create_dir(&inner).unwrap();

    let deep_file = inner.join("deep.txt");
    fs::write(&deep_file, "Deep content").unwrap();

    // Verify structure
    assert!(outer.is_dir());
    assert!(inner.is_dir());
    assert!(deep_file.is_file());

    // Cleanup (must be done in order)
    fs::remove_file(&deep_file).unwrap();
    fs::remove_dir(&inner).unwrap();
    fs::remove_dir(&outer).unwrap();
}

#[test]
fn test_integration_concurrent_temp_files() {
    // Create many temp files to test uniqueness
    let prefix = format!("concurrent_{}", std::process::id());
    let mut paths = Vec::new();

    for _ in 0..20 {
        let path = create_temp_file(&prefix, ".tmp").unwrap();
        paths.push(path);
    }

    // Verify all paths are unique
    let unique_count = paths.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(unique_count, 20);

    // Cleanup
    cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
}
