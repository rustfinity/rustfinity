use path_operations::*;
use std::path::PathBuf;

// ============== join_paths tests ==============

#[test]
fn test_join_paths_single_part() {
    let result = join_paths("/home/user", &["documents"]);
    assert_eq!(result, PathBuf::from("/home/user/documents"));
}

#[test]
fn test_join_paths_multiple_parts() {
    let result = join_paths("/home/user", &["documents", "reports", "2024"]);
    assert_eq!(result, PathBuf::from("/home/user/documents/reports/2024"));
}

#[test]
fn test_join_paths_empty_parts() {
    let result = join_paths("/home/user", &[]);
    assert_eq!(result, PathBuf::from("/home/user"));
}

#[test]
fn test_join_paths_relative_base() {
    let result = join_paths("relative/path", &["file.txt"]);
    assert_eq!(result, PathBuf::from("relative/path/file.txt"));
}

#[test]
fn test_join_paths_with_file() {
    let result = join_paths("/var/log", &["app", "error.log"]);
    assert_eq!(result, PathBuf::from("/var/log/app/error.log"));
}

// ============== get_extension tests ==============

#[test]
fn test_get_extension_simple() {
    assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
}

#[test]
fn test_get_extension_pdf() {
    assert_eq!(get_extension("report.pdf"), Some("pdf".to_string()));
}

#[test]
fn test_get_extension_double() {
    assert_eq!(get_extension("archive.tar.gz"), Some("gz".to_string()));
}

#[test]
fn test_get_extension_none() {
    assert_eq!(get_extension("Makefile"), None);
}

#[test]
fn test_get_extension_hidden_file() {
    assert_eq!(get_extension(".gitignore"), None);
}

#[test]
fn test_get_extension_hidden_with_ext() {
    assert_eq!(get_extension(".bashrc.bak"), Some("bak".to_string()));
}

#[test]
fn test_get_extension_full_path() {
    assert_eq!(get_extension("/home/user/doc.txt"), Some("txt".to_string()));
}

#[test]
fn test_get_extension_empty_extension() {
    assert_eq!(get_extension("file."), Some("".to_string()));
}

// ============== get_file_name tests ==============

#[test]
fn test_get_file_name_simple() {
    assert_eq!(get_file_name("document.txt"), Some("document.txt".to_string()));
}

#[test]
fn test_get_file_name_with_path() {
    assert_eq!(get_file_name("/home/user/document.txt"), Some("document.txt".to_string()));
}

#[test]
fn test_get_file_name_no_extension() {
    assert_eq!(get_file_name("/bin/bash"), Some("bash".to_string()));
}

#[test]
fn test_get_file_name_hidden() {
    assert_eq!(get_file_name("/home/user/.bashrc"), Some(".bashrc".to_string()));
}

#[test]
fn test_get_file_name_trailing_slash() {
    // Rust's Path::file_name() ignores trailing slashes
    assert_eq!(get_file_name("/home/user/"), Some("user".to_string()));
}

#[test]
fn test_get_file_name_root() {
    assert_eq!(get_file_name("/"), None);
}

#[test]
fn test_get_file_name_double_dot() {
    assert_eq!(get_file_name("/home/user/.."), None);
}

// ============== get_file_stem tests ==============

#[test]
fn test_get_file_stem_simple() {
    assert_eq!(get_file_stem("document.txt"), Some("document".to_string()));
}

#[test]
fn test_get_file_stem_double_extension() {
    assert_eq!(get_file_stem("archive.tar.gz"), Some("archive.tar".to_string()));
}

#[test]
fn test_get_file_stem_no_extension() {
    assert_eq!(get_file_stem("Makefile"), Some("Makefile".to_string()));
}

#[test]
fn test_get_file_stem_hidden() {
    assert_eq!(get_file_stem(".gitignore"), Some(".gitignore".to_string()));
}

#[test]
fn test_get_file_stem_hidden_with_ext() {
    assert_eq!(get_file_stem(".bashrc.bak"), Some(".bashrc".to_string()));
}

#[test]
fn test_get_file_stem_with_path() {
    assert_eq!(get_file_stem("/home/user/report.pdf"), Some("report".to_string()));
}

// ============== get_parent tests ==============

#[test]
fn test_get_parent_simple() {
    assert_eq!(get_parent("/home/user/file.txt"), Some(PathBuf::from("/home/user")));
}

#[test]
fn test_get_parent_nested() {
    assert_eq!(get_parent("/var/log/app/error.log"), Some(PathBuf::from("/var/log/app")));
}

#[test]
fn test_get_parent_root() {
    assert_eq!(get_parent("/"), None);
}

#[test]
fn test_get_parent_single_component() {
    // A single component relative path has no meaningful parent
    assert_eq!(get_parent("file.txt"), None);
}

#[test]
fn test_get_parent_relative() {
    assert_eq!(get_parent("path/to/file.txt"), Some(PathBuf::from("path/to")));
}

#[test]
fn test_get_parent_two_levels() {
    assert_eq!(get_parent("/home/user"), Some(PathBuf::from("/home")));
}

