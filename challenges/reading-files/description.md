Reading files is a fundamental operation in any programming language. In Rust, the standard library provides several ways to read files, each suited for different use cases.

## Opening and Reading Files

The most basic way to read a file is using `std::fs::read_to_string`:

```rust
use std::fs;

let contents = fs::read_to_string("file.txt")?;
println!("{}", contents);
```

For more control, you can use `File::open` with `Read` trait methods:

```rust
use std::fs::File;
use std::io::Read;

let mut file = File::open("file.txt")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
```

## Buffered Reading with BufReader

For line-by-line reading, `BufReader` provides efficient buffered access:

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let file = File::open("file.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}
```

The `BufReader` buffers reads internally, which is more efficient than reading one byte at a time, especially for large files.

## Your Task

Implement the following functions for reading file contents:

1. `read_entire_file(path: &str) -> Result<String, io::Error>` - Read the entire contents of a file into a String
2. `count_lines(path: &str) -> Result<usize, io::Error>` - Count the number of lines in a file
3. `count_words(path: &str) -> Result<usize, io::Error>` - Count the number of words in a file (words are whitespace-separated)
4. `read_lines(path: &str) -> Result<Vec<String>, io::Error>` - Read all lines into a Vec, with trailing newlines removed
5. `first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error>` - Read only the first n lines from a file

## Examples

```rust
// Assuming a file "example.txt" contains:
// Hello World
// This is a test
// Rust is great

assert_eq!(
    read_entire_file("example.txt")?,
    "Hello World\nThis is a test\nRust is great\n"
);
assert_eq!(count_lines("example.txt")?, 3);
assert_eq!(count_words("example.txt")?, 9);
assert_eq!(
    read_lines("example.txt")?,
    vec!["Hello World", "This is a test", "Rust is great"]
);
assert_eq!(
    first_n_lines("example.txt", 2)?,
    vec!["Hello World", "This is a test"]
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::fs::read_to_string` for the simplest way to read a whole file
- Use `BufReader::new(File::open(path)?)` to create a buffered reader
- The `lines()` iterator from `BufRead` returns `Result<String, io::Error>` for each line
- Use `split_whitespace()` to split a string into words
- Use `.take(n)` on an iterator to limit how many items you process
- Remember to handle the `Result` type returned by `File::open` and I/O operations

</details>
