use std::io::{self, BufRead, Write};

/// Read a single line from any reader implementing `BufRead`.
/// The trailing newline (both `\n` and `\r\n`) is trimmed from the result.
pub fn read_line_from_reader<R: BufRead>(mut reader: R) -> io::Result<String> {
    // TODO: Implement this function
    unimplemented!()
}

/// Read all lines from a reader into a Vec.
/// Each line has its trailing newline removed.
pub fn read_all_lines_from_reader<R: BufRead>(reader: R) -> io::Result<Vec<String>> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message to any writer implementing `Write`.
/// Does not add a trailing newline.
pub fn write_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message with a trailing newline to any writer implementing `Write`.
pub fn writeln_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write a message and immediately flush the buffer.
/// Useful for prompts or real-time output.
pub fn write_and_flush<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

/// Write an error message in the format "[ERROR] message" with a newline.
pub fn write_error_to_writer<W: Write>(writer: &mut W, error: &str) -> io::Result<()> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    use std::io::Cursor;

    println!("=== Stdio Operations Demo ===\n");

    // Demonstrate reading from a mock input
    println!("Reading a single line:");
    let input = Cursor::new("Hello from stdin!\n");
    match read_line_from_reader(input) {
        Ok(line) => println!("  Read: '{}'", line),
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Demonstrate reading multiple lines
    println!("\nReading multiple lines:");
    let input = Cursor::new("First line\nSecond line\nThird line\n");
    match read_all_lines_from_reader(input) {
        Ok(lines) => {
            for (i, line) in lines.iter().enumerate() {
                println!("  Line {}: '{}'", i + 1, line);
            }
        }
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Demonstrate writing to stdout
    println!("\nWriting to stdout:");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let _ = write_to_writer(&mut handle, "  This is written without newline... ");
    let _ = write_and_flush(&mut handle, "and this is flushed immediately!\n");
    let _ = writeln_to_writer(&mut handle, "  This has an automatic newline");

    // Demonstrate writing to stderr
    println!("\nWriting to stderr:");
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = write_error_to_writer(&mut handle, "This is an error message");

    println!("\n=== Demo Complete ===");
}
