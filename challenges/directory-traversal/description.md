Traversing directories is a fundamental operation when working with filesystems. Whether you're building a file search tool, calculating disk usage, or processing batches of files, you need to navigate directory trees efficiently. Rust's `std::fs` module provides `read_dir()` for listing directory contents and `metadata()` for inspecting file properties.

## Reading Directory Contents

The `std::fs::read_dir()` function returns an iterator over directory entries:

```rust
use std::fs;

for entry in fs::read_dir("./src")? {
    let entry = entry?;
    let path = entry.path();
    println!("{}", path.display());
}
```

Each `DirEntry` provides access to the file's path, name, metadata, and file type without additional system calls for basic information.

## Recursive Traversal

To traverse subdirectories, you need to implement recursion yourself. Check if each entry is a directory using `entry.file_type()?.is_dir()` and recurse into it:

```rust
fn visit_dirs(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Recurse into subdirectory
            visit_dirs(&path)?;
        } else {
            println!("File: {}", path.display());
        }
    }
    Ok(())
}
```

## Filtering Files

You can filter entries by extension, name pattern, or other criteria as you iterate:

```rust
for entry in fs::read_dir(".")? {
    let path = entry?.path();
    if path.extension().map_or(false, |ext| ext == "rs") {
        println!("Rust file: {}", path.display());
    }
}
```

## Your Task

Implement the following directory traversal functions:

1. `list_files(dir: &Path) -> io::Result<Vec<PathBuf>>`
   - List all files (not directories) in a directory
     (non-recursive)
2. `list_directories(dir: &Path) -> io::Result<Vec<PathBuf>>`
   - List all subdirectories in a directory (non-recursive)
3. `list_all_recursive(dir: &Path) -> io::Result<Vec<PathBuf>>`
   - Recursively list all files and directories
4. `find_by_extension(dir: &Path, ext: &str) -> io::Result<Vec<PathBuf>>`
   - Find all files with a given extension (recursive)
5. `find_by_name(dir: &Path, name: &str) -> io::Result<Vec<PathBuf>>`
   - Find all files/directories matching a name exactly
     (recursive)
6. `calculate_dir_size(dir: &Path) -> io::Result<u64>`
   - Calculate total size of all files in a directory
     (recursive)
7. `count_files_recursive(dir: &Path) -> io::Result<usize>` - Count all files in a directory tree

## Examples

```rust
use std::path::Path;

// List only files in a directory
// (not subdirectories or their contents)
let files = list_files(Path::new("./src"))?;
// Returns: ["./src/main.rs", "./src/lib.rs"]

// List only immediate subdirectories
let dirs = list_directories(Path::new("./project"))?;
// Returns: ["./project/src", "./project/tests"]

// Recursively list everything
let all = list_all_recursive(Path::new("./src"))?;
// Returns all files and directories at all levels

// Find all Rust source files
let rust_files = find_by_extension(Path::new("./"), "rs")?;
// Returns all .rs files recursively

// Find files or directories named "Cargo.toml"
let cargo_files = find_by_name(Path::new("./"), "Cargo.toml")?;
// Returns all paths ending in "Cargo.toml"

// Calculate total size of a directory
let size = calculate_dir_size(Path::new("./src"))?;
// Returns total bytes of all files

// Count files in directory tree
let count = count_files_recursive(Path::new("./src"))?;
// Returns number of files (not directories)
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::fs::read_dir(path)` to get an iterator over directory entries
- Each entry from `read_dir()` is a `Result<DirEntry, io::Error>` - handle errors with `?`
- Use `entry.path()` to get the full `PathBuf` for an entry
- Use `entry.file_type()?.is_dir()` or `entry.path().is_dir()` to check if it's a directory
- For recursive functions, call the function on subdirectories and combine results
- Use `entry.metadata()?.len()` to get the file size
  in bytes
- For extension matching, use `path.extension().map_or(false, |e| e == ext)`
- For name matching, use `path.file_name().map_or(false, |n| n == name)`
- Remember that `read_dir()` only lists immediate children, not nested contents

</details>
