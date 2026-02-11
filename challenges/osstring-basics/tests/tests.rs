use osstring_basics::*;
use std::ffi::{OsStr, OsString};
use std::path::Path;

// ============================================================================
// to_os_string tests
// ============================================================================

#[test]
fn test_to_os_string_simple() {
    let os = to_os_string("hello.txt");
    assert_eq!(os, OsString::from("hello.txt"));
}

#[test]
fn test_to_os_string_empty() {
    let os = to_os_string("");
    assert_eq!(os, OsString::from(""));
}

#[test]
fn test_to_os_string_with_spaces() {
    let os = to_os_string("hello world.txt");
    assert_eq!(os, OsString::from("hello world.txt"));
}

#[test]
fn test_to_os_string_with_path_separators() {
    let os = to_os_string("path/to/file.txt");
    assert_eq!(os, OsString::from("path/to/file.txt"));
}

#[test]
fn test_to_os_string_unicode() {
    let os = to_os_string("文件.txt");
    assert_eq!(os, OsString::from("文件.txt"));
}

#[test]
fn test_to_os_string_special_chars() {
    let os = to_os_string("file-name_123.txt");
    assert_eq!(os, OsString::from("file-name_123.txt"));
}

// ============================================================================
// os_str_to_str tests
// ============================================================================

#[test]
fn test_os_str_to_str_valid() {
    let os_str = OsStr::new("hello.txt");
    assert_eq!(os_str_to_str(os_str), Some("hello.txt"));
}

#[test]
fn test_os_str_to_str_empty() {
    let os_str = OsStr::new("");
    assert_eq!(os_str_to_str(os_str), Some(""));
}

#[test]
fn test_os_str_to_str_unicode() {
    let os_str = OsStr::new("café.txt");
    assert_eq!(os_str_to_str(os_str), Some("café.txt"));
}

#[test]
fn test_os_str_to_str_chinese() {
    let os_str = OsStr::new("日本語.txt");
    assert_eq!(os_str_to_str(os_str), Some("日本語.txt"));
}

#[test]
fn test_os_str_to_str_emoji() {
    let os_str = OsStr::new("🎉🎊.txt");
    assert_eq!(os_str_to_str(os_str), Some("🎉🎊.txt"));
}

#[test]
fn test_os_str_to_str_spaces_and_special() {
    let os_str = OsStr::new("my file (1).txt");
    assert_eq!(os_str_to_str(os_str), Some("my file (1).txt"));
}

// ============================================================================
// os_string_to_string_lossy tests
// ============================================================================

#[test]
fn test_os_string_to_string_lossy_valid() {
    let os_str = OsStr::new("valid utf8");
    assert_eq!(os_string_to_string_lossy(os_str), "valid utf8");
}

#[test]
fn test_os_string_to_string_lossy_empty() {
    let os_str = OsStr::new("");
    assert_eq!(os_string_to_string_lossy(os_str), "");
}

#[test]
fn test_os_string_to_string_lossy_unicode() {
    let os_str = OsStr::new("こんにちは世界");
    assert_eq!(os_string_to_string_lossy(os_str), "こんにちは世界");
}

#[test]
fn test_os_string_to_string_lossy_mixed() {
    let os_str = OsStr::new("Hello, 世界! 🌍");
    assert_eq!(os_string_to_string_lossy(os_str), "Hello, 世界! 🌍");
}

#[test]
fn test_os_string_to_string_lossy_path_like() {
    let os_str = OsStr::new("/home/user/documents/file.txt");
    assert_eq!(
        os_string_to_string_lossy(os_str),
        "/home/user/documents/file.txt"
    );
}

// ============================================================================
// get_file_extension tests
// ============================================================================

#[test]
fn test_get_file_extension_simple() {
    let path = Path::new("document.pdf");
    assert_eq!(get_file_extension(path), Some("pdf".to_string()));
}

#[test]
fn test_get_file_extension_txt() {
    let path = Path::new("readme.txt");
    assert_eq!(get_file_extension(path), Some("txt".to_string()));
}

#[test]
fn test_get_file_extension_rust() {
    let path = Path::new("main.rs");
    assert_eq!(get_file_extension(path), Some("rs".to_string()));
}

#[test]
fn test_get_file_extension_double() {
    let path = Path::new("archive.tar.gz");
    assert_eq!(get_file_extension(path), Some("gz".to_string()));
}

#[test]
fn test_get_file_extension_none() {
    let path = Path::new("README");
    assert_eq!(get_file_extension(path), None);
}

#[test]
fn test_get_file_extension_hidden_file() {
    let path = Path::new(".gitignore");
    assert_eq!(get_file_extension(path), None);
}

