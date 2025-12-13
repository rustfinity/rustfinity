use result_to_option::*;
use std::fs::{remove_file, write};

#[test]
fn test_read_file_valid_file() {
    let file_path = "test_valid.txt";
    write(file_path, "Hello, world!\nSecond line.").unwrap();
    assert_eq!(
        read_file(file_path),
        Some("Hello, world!\nSecond line.".to_string())
    );
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_file_empty_file() {
    let file_path = "test_empty.txt";
    write(file_path, "").unwrap();
    assert_eq!(read_file(file_path), Some("".to_string()));
    remove_file(file_path).unwrap();
}

#[test]
fn test_read_file_nonexistent_file() {
    let file_path = "nonexistent.txt";
    assert_eq!(read_file(file_path), None);
}

#[test]
fn test_read_file_permission_denied() {
    #[cfg(unix)]
    {
        use std::fs::set_permissions;
        use std::os::unix::fs::PermissionsExt;
        use tempfile::NamedTempFile;

        let tmp_file = NamedTempFile::new().unwrap();
        write(&tmp_file, "Cannot read this file.").unwrap();

        let permissions = set_permissions(&tmp_file, PermissionsExt::from_mode(0o000));
        assert!(permissions.is_ok());
        assert_eq!(read_file(tmp_file.path().to_str().unwrap()), None);
    }
}

#[test]
fn test_read_file_large_file() {
    let file_path = "test_large.txt";
    let content = "a".repeat(10_000); // 10 KB of data
    write(file_path, &content).unwrap();
    assert_eq!(read_file(file_path), Some(content));
    remove_file(file_path).unwrap();
}