// ============== change_extension tests ==============

#[test]
fn test_change_extension_simple() {
    assert_eq!(change_extension("report.doc", "pdf"), PathBuf::from("report.pdf"));
}

#[test]
fn test_change_extension_add_new() {
    assert_eq!(change_extension("Makefile", "bak"), PathBuf::from("Makefile.bak"));
}

#[test]
fn test_change_extension_double() {
    assert_eq!(change_extension("archive.tar.gz", "bz2"), PathBuf::from("archive.tar.bz2"));
}

#[test]
fn test_change_extension_with_path() {
    assert_eq!(
        change_extension("/home/user/doc.txt", "md"),
        PathBuf::from("/home/user/doc.md")
    );
}

#[test]
fn test_change_extension_remove() {
    assert_eq!(change_extension("file.txt", ""), PathBuf::from("file"));
}

#[test]
fn test_change_extension_hidden() {
    assert_eq!(change_extension(".gitignore", "bak"), PathBuf::from(".gitignore.bak"));
}

// ============== is_absolute tests ==============

#[test]
fn test_is_absolute_unix_root() {
    assert!(is_absolute("/"));
}

#[test]
fn test_is_absolute_unix_path() {
    assert!(is_absolute("/home/user/documents"));
}

#[test]
fn test_is_absolute_relative() {
    assert!(!is_absolute("relative/path"));
}

#[test]
fn test_is_absolute_current_dir() {
    assert!(!is_absolute("./file.txt"));
}

#[test]
fn test_is_absolute_parent_dir() {
    assert!(!is_absolute("../file.txt"));
}

#[test]
fn test_is_absolute_just_filename() {
    assert!(!is_absolute("file.txt"));
}

// ============== normalize_path tests ==============

#[test]
fn test_normalize_path_simple() {
    let result = normalize_path("home/user/file.txt");
    assert_eq!(result, PathBuf::from("home/user/file.txt"));
}

#[test]
fn test_normalize_path_absolute() {
    let result = normalize_path("/var/log/app.log");
    assert_eq!(result, PathBuf::from("/var/log/app.log"));
}

#[test]
fn test_normalize_path_single_component() {
    let result = normalize_path("file.txt");
    assert_eq!(result, PathBuf::from("file.txt"));
}

#[test]
fn test_normalize_path_with_dots() {
    let result = normalize_path("./relative/./path");
    assert_eq!(result, PathBuf::from("./relative/./path"));
}

// ============== Integration tests ==============

#[test]
fn test_build_and_extract_path() {
    // Build a path
    let full = join_paths("/home/user", &["documents", "report.pdf"]);

    // Extract components
    assert_eq!(get_file_name(full.to_str().unwrap()), Some("report.pdf".to_string()));
    assert_eq!(get_extension(full.to_str().unwrap()), Some("pdf".to_string()));
    assert_eq!(get_file_stem(full.to_str().unwrap()), Some("report".to_string()));
    assert_eq!(
        get_parent(full.to_str().unwrap()),
        Some(PathBuf::from("/home/user/documents"))
    );
}

#[test]
fn test_change_extension_workflow() {
    let original = "/home/user/document.doc";

    // Change to different format
    let as_pdf = change_extension(original, "pdf");
    assert_eq!(as_pdf, PathBuf::from("/home/user/document.pdf"));

    // Create backup
    let backup = change_extension(original, "doc.bak");
    assert_eq!(backup, PathBuf::from("/home/user/document.doc.bak"));
}

#[test]
fn test_path_analysis() {
    let paths = vec![
        ("/home/user/file.txt", true, Some("txt")),
        ("relative/path/doc.pdf", false, Some("pdf")),
        ("/bin/bash", true, None),
        ("Makefile", false, None),
    ];

    for (path, expected_absolute, expected_ext) in paths {
        assert_eq!(is_absolute(path), expected_absolute, "Failed for path: {}", path);
        assert_eq!(
            get_extension(path).as_deref(),
            expected_ext,
            "Failed extension for path: {}",
            path
        );
    }
}

#[test]
fn test_nested_directories() {
    let base = "/var";
    let full = join_paths(base, &["log", "nginx", "access.log"]);

    assert_eq!(full, PathBuf::from("/var/log/nginx/access.log"));
    assert!(is_absolute(full.to_str().unwrap()));

    let parent = get_parent(full.to_str().unwrap()).unwrap();
    assert_eq!(parent, PathBuf::from("/var/log/nginx"));

    let grandparent = get_parent(parent.to_str().unwrap()).unwrap();
    assert_eq!(grandparent, PathBuf::from("/var/log"));
}

#[test]
fn test_hidden_files() {
    let hidden = ".config";
    let hidden_with_ext = ".bashrc.backup";

    assert_eq!(get_extension(hidden), None);
    assert_eq!(get_file_stem(hidden), Some(".config".to_string()));

    assert_eq!(get_extension(hidden_with_ext), Some("backup".to_string()));
    assert_eq!(get_file_stem(hidden_with_ext), Some(".bashrc".to_string()));
}
