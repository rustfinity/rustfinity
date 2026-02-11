use std::io::{self, BufRead, Write};

/// Read a single line from any reader implementing `BufRead`.
/// The trailing newline (both `\n` and `\r\n`) is trimmed from the result.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use stdio_operations::read_line_from_reader;
///
/// let input = Cursor::new("Hello, World!\n");
/// let line = read_line_from_reader(input).unwrap();
/// assert_eq!(line, "Hello, World!");
/// ```
pub fn read_line_from_reader<R: BufRead>(mut reader: R) -> io::Result<String> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    // Trim trailing newline characters (handles both \n and \r\n)
    let trimmed = buffer.trim_end_matches('\n').trim_end_matches('\r');
    Ok(trimmed.to_string())
}

/// Read all lines from a reader into a Vec.
/// Each line has its trailing newline removed.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use stdio_operations::read_all_lines_from_reader;
///
/// let input = Cursor::new("Line 1\nLine 2\nLine 3\n");
/// let lines = read_all_lines_from_reader(input).unwrap();
/// assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
/// ```
pub fn read_all_lines_from_reader<R: BufRead>(reader: R) -> io::Result<Vec<String>> {
    reader.lines().collect()
}

/// Write a message to any writer implementing `Write`.
/// Does not add a trailing newline.
///
/// # Examples
///
/// ```
/// use stdio_operations::write_to_writer;
///
/// let mut output = Vec::new();
/// write_to_writer(&mut output, "Hello").unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Hello");
/// ```
pub fn write_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    write!(writer, "{}", message)
}

/// Write a message with a trailing newline to any writer implementing `Write`.
///
/// # Examples
///
/// ```
/// use stdio_operations::writeln_to_writer;
///
/// let mut output = Vec::new();
/// writeln_to_writer(&mut output, "Hello").unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
/// ```
pub fn writeln_to_writer<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    writeln!(writer, "{}", message)
}

/// Write a message and immediately flush the buffer.
/// Useful for prompts or real-time output.
///
/// # Examples
///
/// ```
/// use stdio_operations::write_and_flush;
///
/// let mut output = Vec::new();
/// write_and_flush(&mut output, "Enter name: ").unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Enter name: ");
/// ```
pub fn write_and_flush<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    write!(writer, "{}", message)?;
    writer.flush()
}

/// Write an error message in the format "[ERROR] message" with a newline.
///
/// # Examples
///
/// ```
/// use stdio_operations::write_error_to_writer;
///
/// let mut output = Vec::new();
/// write_error_to_writer(&mut output, "File not found").unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] File not found\n");
/// ```
pub fn write_error_to_writer<W: Write>(writer: &mut W, error: &str) -> io::Result<()> {
    writeln!(writer, "[ERROR] {}", error)
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
