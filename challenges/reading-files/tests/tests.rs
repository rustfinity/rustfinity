use reading_files::*;
use std::fs::{remove_file, write};
use std::io;

// Helper to create a test file and ensure cleanup
struct TestFile {
    path: String,
}

impl TestFile {
    fn new(name: &str, content: &str) -> Self {
        let path = format!("test_{}.txt", name);
        write(&path, content).expect("Failed to create test file");
        TestFile { path }
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.path);
    }
}

// ==================== read_entire_file tests ====================

#[test]
fn test_read_entire_file_simple() {
    let file = TestFile::new("read_simple", "Hello, World!");
    let result = read_entire_file(&file.path).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_read_entire_file_multiline() {
    let file = TestFile::new("read_multiline", "Line 1\nLine 2\nLine 3\n");
    let result = read_entire_file(&file.path).unwrap();
    assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
}

#[test]
fn test_read_entire_file_empty() {
    let file = TestFile::new("read_empty", "");
    let result = read_entire_file(&file.path).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_read_entire_file_unicode() {
    let file = TestFile::new("read_unicode", "Hello, \u{4E16}\u{754C}! \u{1F600}");
    let result = read_entire_file(&file.path).unwrap();
    assert_eq!(result, "Hello, \u{4E16}\u{754C}! \u{1F600}");
}

#[test]
fn test_read_entire_file_not_found() {
    let result = read_entire_file("nonexistent_file_12345.txt");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
}

// ==================== count_lines tests ====================

#[test]
fn test_count_lines_simple() {
    let file = TestFile::new("lines_simple", "Line 1\nLine 2\nLine 3\n");
    let result = count_lines(&file.path).unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_count_lines_no_trailing_newline() {
    let file = TestFile::new("lines_no_trailing", "Line 1\nLine 2\nLine 3");
    let result = count_lines(&file.path).unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_count_lines_empty() {
    let file = TestFile::new("lines_empty", "");
    let result = count_lines(&file.path).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_count_lines_single_line() {
    let file = TestFile::new("lines_single", "Single line");
    let result = count_lines(&file.path).unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_count_lines_blank_lines() {
    let file = TestFile::new("lines_blank", "Line 1\n\nLine 3\n\n");
    let result = count_lines(&file.path).unwrap();
    assert_eq!(result, 4);
}

#[test]
fn test_count_lines_not_found() {
    let result = count_lines("nonexistent_file_12345.txt");
    assert!(result.is_err());
}

// ==================== count_words tests ====================

#[test]
fn test_count_words_simple() {
    let file = TestFile::new("words_simple", "Hello World");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 2);
}

#[test]
fn test_count_words_multiline() {
    let file = TestFile::new("words_multiline", "Hello World\nThis is a test\n");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 6);
}

#[test]
fn test_count_words_empty() {
    let file = TestFile::new("words_empty", "");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_count_words_whitespace_only() {
    let file = TestFile::new("words_whitespace", "   \n\t\n   ");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_count_words_extra_whitespace() {
    let file = TestFile::new("words_extra", "  Hello   World  \n  Test  ");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_count_words_single_word() {
    let file = TestFile::new("words_single", "Hello");
    let result = count_words(&file.path).unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_count_words_not_found() {
    let result = count_words("nonexistent_file_12345.txt");
    assert!(result.is_err());
}

// ==================== read_lines tests ====================

#[test]
fn test_read_lines_simple() {
    let file = TestFile::new("readlines_simple", "Line 1\nLine 2\nLine 3\n");
    let result = read_lines(&file.path).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_read_lines_no_trailing_newline() {
    let file = TestFile::new("readlines_no_trailing", "Line 1\nLine 2\nLine 3");
    let result = read_lines(&file.path).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_read_lines_empty() {
    let file = TestFile::new("readlines_empty", "");
    let result = read_lines(&file.path).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_read_lines_single_line() {
    let file = TestFile::new("readlines_single", "Single line");
    let result = read_lines(&file.path).unwrap();
    assert_eq!(result, vec!["Single line"]);
}

#[test]
fn test_read_lines_preserves_whitespace() {
    let file = TestFile::new("readlines_whitespace", "  Indented\n\tTabbed\n");
    let result = read_lines(&file.path).unwrap();
    assert_eq!(result, vec!["  Indented", "\tTabbed"]);
}

#[test]
fn test_read_lines_blank_lines() {
    let file = TestFile::new("readlines_blank", "Line 1\n\nLine 3\n");
    let result = read_lines(&file.path).unwrap();
    assert_eq!(result, vec!["Line 1", "", "Line 3"]);
}

#[test]
fn test_read_lines_not_found() {
    let result = read_lines("nonexistent_file_12345.txt");
    assert!(result.is_err());
}

// ==================== first_n_lines tests ====================

#[test]
fn test_first_n_lines_simple() {
    let file = TestFile::new("firstn_simple", "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n");
    let result = first_n_lines(&file.path, 3).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_first_n_lines_more_than_file() {
    let file = TestFile::new("firstn_more", "Line 1\nLine 2\n");
    let result = first_n_lines(&file.path, 10).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2"]);
}

#[test]
fn test_first_n_lines_zero() {
    let file = TestFile::new("firstn_zero", "Line 1\nLine 2\nLine 3\n");
    let result = first_n_lines(&file.path, 0).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_first_n_lines_one() {
    let file = TestFile::new("firstn_one", "First line\nSecond line\n");
    let result = first_n_lines(&file.path, 1).unwrap();
    assert_eq!(result, vec!["First line"]);
}

#[test]
fn test_first_n_lines_empty_file() {
    let file = TestFile::new("firstn_empty", "");
    let result = first_n_lines(&file.path, 5).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_first_n_lines_exact_count() {
    let file = TestFile::new("firstn_exact", "Line 1\nLine 2\nLine 3\n");
    let result = first_n_lines(&file.path, 3).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_first_n_lines_not_found() {
    let result = first_n_lines("nonexistent_file_12345.txt", 5);
    assert!(result.is_err());
}

// ==================== Integration tests ====================

#[test]
fn test_integration_read_and_count() {
    let content = "The quick brown fox\njumps over the lazy dog\n";
    let file = TestFile::new("integration", content);

    // All functions should work on the same file
    assert_eq!(read_entire_file(&file.path).unwrap(), content);
    assert_eq!(count_lines(&file.path).unwrap(), 2);
    assert_eq!(count_words(&file.path).unwrap(), 9);
    assert_eq!(read_lines(&file.path).unwrap(), vec!["The quick brown fox", "jumps over the lazy dog"]);
    assert_eq!(first_n_lines(&file.path, 1).unwrap(), vec!["The quick brown fox"]);
}

#[test]
fn test_integration_large_content() {
    // Create content with many lines
    let lines: Vec<String> = (1..=100).map(|i| format!("Line number {}", i)).collect();
    let content = lines.join("\n") + "\n";
    let file = TestFile::new("integration_large", &content);

    assert_eq!(count_lines(&file.path).unwrap(), 100);
    assert_eq!(read_lines(&file.path).unwrap().len(), 100);
    assert_eq!(first_n_lines(&file.path, 10).unwrap().len(), 10);
}
