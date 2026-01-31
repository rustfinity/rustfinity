Temporary files are essential for many programming tasks: processing large datasets that don't fit in memory, storing intermediate results, creating scratch space for operations, or safely writing files before atomically moving them to their final location. Rust's standard library provides tools for working with temporary files safely.

## Understanding Temporary Files

The `std::env::temp_dir()` function returns the system's temporary directory path. On Unix-like systems, this is typically `/tmp` or the value of the `TMPDIR` environment variable. On Windows, it's usually `C:\Users\<username>\AppData\Local\Temp`.

```rust
use std::env;

let temp_path = env::temp_dir();
println!("System temp directory: {}", temp_path.display());
```

## Creating Unique Temporary Files

When creating temporary files, it's important to use unique names to avoid conflicts with other processes. A common pattern is to include a timestamp, process ID, or random component in the filename:

```rust
use std::fs::File;
use std::io::Write;
use std::env;
use std::process;

fn create_temp_file(
    prefix: &str,
    suffix: &str
) -> std::io::Result<std::path::PathBuf> {
    let mut path = env::temp_dir();
    let pid = process::id();
    let filename = format!("{}_{pid}_{}", prefix, suffix);
    path.push(filename);
    File::create(&path)?;
    Ok(path)
}
```

## Cleanup Patterns

Temporary files should be cleaned up when no longer needed. Rust's `Drop` trait is perfect for this - you can create a wrapper struct that deletes the file when it goes out of scope:

```rust
use std::path::PathBuf;
use std::fs;

struct TempFile {
    path: PathBuf,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
```

## Your Task

Implement the following functions and types for working with temporary files:

1. `get_temp_dir() -> PathBuf` - Return the system's temporary directory
2. `create_temp_file(prefix: &str, suffix: &str) -> io::Result<PathBuf>` - Create a uniquely named temp file and return its path
3. `create_temp_file_with_content(prefix: &str, suffix: &str, content: &str) -> io::Result<PathBuf>` - Create a temp file with content
4. `TempFile` struct that:
   - Has a `path` field of type `PathBuf`
   - Implements `new(prefix: &str, suffix: &str)
     -> io::Result<Self>` to create a new temp file
   - Implements `path(&self) -> &Path` to get the file path
   - Implements `write(&self, content: &str)
     -> io::Result<()>` to write content
   - Implements `read(&self) -> io::Result<String>` to read content
   - Implements `Drop` to automatically delete the file
     when the struct is dropped
5. `create_temp_dir(prefix: &str) -> io::Result<PathBuf>` - Create a uniquely named temp directory
6. `cleanup_temp_files(dir: &Path, prefix: &str)
   -> io::Result<usize>` - Delete all files in a directory
   that start with the given prefix, returning the count
   of deleted files

## Examples

```rust
// Get system temp directory
let temp_dir = get_temp_dir();
assert!(temp_dir.exists());

// Create a temporary file
let path = create_temp_file("myapp", ".tmp")?;
assert!(path.exists());
assert!(path.to_string_lossy().contains("myapp"));
// Manual cleanup
std::fs::remove_file(&path)?;

// Create temp file with content
let path = create_temp_file_with_content(
    "data",
    ".txt",
    "Hello, World!"
)?;
let content = std::fs::read_to_string(&path)?;
assert_eq!(content, "Hello, World!");
std::fs::remove_file(&path)?;

// Use TempFile for automatic cleanup
{
    let temp = TempFile::new("auto", ".tmp")?;
    temp.write("Some data")?;
    let content = temp.read()?;
    assert_eq!(content, "Some data");
    // File is automatically deleted when temp goes out of scope
}

// Create a temp directory
let temp_dir = create_temp_dir("workdir")?;
assert!(temp_dir.is_dir());
std::fs::remove_dir(&temp_dir)?;

// Cleanup files with prefix
let dir = get_temp_dir();
create_temp_file_with_content(
    "cleanup_test",
    ".tmp",
    "test"
)?;
create_temp_file_with_content(
    "cleanup_test",
    ".tmp",
    "test"
)?;
let deleted = cleanup_temp_files(&dir, "cleanup_test")?;
assert!(deleted >= 2);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::env::temp_dir()` to get the system temp directory
- Use `std::process::id()` to get the current process ID for unique filenames
- Use `std::time::SystemTime::now()` for additional
  uniqueness in filenames
- For the `TempFile` struct, store the path and implement
  `Drop` to call `std::fs::remove_file`
- Use `std::fs::create_dir()` to create directories
- For `cleanup_temp_files`, use `std::fs::read_dir()`
  to iterate and `starts_with()` for prefix checks
- Remember to handle errors gracefully - cleanup
  operations shouldn't panic if a file doesn't exist

</details>
