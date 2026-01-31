Standard I/O (stdio) operations are fundamental for any program that interacts with users or other processes. Rust provides safe, efficient access to standard input, output, and error streams through the `std::io` module.

## Understanding Standard Streams

Every process has three standard streams:

- **stdin** (standard input): For reading input, typically from the keyboard or piped data
- **stdout** (standard output): For normal program output
- **stderr** (standard error): For error messages and diagnostics (not buffered by default)

## Reading from stdin

Use `std::io::stdin()` to get a handle to standard input:

```rust
use std::io::{self, BufRead};

let stdin = io::stdin();
let mut line = String::new();
stdin.read_line(&mut line)?;
// line now contains the input including the newline character
```

For reading multiple lines, use the `lines()` iterator from `BufRead`:

```rust
use std::io::{self, BufRead};

let stdin = io::stdin();
for line in stdin.lock().lines() {
    let line = line?;
    println!("Read: {}", line);
}
```

## Writing to stdout and stderr

Use `std::io::stdout()` and `std::io::stderr()` for output handles:

```rust
use std::io::{self, Write};

let mut stdout = io::stdout();
writeln!(stdout, "Normal output")?;
stdout.flush()?;  // Ensure output is written immediately

let mut stderr = io::stderr();
writeln!(stderr, "Error message")?;
```

## Buffering and Flushing

stdout is line-buffered when connected to a terminal, meaning output is flushed after each newline. When writing to a file or pipe, it's fully buffered. Use `.flush()` to ensure output is written immediately:

```rust
use std::io::{self, Write};

print!("Enter your name: ");
io::stdout().flush()?;  // Important! Without this, prompt might not appear
```

## Your Task

Implement the following functions for working with standard I/O:

1. `read_line_from_reader<R: BufRead>(reader: R) -> io::Result<String>` - Read a single line from any reader implementing `BufRead`, trimming the trailing newline
2. `read_all_lines_from_reader<R: BufRead>(reader: R) -> io::Result<Vec<String>>` - Read all lines from a reader into a Vec
3. `write_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message to any writer implementing `Write`
4. `writeln_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message with a newline to any writer
5. `write_and_flush<W: Write>(writer: &mut W, message: &str) -> io::Result<()>` - Write a message and immediately flush the buffer
6. `write_error_to_writer<W: Write>(writer: &mut W, error: &str) -> io::Result<()>` - Write an error message in the format "[ERROR] message" with a newline

Note: The functions are generic over `BufRead` and `Write` traits to make them testable without needing actual stdin/stdout.

## Examples

```rust
use std::io::Cursor;

// Reading from a mock input
let input = Cursor::new("Hello, World!\n");
let line = read_line_from_reader(input)?;
assert_eq!(line, "Hello, World!");

// Reading multiple lines
let input = Cursor::new("Line 1\nLine 2\nLine 3\n");
let lines = read_all_lines_from_reader(input)?;
assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);

// Writing to a buffer
let mut output = Vec::new();
write_to_writer(&mut output, "Hello")?;
assert_eq!(String::from_utf8(output).unwrap(), "Hello");

// Writing with newline
let mut output = Vec::new();
writeln_to_writer(&mut output, "Hello")?;
assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");

// Writing an error message
let mut output = Vec::new();
write_error_to_writer(&mut output, "Something went wrong")?;
assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] Something went wrong\n");
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `reader.read_line(&mut buffer)` to read a single line
- Use `buffer.trim_end_matches('\n').trim_end_matches('\r')` to handle both Unix and Windows line endings
- The `lines()` iterator from `BufRead` automatically strips newlines
- Use `write!(writer, "{}", message)` or `writer.write_all(message.as_bytes())` to write
- Use `writeln!(writer, "{}", message)` to write with a newline
- Call `writer.flush()` to ensure buffered output is written immediately
- Format error messages using `format!("[ERROR] {}", error)` or `write!(writer, "[ERROR] {}\n", error)`

</details>
