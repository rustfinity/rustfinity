use directory_traversal::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

// Helper to create a unique test directory
fn create_test_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(format!("test_dir_{}", name));
    let _ = fs::remove_dir_all(&dir); // Clean up any existing
    fs::create_dir_all(&dir).unwrap();
    dir
}

// Helper to clean up test directory
fn cleanup_test_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
}

// ============================================
// Tests for list_files
// ============================================

#[test]
fn test_list_files_basic() {
    let dir = create_test_dir("list_files_basic");
    File::create(dir.join("file1.txt")).unwrap();
    File::create(dir.join("file2.txt")).unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();

    let files = list_files(&dir).unwrap();
    assert_eq!(files.len(), 2);
    assert!(files.iter().any(|p| p.file_name().unwrap() == "file1.txt"));
    assert!(files.iter().any(|p| p.file_name().unwrap() == "file2.txt"));

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_files_empty_directory() {
    let dir = create_test_dir("list_files_empty");

    let files = list_files(&dir).unwrap();
    assert!(files.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_files_only_directories() {
    let dir = create_test_dir("list_files_only_dirs");
    fs::create_dir(dir.join("subdir1")).unwrap();
    fs::create_dir(dir.join("subdir2")).unwrap();

    let files = list_files(&dir).unwrap();
    assert!(files.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_files_non_recursive() {
    let dir = create_test_dir("list_files_nonrec");
    File::create(dir.join("top.txt")).unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    File::create(dir.join("subdir").join("nested.txt")).unwrap();

    let files = list_files(&dir).unwrap();
    assert_eq!(files.len(), 1);
    assert!(files[0].file_name().unwrap() == "top.txt");

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_files_nonexistent_directory() {
    let result = list_files(Path::new("nonexistent_dir_xyz"));
    assert!(result.is_err());
}

// ============================================
// Tests for list_directories
// ============================================

#[test]
fn test_list_directories_basic() {
    let dir = create_test_dir("list_dirs_basic");
    fs::create_dir(dir.join("subdir1")).unwrap();
    fs::create_dir(dir.join("subdir2")).unwrap();
    File::create(dir.join("file.txt")).unwrap();

    let dirs = list_directories(&dir).unwrap();
    assert_eq!(dirs.len(), 2);
    assert!(dirs.iter().any(|p| p.file_name().unwrap() == "subdir1"));
    assert!(dirs.iter().any(|p| p.file_name().unwrap() == "subdir2"));

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_directories_empty() {
    let dir = create_test_dir("list_dirs_empty");

    let dirs = list_directories(&dir).unwrap();
    assert!(dirs.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_directories_only_files() {
    let dir = create_test_dir("list_dirs_only_files");
    File::create(dir.join("file1.txt")).unwrap();
    File::create(dir.join("file2.txt")).unwrap();

    let dirs = list_directories(&dir).unwrap();
    assert!(dirs.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_directories_non_recursive() {
    let dir = create_test_dir("list_dirs_nonrec");
    fs::create_dir(dir.join("level1")).unwrap();
    fs::create_dir(dir.join("level1").join("level2")).unwrap();

    let dirs = list_directories(&dir).unwrap();
    assert_eq!(dirs.len(), 1);
    assert!(dirs[0].file_name().unwrap() == "level1");

    cleanup_test_dir(&dir);
}

// ============================================
// Tests for list_all_recursive
// ============================================

#[test]
fn test_list_all_recursive_basic() {
    let dir = create_test_dir("list_all_basic");
    File::create(dir.join("file1.txt")).unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    File::create(dir.join("subdir").join("file2.txt")).unwrap();

    let all = list_all_recursive(&dir).unwrap();
    assert_eq!(all.len(), 3); // file1.txt, subdir, subdir/file2.txt

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_all_recursive_deep_nesting() {
    let dir = create_test_dir("list_all_deep");
    fs::create_dir_all(dir.join("a").join("b").join("c")).unwrap();
    File::create(dir.join("a").join("b").join("c").join("deep.txt")).unwrap();
    File::create(dir.join("a").join("top.txt")).unwrap();

    let all = list_all_recursive(&dir).unwrap();
    // a, a/b, a/b/c, a/b/c/deep.txt, a/top.txt = 5 items
    assert_eq!(all.len(), 5);

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_all_recursive_empty() {
    let dir = create_test_dir("list_all_empty");

    let all = list_all_recursive(&dir).unwrap();
    assert!(all.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_list_all_recursive_files_only() {
    let dir = create_test_dir("list_all_files_only");
    File::create(dir.join("a.txt")).unwrap();
    File::create(dir.join("b.txt")).unwrap();
    File::create(dir.join("c.txt")).unwrap();

    let all = list_all_recursive(&dir).unwrap();
    assert_eq!(all.len(), 3);

    cleanup_test_dir(&dir);
}

// ============================================
// Tests for find_by_extension
// ============================================

#[test]
fn test_find_by_extension_basic() {
    let dir = create_test_dir("find_ext_basic");
    File::create(dir.join("file1.rs")).unwrap();
    File::create(dir.join("file2.rs")).unwrap();
    File::create(dir.join("file3.txt")).unwrap();

    let rs_files = find_by_extension(&dir, "rs").unwrap();
    assert_eq!(rs_files.len(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_extension_recursive() {
    let dir = create_test_dir("find_ext_recursive");
    File::create(dir.join("top.rs")).unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    File::create(dir.join("subdir").join("nested.rs")).unwrap();
    File::create(dir.join("subdir").join("other.txt")).unwrap();

    let rs_files = find_by_extension(&dir, "rs").unwrap();
    assert_eq!(rs_files.len(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_extension_no_matches() {
    let dir = create_test_dir("find_ext_no_match");
    File::create(dir.join("file1.txt")).unwrap();
    File::create(dir.join("file2.txt")).unwrap();

    let rs_files = find_by_extension(&dir, "rs").unwrap();
    assert!(rs_files.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_extension_empty_dir() {
    let dir = create_test_dir("find_ext_empty");

    let files = find_by_extension(&dir, "txt").unwrap();
    assert!(files.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_extension_multiple_dots() {
    let dir = create_test_dir("find_ext_multi_dots");
    File::create(dir.join("archive.tar.gz")).unwrap();
    File::create(dir.join("file.gz")).unwrap();

    let gz_files = find_by_extension(&dir, "gz").unwrap();
    assert_eq!(gz_files.len(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_extension_no_extension() {
    let dir = create_test_dir("find_ext_no_ext");
    File::create(dir.join("Makefile")).unwrap();
    File::create(dir.join("README")).unwrap();
    File::create(dir.join("file.txt")).unwrap();

    let txt_files = find_by_extension(&dir, "txt").unwrap();
    assert_eq!(txt_files.len(), 1);

    cleanup_test_dir(&dir);
}

// ============================================
// Tests for find_by_name
// ============================================

#[test]
fn test_find_by_name_basic() {
    let dir = create_test_dir("find_name_basic");
    File::create(dir.join("target.txt")).unwrap();
    File::create(dir.join("other.txt")).unwrap();

    let found = find_by_name(&dir, "target.txt").unwrap();
    assert_eq!(found.len(), 1);
    assert!(found[0].file_name().unwrap() == "target.txt");

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_name_recursive() {
    let dir = create_test_dir("find_name_recursive");
    File::create(dir.join("Cargo.toml")).unwrap();
    fs::create_dir(dir.join("subproject")).unwrap();
    File::create(dir.join("subproject").join("Cargo.toml")).unwrap();

    let found = find_by_name(&dir, "Cargo.toml").unwrap();
    assert_eq!(found.len(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_name_directory() {
    let dir = create_test_dir("find_name_dir");
    fs::create_dir(dir.join("src")).unwrap();
    fs::create_dir(dir.join("tests")).unwrap();

    let found = find_by_name(&dir, "src").unwrap();
    assert_eq!(found.len(), 1);
    assert!(found[0].is_dir());

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_name_no_matches() {
    let dir = create_test_dir("find_name_no_match");
    File::create(dir.join("file.txt")).unwrap();

    let found = find_by_name(&dir, "nonexistent").unwrap();
    assert!(found.is_empty());

    cleanup_test_dir(&dir);
}

#[test]
fn test_find_by_name_case_sensitive() {
    let dir = create_test_dir("find_name_case");
    File::create(dir.join("File.txt")).unwrap();
    File::create(dir.join("file.txt")).unwrap();

    let found = find_by_name(&dir, "File.txt").unwrap();
    assert_eq!(found.len(), 1);

    cleanup_test_dir(&dir);
}

// ============================================
// Tests for calculate_dir_size
// ============================================

#[test]
fn test_calculate_dir_size_basic() {
    let dir = create_test_dir("calc_size_basic");
    let mut f1 = File::create(dir.join("file1.txt")).unwrap();
    f1.write_all(b"Hello").unwrap(); // 5 bytes
    let mut f2 = File::create(dir.join("file2.txt")).unwrap();
    f2.write_all(b"World!").unwrap(); // 6 bytes

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 11);

    cleanup_test_dir(&dir);
}

#[test]
fn test_calculate_dir_size_recursive() {
    let dir = create_test_dir("calc_size_recursive");
    let mut f1 = File::create(dir.join("top.txt")).unwrap();
    f1.write_all(b"12345").unwrap(); // 5 bytes
    fs::create_dir(dir.join("subdir")).unwrap();
    let mut f2 = File::create(dir.join("subdir").join("nested.txt")).unwrap();
    f2.write_all(b"67890").unwrap(); // 5 bytes

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 10);

    cleanup_test_dir(&dir);
}

#[test]
fn test_calculate_dir_size_empty() {
    let dir = create_test_dir("calc_size_empty");

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_calculate_dir_size_empty_files() {
    let dir = create_test_dir("calc_size_empty_files");
    File::create(dir.join("empty1.txt")).unwrap();
    File::create(dir.join("empty2.txt")).unwrap();

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_calculate_dir_size_only_directories() {
    let dir = create_test_dir("calc_size_only_dirs");
    fs::create_dir(dir.join("subdir1")).unwrap();
    fs::create_dir(dir.join("subdir2")).unwrap();

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_calculate_dir_size_deep_nesting() {
    let dir = create_test_dir("calc_size_deep");
    fs::create_dir_all(dir.join("a").join("b").join("c")).unwrap();
    let mut f = File::create(dir.join("a").join("b").join("c").join("deep.txt")).unwrap();
    f.write_all(b"deep content").unwrap(); // 12 bytes

    let size = calculate_dir_size(&dir).unwrap();
    assert_eq!(size, 12);

    cleanup_test_dir(&dir);
}

// ============================================
// Tests for count_files_recursive
// ============================================

#[test]
fn test_count_files_recursive_basic() {
    let dir = create_test_dir("count_files_basic");
    File::create(dir.join("file1.txt")).unwrap();
    File::create(dir.join("file2.txt")).unwrap();
    File::create(dir.join("file3.txt")).unwrap();

    let count = count_files_recursive(&dir).unwrap();
    assert_eq!(count, 3);

    cleanup_test_dir(&dir);
}

#[test]
fn test_count_files_recursive_with_subdirs() {
    let dir = create_test_dir("count_files_subdirs");
    File::create(dir.join("top.txt")).unwrap();
    fs::create_dir(dir.join("subdir")).unwrap();
    File::create(dir.join("subdir").join("nested1.txt")).unwrap();
    File::create(dir.join("subdir").join("nested2.txt")).unwrap();

    let count = count_files_recursive(&dir).unwrap();
    assert_eq!(count, 3);

    cleanup_test_dir(&dir);
}

#[test]
fn test_count_files_recursive_empty() {
    let dir = create_test_dir("count_files_empty");

    let count = count_files_recursive(&dir).unwrap();
    assert_eq!(count, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_count_files_recursive_only_directories() {
    let dir = create_test_dir("count_files_only_dirs");
    fs::create_dir(dir.join("subdir1")).unwrap();
    fs::create_dir(dir.join("subdir2")).unwrap();
    fs::create_dir(dir.join("subdir1").join("nested")).unwrap();

    let count = count_files_recursive(&dir).unwrap();
    assert_eq!(count, 0);

    cleanup_test_dir(&dir);
}

#[test]
fn test_count_files_recursive_deep() {
    let dir = create_test_dir("count_files_deep");
    fs::create_dir_all(dir.join("a").join("b").join("c")).unwrap();
    File::create(dir.join("a").join("x.txt")).unwrap();
    File::create(dir.join("a").join("b").join("y.txt")).unwrap();
    File::create(dir.join("a").join("b").join("c").join("z.txt")).unwrap();

    let count = count_files_recursive(&dir).unwrap();
    assert_eq!(count, 3);

    cleanup_test_dir(&dir);
}

// ============================================
// Integration tests
// ============================================

#[test]
fn test_integration_complex_structure() {
    let dir = create_test_dir("integration_complex");

    // Create a complex directory structure
    fs::create_dir_all(dir.join("src").join("utils")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::create_dir(dir.join("docs")).unwrap();

    // Create files
    let mut f = File::create(dir.join("Cargo.toml")).unwrap();
    f.write_all(b"[package]").unwrap();
    let mut f = File::create(dir.join("README.md")).unwrap();
    f.write_all(b"# Project").unwrap();
    let mut f = File::create(dir.join("src").join("main.rs")).unwrap();
    f.write_all(b"fn main() {}").unwrap();
    let mut f = File::create(dir.join("src").join("lib.rs")).unwrap();
    f.write_all(b"pub mod utils;").unwrap();
    let mut f = File::create(dir.join("src").join("utils").join("mod.rs")).unwrap();
    f.write_all(b"pub fn helper() {}").unwrap();
    let mut f = File::create(dir.join("tests").join("test.rs")).unwrap();
    f.write_all(b"#[test]").unwrap();

    // Test list_files (top level only)
    let top_files = list_files(&dir).unwrap();
    assert_eq!(top_files.len(), 2); // Cargo.toml, README.md

    // Test list_directories
    let dirs = list_directories(&dir).unwrap();
    assert_eq!(dirs.len(), 3); // src, tests, docs

    // Test find_by_extension
    let rs_files = find_by_extension(&dir, "rs").unwrap();
    assert_eq!(rs_files.len(), 4); // main.rs, lib.rs, mod.rs, test.rs

    // Test find_by_name
    let cargo_files = find_by_name(&dir, "Cargo.toml").unwrap();
    assert_eq!(cargo_files.len(), 1);

    // Test count_files_recursive
    let file_count = count_files_recursive(&dir).unwrap();
    assert_eq!(file_count, 6);

    // Test calculate_dir_size
    let size = calculate_dir_size(&dir).unwrap();
    // [package] + # Project + fn main() {} + pub mod utils; + pub fn helper() {} + #[test]
    // 9 + 9 + 12 + 14 + 18 + 7 = 69 bytes
    assert_eq!(size, 69);

    cleanup_test_dir(&dir);
}

#[test]
fn test_integration_find_all_matches() {
    let dir = create_test_dir("integration_find_all");

    // Create structure with multiple matches
    fs::create_dir_all(dir.join("project1").join("src")).unwrap();
    fs::create_dir_all(dir.join("project2").join("src")).unwrap();

    File::create(dir.join("project1").join("Cargo.toml")).unwrap();
    File::create(dir.join("project1").join("src").join("lib.rs")).unwrap();
    File::create(dir.join("project2").join("Cargo.toml")).unwrap();
    File::create(dir.join("project2").join("src").join("lib.rs")).unwrap();

    // Find all Cargo.toml files
    let cargo_files = find_by_name(&dir, "Cargo.toml").unwrap();
    assert_eq!(cargo_files.len(), 2);

    // Find all lib.rs files
    let lib_files = find_by_name(&dir, "lib.rs").unwrap();
    assert_eq!(lib_files.len(), 2);

    // Find all src directories
    let src_dirs = find_by_name(&dir, "src").unwrap();
    assert_eq!(src_dirs.len(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_integration_size_vs_count() {
    let dir = create_test_dir("integration_size_count");

    // Create files with known sizes
    let mut f = File::create(dir.join("small.txt")).unwrap();
    f.write_all(b"abc").unwrap(); // 3 bytes

    fs::create_dir(dir.join("subdir")).unwrap();
    let mut f = File::create(dir.join("subdir").join("medium.txt")).unwrap();
    f.write_all(b"12345").unwrap(); // 5 bytes

    let mut f = File::create(dir.join("subdir").join("large.txt")).unwrap();
    f.write_all(b"0123456789").unwrap(); // 10 bytes

    let count = count_files_recursive(&dir).unwrap();
    let size = calculate_dir_size(&dir).unwrap();

    assert_eq!(count, 3);
    assert_eq!(size, 18);

    cleanup_test_dir(&dir);
}
