Working with file paths is essential in any program that interacts with the filesystem. Rust provides the `Path` and `PathBuf` types in `std::path` for safe, cross-platform path manipulation. Unlike raw strings, these types understand path semantics and handle platform differences automatically.

## Path vs PathBuf

`Path` is an unsized type (like `str`) that represents a borrowed path, while `PathBuf` is an owned, mutable path (like `String`). You'll typically use `&Path` for function parameters and `PathBuf` when you need to own or modify a path:

```rust
use std::path::{Path, PathBuf};

let path: &Path = Path::new("/home/user/file.txt");
let mut owned: PathBuf = PathBuf::from("/home/user");
owned.push("documents");
owned.push("file.txt");
```

## Building Paths Safely

Use `.join()` to combine path components safely - it handles separators automatically:

```rust
let base = Path::new("/home/user");
let full = base.join("documents").join("report.pdf");
// Result: /home/user/documents/report.pdf
```

## Extracting Path Components

The `Path` type provides methods to extract various components:

```rust
let path = Path::new("/home/user/document.txt");

path.file_name();      // Some("document.txt")
path.extension();      // Some("txt")
path.file_stem();      // Some("document")
path.parent();         // Some("/home/user")
```

## Your Task

Implement the following functions for path manipulation:

1. `join_paths(base: &str, parts: &[&str]) -> PathBuf` - Join a base path with multiple components
2. `get_extension(path: &str) -> Option<String>` - Extract the file extension (without the dot)
3. `get_file_name(path: &str) -> Option<String>` - Extract the file name (including extension)
4. `get_file_stem(path: &str) -> Option<String>` - Extract the file name without extension
5. `get_parent(path: &str) -> Option<PathBuf>` - Get the parent directory
6. `change_extension(path: &str, new_ext: &str) -> PathBuf` - Change the file extension
7. `is_absolute(path: &str) -> bool` - Check if a path is absolute
8. `normalize_path(path: &str) -> PathBuf` - Convert a path to use the platform's separator

## Examples

```rust
// Join paths safely
let result = join_paths(
    "/home/user",
    &["documents", "reports", "2024"]
);
let expected = PathBuf::from(
    "/home/user/documents/reports/2024"
);
assert_eq!(result, expected);

// Extract extension
assert_eq!(get_extension("report.pdf"), Some("pdf".to_string()));
assert_eq!(get_extension("Makefile"), None);

// Get file name
assert_eq!(
    get_file_name("/home/user/doc.txt"),
    Some("doc.txt".to_string())
);

// Get file stem (name without extension)
assert_eq!(
    get_file_stem("archive.tar.gz"),
    Some("archive.tar".to_string())
);

// Get parent directory
assert_eq!(
    get_parent("/home/user/file.txt"),
    Some(PathBuf::from("/home/user"))
);
assert_eq!(get_parent("/"), None);

// Change extension
assert_eq!(
    change_extension("report.doc", "pdf"),
    PathBuf::from("report.pdf")
);
assert_eq!(
    change_extension("Makefile", "bak"),
    PathBuf::from("Makefile.bak")
);

// Check if absolute
assert!(is_absolute("/home/user"));
assert!(!is_absolute("relative/path"));

// Normalize path (result depends on platform)
let normalized = normalize_path("home/user/file.txt");
// On Unix: "home/user/file.txt"
// On Windows: "home\\user\\file.txt"
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `Path::new()` to create a `Path` reference from a string
- Use `PathBuf::from()` to create an owned path
- The `.join()` method returns a new `PathBuf` with the path component appended
- `.extension()` returns an `Option<&OsStr>` - convert with `.to_str()` and `.to_string()`
- `.file_name()` and `.file_stem()` also return `Option<&OsStr>`
- `.parent()` returns `Option<&Path>` - convert to `PathBuf` with `.to_path_buf()`
- Use `.set_extension()` on a `PathBuf` to change the extension
- `.is_absolute()` checks if a path starts from a root
- `PathBuf::from()` with a string already normalizes separators for the current platform

</details>
