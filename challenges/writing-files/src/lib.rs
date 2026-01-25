use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};

/// Write a string to a file, creating or overwriting it.
///
/// # Examples
///
/// ```no_run
/// use writing_files::write_string;
///
/// write_string("output.txt", "Hello, World!").unwrap();
/// ```
pub fn write_string(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Write raw bytes to a file, creating or overwriting it.
///
/// # Examples
///
/// ```no_run
/// use writing_files::write_bytes;
///
/// write_bytes("binary.dat", &[0x48, 0x65, 0x6c, 0x6c, 0x6f]).unwrap();
/// ```
pub fn write_bytes(path: &str, data: &[u8]) -> Result<(), io::Error> {
    fs::write(path, data)
}

/// Append a string to a file, creating it if it doesn't exist.
///
/// # Examples
///
/// ```no_run
/// use writing_files::append_string;
///
/// append_string("log.txt", "New log entry\n").unwrap();
/// ```
pub fn append_string(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}

/// Write multiple lines to a file, each followed by a newline.
///
/// # Examples
///
/// ```no_run
/// use writing_files::write_lines;
///
/// write_lines("list.txt", &["Apple", "Banana", "Cherry"]).unwrap();
/// // File contains:
/// // Apple
/// // Banana
/// // Cherry
/// ```
pub fn write_lines(path: &str, lines: &[&str]) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Write multiple chunks to a file using buffered writing for efficiency.
///
/// # Examples
///
/// ```no_run
/// use writing_files::write_with_buffer;
///
/// write_with_buffer("output.txt", &["Hello, ", "World", "!"]).unwrap();
/// // File contains: Hello, World!
/// ```
pub fn write_with_buffer(path: &str, chunks: &[&str]) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for chunk in chunks {
        writer.write_all(chunk.as_bytes())?;
    }
    writer.flush()
}

pub fn main() {
    // Example: Write various types of content to files

    // Write a simple string
    if write_string("example_string.txt", "Hello, World!").is_ok() {
        println!("String written successfully.");
        let _ = fs::remove_file("example_string.txt");
    }

    // Write raw bytes
    if write_bytes("example_bytes.dat", &[72, 101, 108, 108, 111]).is_ok() {
        println!("Bytes written successfully.");
        let _ = fs::remove_file("example_bytes.dat");
    }

    // Write and append
    let log_path = "example_log.txt";
    if write_string(log_path, "First entry\n").is_ok() {
        if append_string(log_path, "Second entry\n").is_ok() {
            println!("Appended successfully.");
            if let Ok(content) = fs::read_to_string(log_path) {
                println!("Log contents:\n{}", content);
            }
        }
        let _ = fs::remove_file(log_path);
    }

    // Write multiple lines
    let list_path = "example_list.txt";
    if write_lines(list_path, &["Apple", "Banana", "Cherry"]).is_ok() {
        println!("Lines written successfully.");
        if let Ok(content) = fs::read_to_string(list_path) {
            println!("List contents:\n{}", content);
        }
        let _ = fs::remove_file(list_path);
    }

    // Buffered writing
    let buffered_path = "example_buffered.txt";
    if write_with_buffer(buffered_path, &["Rust ", "is ", "awesome!"]).is_ok() {
        println!("Buffered write successful.");
        if let Ok(content) = fs::read_to_string(buffered_path) {
            println!("Buffered contents: {}", content);
        }
        let _ = fs::remove_file(buffered_path);
    }
}
