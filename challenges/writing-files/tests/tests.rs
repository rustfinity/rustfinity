use std::fs::{self, remove_file};
use writing_files::*;

// Helper to create a test file and ensure cleanup
struct TestFile {
    path: String,
}

impl TestFile {
    fn new(name: &str) -> Self {
        let path = format!("test_write_{}.txt", name);
        // Clean up any existing file first
        let _ = remove_file(&path);
        TestFile { path }
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn read_contents(&self) -> String {
        fs::read_to_string(&self.path).unwrap_or_default()
    }

    fn read_bytes(&self) -> Vec<u8> {
        fs::read(&self.path).unwrap_or_default()
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

// ==================== write_string tests ====================

#[test]
fn test_write_string_simple() {
    let file = TestFile::new("string_simple");
    write_string(file.path(), "Hello, World!").unwrap();
    assert_eq!(file.read_contents(), "Hello, World!");
}

#[test]
fn test_write_string_multiline() {
    let file = TestFile::new("string_multiline");
    write_string(file.path(), "Line 1\nLine 2\nLine 3\n").unwrap();
    assert_eq!(file.read_contents(), "Line 1\nLine 2\nLine 3\n");
}

#[test]
fn test_write_string_empty() {
    let file = TestFile::new("string_empty");
    write_string(file.path(), "").unwrap();
    assert_eq!(file.read_contents(), "");
}

#[test]
fn test_write_string_unicode() {
    let file = TestFile::new("string_unicode");
    write_string(file.path(), "Hello, \u{4E16}\u{754C}! \u{1F600}").unwrap();
    assert_eq!(file.read_contents(), "Hello, \u{4E16}\u{754C}! \u{1F600}");
}

#[test]
fn test_write_string_overwrites() {
    let file = TestFile::new("string_overwrite");
    write_string(file.path(), "Original content").unwrap();
    write_string(file.path(), "New content").unwrap();
    assert_eq!(file.read_contents(), "New content");
}

#[test]
fn test_write_string_with_special_chars() {
    let file = TestFile::new("string_special");
    write_string(file.path(), "Tab:\tNewline:\nCarriage:\r").unwrap();
    assert_eq!(file.read_contents(), "Tab:\tNewline:\nCarriage:\r");
}

// ==================== write_bytes tests ====================

#[test]
fn test_write_bytes_simple() {
    let file = TestFile::new("bytes_simple");
    write_bytes(file.path(), &[72, 101, 108, 108, 111]).unwrap();
    assert_eq!(file.read_bytes(), vec![72, 101, 108, 108, 111]);
    assert_eq!(file.read_contents(), "Hello");
}

#[test]
fn test_write_bytes_empty() {
    let file = TestFile::new("bytes_empty");
    write_bytes(file.path(), &[]).unwrap();
    assert_eq!(file.read_bytes(), Vec::<u8>::new());
}

#[test]
fn test_write_bytes_binary() {
    let file = TestFile::new("bytes_binary");
    let binary_data: Vec<u8> = (0..=255).collect();
    write_bytes(file.path(), &binary_data).unwrap();
    assert_eq!(file.read_bytes(), binary_data);
}

#[test]
fn test_write_bytes_overwrites() {
    let file = TestFile::new("bytes_overwrite");
    write_bytes(file.path(), &[1, 2, 3, 4, 5]).unwrap();
    write_bytes(file.path(), &[6, 7, 8]).unwrap();
    assert_eq!(file.read_bytes(), vec![6, 7, 8]);
}

#[test]
fn test_write_bytes_null_bytes() {
    let file = TestFile::new("bytes_null");
    write_bytes(file.path(), &[0, 1, 0, 2, 0]).unwrap();
    assert_eq!(file.read_bytes(), vec![0, 1, 0, 2, 0]);
}

// ==================== append_string tests ====================

#[test]
fn test_append_string_to_new_file() {
    let file = TestFile::new("append_new");
    append_string(file.path(), "First content").unwrap();
    assert_eq!(file.read_contents(), "First content");
}

#[test]
fn test_append_string_to_existing() {
    let file = TestFile::new("append_existing");
    write_string(file.path(), "Original\n").unwrap();
    append_string(file.path(), "Appended\n").unwrap();
    assert_eq!(file.read_contents(), "Original\nAppended\n");
}

#[test]
fn test_append_string_multiple_times() {
    let file = TestFile::new("append_multiple");
    append_string(file.path(), "Line 1\n").unwrap();
    append_string(file.path(), "Line 2\n").unwrap();
    append_string(file.path(), "Line 3\n").unwrap();
    assert_eq!(file.read_contents(), "Line 1\nLine 2\nLine 3\n");
}

#[test]
fn test_append_string_empty() {
    let file = TestFile::new("append_empty");
    write_string(file.path(), "Original").unwrap();
    append_string(file.path(), "").unwrap();
    assert_eq!(file.read_contents(), "Original");
}

#[test]
fn test_append_string_unicode() {
    let file = TestFile::new("append_unicode");
    append_string(file.path(), "\u{4E16}").unwrap();
    append_string(file.path(), "\u{754C}").unwrap();
    assert_eq!(file.read_contents(), "\u{4E16}\u{754C}");
}

#[test]
fn test_append_preserves_content() {
    let file = TestFile::new("append_preserve");
    write_string(file.path(), "AAAAAAAAAA").unwrap();
    append_string(file.path(), "B").unwrap();
    assert_eq!(file.read_contents(), "AAAAAAAAAAB");
}

// ==================== write_lines tests ====================

#[test]
fn test_write_lines_simple() {
    let file = TestFile::new("lines_simple");
    write_lines(file.path(), &["Apple", "Banana", "Cherry"]).unwrap();
    assert_eq!(file.read_contents(), "Apple\nBanana\nCherry\n");
}

#[test]
fn test_write_lines_empty_vec() {
    let file = TestFile::new("lines_empty_vec");
    write_lines(file.path(), &[]).unwrap();
    assert_eq!(file.read_contents(), "");
}

#[test]
fn test_write_lines_single_line() {
    let file = TestFile::new("lines_single");
    write_lines(file.path(), &["Only one line"]).unwrap();
    assert_eq!(file.read_contents(), "Only one line\n");
}

#[test]
fn test_write_lines_with_empty_strings() {
    let file = TestFile::new("lines_empty_strings");
    write_lines(file.path(), &["First", "", "Third"]).unwrap();
    assert_eq!(file.read_contents(), "First\n\nThird\n");
}

#[test]
fn test_write_lines_unicode() {
    let file = TestFile::new("lines_unicode");
    write_lines(file.path(), &["\u{4E16}\u{754C}", "\u{1F600}\u{1F600}"]).unwrap();
    assert_eq!(file.read_contents(), "\u{4E16}\u{754C}\n\u{1F600}\u{1F600}\n");
}

#[test]
fn test_write_lines_overwrites() {
    let file = TestFile::new("lines_overwrite");
    write_lines(file.path(), &["Original", "Content"]).unwrap();
    write_lines(file.path(), &["New"]).unwrap();
    assert_eq!(file.read_contents(), "New\n");
}

#[test]
fn test_write_lines_many_lines() {
    let file = TestFile::new("lines_many");
    let lines: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    write_lines(file.path(), &line_refs).unwrap();

    let content = file.read_contents();
    let written_lines: Vec<&str> = content.lines().collect();
    assert_eq!(written_lines.len(), 100);
    assert_eq!(written_lines[0], "Line 1");
    assert_eq!(written_lines[99], "Line 100");
}

// ==================== write_with_buffer tests ====================

#[test]
fn test_write_with_buffer_simple() {
    let file = TestFile::new("buffer_simple");
    write_with_buffer(file.path(), &["Hello, ", "World", "!"]).unwrap();
    assert_eq!(file.read_contents(), "Hello, World!");
}

#[test]
fn test_write_with_buffer_empty() {
    let file = TestFile::new("buffer_empty");
    write_with_buffer(file.path(), &[]).unwrap();
    assert_eq!(file.read_contents(), "");
}

#[test]
fn test_write_with_buffer_single_chunk() {
    let file = TestFile::new("buffer_single");
    write_with_buffer(file.path(), &["Single chunk"]).unwrap();
    assert_eq!(file.read_contents(), "Single chunk");
}

#[test]
fn test_write_with_buffer_many_chunks() {
    let file = TestFile::new("buffer_many");
    let chunks: Vec<&str> = vec!["a"; 1000];
    write_with_buffer(file.path(), &chunks).unwrap();
    assert_eq!(file.read_contents().len(), 1000);
    assert!(file.read_contents().chars().all(|c| c == 'a'));
}

#[test]
fn test_write_with_buffer_with_newlines() {
    let file = TestFile::new("buffer_newlines");
    write_with_buffer(file.path(), &["Line 1\n", "Line 2\n", "Line 3\n"]).unwrap();
    assert_eq!(file.read_contents(), "Line 1\nLine 2\nLine 3\n");
}

#[test]
fn test_write_with_buffer_unicode() {
    let file = TestFile::new("buffer_unicode");
    write_with_buffer(file.path(), &["\u{4E16}", "\u{754C}"]).unwrap();
    assert_eq!(file.read_contents(), "\u{4E16}\u{754C}");
}

#[test]
fn test_write_with_buffer_empty_chunks() {
    let file = TestFile::new("buffer_empty_chunks");
    write_with_buffer(file.path(), &["A", "", "B", "", "C"]).unwrap();
    assert_eq!(file.read_contents(), "ABC");
}

#[test]
fn test_write_with_buffer_overwrites() {
    let file = TestFile::new("buffer_overwrite");
    write_with_buffer(file.path(), &["Original"]).unwrap();
    write_with_buffer(file.path(), &["New"]).unwrap();
    assert_eq!(file.read_contents(), "New");
}

// ==================== Integration tests ====================

#[test]
fn test_integration_write_and_append() {
    let file = TestFile::new("integration_write_append");

    // Write initial content
    write_string(file.path(), "Header\n").unwrap();
    assert_eq!(file.read_contents(), "Header\n");

    // Append more content
    append_string(file.path(), "Entry 1\n").unwrap();
    append_string(file.path(), "Entry 2\n").unwrap();
    assert_eq!(file.read_contents(), "Header\nEntry 1\nEntry 2\n");

    // Overwrite completely
    write_string(file.path(), "Fresh start\n").unwrap();
    assert_eq!(file.read_contents(), "Fresh start\n");
}

#[test]
fn test_integration_string_and_bytes() {
    let file = TestFile::new("integration_string_bytes");

    // Write as string
    write_string(file.path(), "Hello").unwrap();
    assert_eq!(file.read_bytes(), vec![72, 101, 108, 108, 111]);

    // Overwrite as bytes
    write_bytes(file.path(), &[87, 111, 114, 108, 100]).unwrap();
    assert_eq!(file.read_contents(), "World");
}

#[test]
fn test_integration_lines_and_buffer() {
    let file = TestFile::new("integration_lines_buffer");

    // Write with write_lines
    write_lines(file.path(), &["Line 1", "Line 2"]).unwrap();
    assert_eq!(file.read_contents(), "Line 1\nLine 2\n");

    // Overwrite with buffered writing
    write_with_buffer(file.path(), &["Buffered ", "content"]).unwrap();
    assert_eq!(file.read_contents(), "Buffered content");
}

#[test]
fn test_integration_log_file_pattern() {
    let file = TestFile::new("integration_log");

    // Simulate a log file pattern
    write_string(file.path(), "=== Log Start ===\n").unwrap();
    append_string(file.path(), "[INFO] Application started\n").unwrap();
    append_string(file.path(), "[DEBUG] Loading config\n").unwrap();
    append_string(file.path(), "[INFO] Ready\n").unwrap();

    let content = file.read_contents();
    assert!(content.starts_with("=== Log Start ===\n"));
    assert!(content.contains("[INFO] Application started"));
    assert!(content.contains("[DEBUG] Loading config"));
    assert!(content.ends_with("[INFO] Ready\n"));
}

#[test]
fn test_integration_large_buffered_write() {
    let file = TestFile::new("integration_large");

    // Create many small chunks
    let chunks: Vec<String> = (0..10000).map(|i| format!("{:04}", i)).collect();
    let chunk_refs: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();

    write_with_buffer(file.path(), &chunk_refs).unwrap();

    let content = file.read_contents();
    assert_eq!(content.len(), 40000); // 10000 * 4 chars each
    assert!(content.starts_with("0000"));
    assert!(content.ends_with("9999"));
}

// ==================== Error handling tests ====================

#[test]
fn test_write_to_invalid_path() {
    let result = write_string("/nonexistent/directory/file.txt", "test");
    assert!(result.is_err());
}

#[test]
fn test_append_to_invalid_path() {
    let result = append_string("/nonexistent/directory/file.txt", "test");
    assert!(result.is_err());
}

#[test]
fn test_write_lines_to_invalid_path() {
    let result = write_lines("/nonexistent/directory/file.txt", &["test"]);
    assert!(result.is_err());
}

#[test]
fn test_write_bytes_to_invalid_path() {
    let result = write_bytes("/nonexistent/directory/file.txt", &[1, 2, 3]);
    assert!(result.is_err());
}

#[test]
fn test_write_with_buffer_to_invalid_path() {
    let result = write_with_buffer("/nonexistent/directory/file.txt", &["test"]);
    assert!(result.is_err());
}