#[test]
fn test_get_file_extension_hidden_with_ext() {
    let path = Path::new(".config.json");
    assert_eq!(get_file_extension(path), Some("json".to_string()));
}

#[test]
fn test_get_file_extension_with_path() {
    let path = Path::new("/home/user/document.pdf");
    assert_eq!(get_file_extension(path), Some("pdf".to_string()));
}

#[test]
fn test_get_file_extension_ends_with_dot() {
    let path = Path::new("file.");
    assert_eq!(get_file_extension(path), Some("".to_string()));
}

#[test]
fn test_get_file_extension_multiple_dots() {
    let path = Path::new("file.name.with.dots.txt");
    assert_eq!(get_file_extension(path), Some("txt".to_string()));
}

#[test]
fn test_get_file_extension_unicode() {
    let path = Path::new("文件.txt");
    assert_eq!(get_file_extension(path), Some("txt".to_string()));
}

#[test]
fn test_get_file_extension_empty_path() {
    let path = Path::new("");
    assert_eq!(get_file_extension(path), None);
}

// ============================================================================
// join_path_components tests
// ============================================================================

#[test]
fn test_join_path_components_simple() {
    let path = join_path_components(&["home", "user"]);
    let expected = Path::new("home").join("user");
    assert_eq!(path, expected.into_os_string());
}

#[test]
fn test_join_path_components_three() {
    let path = join_path_components(&["home", "user", "documents"]);
    let expected = Path::new("home").join("user").join("documents");
    assert_eq!(path, expected.into_os_string());
}

#[test]
fn test_join_path_components_with_file() {
    let path = join_path_components(&["home", "user", "file.txt"]);
    let expected = Path::new("home").join("user").join("file.txt");
    assert_eq!(path, expected.into_os_string());
}

#[test]
fn test_join_path_components_single() {
    let path = join_path_components(&["home"]);
    assert_eq!(path, OsString::from("home"));
}

#[test]
fn test_join_path_components_empty() {
    let path = join_path_components(&[]);
    assert_eq!(path, OsString::from(""));
}

#[test]
fn test_join_path_components_with_spaces() {
    let path = join_path_components(&["My Documents", "Work Files"]);
    let expected = Path::new("My Documents").join("Work Files");
    assert_eq!(path, expected.into_os_string());
}

#[test]
fn test_join_path_components_unicode() {
    let path = join_path_components(&["用户", "文档"]);
    let expected = Path::new("用户").join("文档");
    assert_eq!(path, expected.into_os_string());
}

#[test]
fn test_join_path_components_many() {
    let path = join_path_components(&["a", "b", "c", "d", "e"]);
    let expected = Path::new("a").join("b").join("c").join("d").join("e");
    assert_eq!(path, expected.into_os_string());
}

// ============================================================================
// is_valid_utf8 tests
// ============================================================================

#[test]
fn test_is_valid_utf8_ascii() {
    let os_str = OsStr::new("hello");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_empty() {
    let os_str = OsStr::new("");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_unicode() {
    let os_str = OsStr::new("café");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_chinese() {
    let os_str = OsStr::new("你好世界");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_emoji() {
    let os_str = OsStr::new("🦀🎉");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_mixed() {
    let os_str = OsStr::new("Hello 世界 🌍 café");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_path_like() {
    let os_str = OsStr::new("/home/user/documents/日本語.txt");
    assert!(is_valid_utf8(os_str));
}

#[test]
fn test_is_valid_utf8_special_chars() {
    let os_str = OsStr::new("file-name_123 (copy).txt");
    assert!(is_valid_utf8(os_str));
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_roundtrip_conversion() {
    let original = "test_file.txt";
    let os_string = to_os_string(original);
    let back = os_str_to_str(&os_string);
    assert_eq!(back, Some(original));
}

#[test]
fn test_path_workflow() {
    // Simulate a common workflow: join components, get extension
    let components = &["home", "user", "document.pdf"];
    let path_os = join_path_components(components);
    let path = Path::new(&path_os);
    let ext = get_file_extension(path);
    assert_eq!(ext, Some("pdf".to_string()));
}

#[test]
fn test_lossy_vs_strict() {
    // For valid UTF-8, both methods should give equivalent results
    let os_str = OsStr::new("valid string");
    let strict = os_str_to_str(os_str);
    let lossy = os_string_to_string_lossy(os_str);
    assert_eq!(strict, Some("valid string"));
    assert_eq!(lossy, "valid string");
}

#[test]
fn test_unicode_path_workflow() {
    let components = &["用户", "文档", "报告.pdf"];
    let path_os = join_path_components(components);
    assert!(is_valid_utf8(&path_os));
    let path = Path::new(&path_os);
    assert_eq!(get_file_extension(path), Some("pdf".to_string()));
}
