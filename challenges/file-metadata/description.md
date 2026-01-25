When working with files, you often need more than just their contents. File metadata provides crucial information about files: their size, when they were created or modified, what type they are (file, directory, symlink), and their permissions. Rust's `std::fs::Metadata` struct gives you access to all of this information through a clean, cross-platform API.

The `metadata()` function (or `symlink_metadata()` for symlinks) returns a `Metadata` struct containing information about a file. This is useful for filtering files by size or date, checking permissions before attempting operations, or building file managers and backup tools.

## Getting File Metadata

You can obtain metadata using `std::fs::metadata()` or from a path directly:

```rust
use std::fs;

// From a path
let meta = fs::metadata("file.txt")?;

// Check file size in bytes
let size = meta.len();

// Check if it's a file, directory, or symlink
let is_file = meta.is_file();
let is_dir = meta.is_dir();
let file_type = meta.file_type();

// Get modification and access times
let modified = meta.modified()?;
let accessed = meta.accessed()?;
```

## File Types

The `FileType` struct tells you what kind of filesystem entry you're dealing with:

```rust
let file_type = meta.file_type();
if file_type.is_file() {
    println!("Regular file");
} else if file_type.is_dir() {
    println!("Directory");
} else if file_type.is_symlink() {
    println!("Symbolic link");
}
```

## Working with Times

Modification and access times are returned as `SystemTime`, which you can compare or convert to durations:

```rust
use std::time::{SystemTime, Duration};

let modified = meta.modified()?;
let age = SystemTime::now().duration_since(modified)?;
println!("File was modified {} seconds ago", age.as_secs());
```

## Platform-Specific Permissions

On Unix systems, you can access file permissions through the `PermissionsExt` trait:

```rust
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

let perms = meta.permissions();
#[cfg(unix)]
{
    let mode = perms.mode();
    let is_executable = mode & 0o111 != 0;  // Check execute bits
}
```

## Your Task

Implement the following functions to work with file metadata:

1. `get_file_size(path: &Path) -> io::Result<u64>` - Returns the file size in bytes
2. `get_file_type(path: &Path) -> io::Result<String>` - Returns "file", "directory", or "symlink"
3. `is_readonly(path: &Path) -> io::Result<bool>` - Checks if the file is read-only
4. `get_modified_time(path: &Path) -> io::Result<SystemTime>` - Returns when the file was last modified
5. `was_modified_within(path: &Path, duration: Duration) -> io::Result<bool>` - Checks if file was modified within the given duration
6. `is_executable(path: &Path) -> io::Result<bool>` - Checks if file has execute permissions (Unix) or has executable extension (Windows)
7. `compare_modified_times(path1: &Path, path2: &Path) -> io::Result<Ordering>` - Compares which file was modified more recently

## Examples

```rust
use std::path::Path;
use std::time::Duration;
use std::cmp::Ordering;

// Get file size
let size = get_file_size(Path::new("data.txt"))?;
assert!(size > 0);

// Check file type
let file_type = get_file_type(Path::new("./src"))?;
assert_eq!(file_type, "directory");

// Check if read-only
let readonly = is_readonly(Path::new("config.txt"))?;

// Get modification time
let modified = get_modified_time(Path::new("file.txt"))?;

// Check if recently modified (within last hour)
let recent = was_modified_within(Path::new("log.txt"), Duration::from_secs(3600))?;

// Check if executable
let executable = is_executable(Path::new("script.sh"))?;

// Compare modification times
let ordering = compare_modified_times(
    Path::new("file1.txt"),
    Path::new("file2.txt")
)?;
match ordering {
    Ordering::Less => println!("file1 is older"),
    Ordering::Greater => println!("file1 is newer"),
    Ordering::Equal => println!("Same modification time"),
}
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::fs::metadata(path)` to get the `Metadata` struct
- `Metadata::len()` returns file size in bytes
- `Metadata::file_type()` returns a `FileType` with `is_file()`, `is_dir()`, `is_symlink()` methods
- `Metadata::permissions()` returns a `Permissions` struct with `readonly()` method
- `Metadata::modified()` returns `io::Result<SystemTime>` - the modification time
- For `was_modified_within`, use `SystemTime::now().duration_since(modified)` to get elapsed time
- For Unix executable check, use `#[cfg(unix)]` and `std::os::unix::fs::PermissionsExt::mode()`
- The execute bit can be checked with `mode & 0o111 != 0` (any of owner/group/other execute)
- For Windows, you might check file extension (`.exe`, `.bat`, `.cmd`) as an alternative
- For comparing times, `SystemTime` implements `PartialOrd` so you can compare directly

</details>
