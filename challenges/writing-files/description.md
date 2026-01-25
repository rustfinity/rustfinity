Writing files is a fundamental operation in Rust programming. The standard library provides several ways to write data to files, each suited for different use cases - from simple one-shot writes to buffered streaming writes for optimal performance.

## Creating and Writing Files

The simplest way to write a file is using `std::fs::write`:

```rust
use std::fs;

fs::write("output.txt", "Hello, World!")?;
```

For more control, use `File::create` with `Write` trait methods:

```rust
use std::fs::File;
use std::io::Write;

let mut file = File::create("output.txt")?;
file.write_all(b"Hello, World!")?;
```

## Buffered Writing with BufWriter

For many small writes, `BufWriter` provides efficient buffering:

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

let file = File::create("output.txt")?;
let mut writer = BufWriter::new(file);

for i in 1..=1000 {
    writeln!(writer, "Line {}", i)?;
}
// Buffer is automatically flushed when writer is dropped
```

The `BufWriter` collects writes in memory and flushes them to disk in larger chunks, which is significantly more efficient for many small writes.

## Appending to Files

To add content to an existing file without overwriting:

```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")?;

writeln!(file, "New log entry")?;
```

## Your Task

Implement the following functions for writing file contents:

1. `write_string(path: &str, content: &str) -> Result<(), io::Error>` - Write a string to a file, creating or overwriting it
2. `write_bytes(path: &str, data: &[u8]) -> Result<(), io::Error>` - Write raw bytes to a file
3. `append_string(path: &str, content: &str) -> Result<(), io::Error>` - Append a string to a file (create if it doesn't exist)
4. `write_lines(path: &str, lines: &[&str]) -> Result<(), io::Error>` - Write multiple lines to a file (each followed by a newline)
5. `write_with_buffer(path: &str, chunks: &[&str]) -> Result<(), io::Error>` - Write multiple chunks using buffered writing for efficiency

## Examples

```rust
// Write a simple string
write_string("hello.txt", "Hello, World!")?;
// File now contains: Hello, World!

// Write raw bytes
write_bytes("binary.dat", &[0x48, 0x65, 0x6c, 0x6c, 0x6f])?;
// File now contains: Hello (as bytes)

// Append to a file
write_string("log.txt", "First entry\n")?;
append_string("log.txt", "Second entry\n")?;
// File now contains:
// First entry
// Second entry

// Write multiple lines
write_lines("list.txt", &["Apple", "Banana", "Cherry"])?;
// File now contains:
// Apple
// Banana
// Cherry

// Efficient buffered writing
write_with_buffer("output.txt", &["Hello, ", "World", "!"])?;
// File now contains: Hello, World!
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::fs::write` for the simplest way to write a string or bytes to a file
- Use `OpenOptions::new().append(true).create(true).open(path)` to open a file for appending
- Use `BufWriter::new(File::create(path)?)` to create a buffered writer
- The `writeln!` macro writes a line followed by a newline character
- Use `write!` or `write_all` for writing without a trailing newline
- Remember to handle the `Result` type returned by all I/O operations
- `BufWriter` automatically flushes when dropped, but you can call `.flush()` explicitly if needed

</details>
