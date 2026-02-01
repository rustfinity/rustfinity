use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

/// Read the entire contents of a file into a String.
pub fn read_entire_file(path: &str) -> Result<String, io::Error> {
    // TODO: Read the entire file contents into a String
    unimplemented!()
}

/// Count the number of lines in a file.
pub fn count_lines(path: &str) -> Result<usize, io::Error> {
    // TODO: Open the file, create a BufReader, and count the lines
    unimplemented!()
}

/// Count the number of words in a file.
/// Words are defined as whitespace-separated sequences of characters.
pub fn count_words(path: &str) -> Result<usize, io::Error> {
    // TODO: Read the file and count whitespace-separated words
    unimplemented!()
}

/// Read all lines from a file into a Vec, with trailing newlines removed.
pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    // TODO: Open the file, create a BufReader, and collect all lines
    unimplemented!()
}

/// Read only the first n lines from a file.
pub fn first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error> {
    // TODO: Read only the first n lines from the file
    unimplemented!()
}

pub fn main() {
    // Example: Create a sample file and read it
    let sample_content = "Hello World\nThis is a test\nRust is great\n";
    let path = "sample.txt";

    // Write sample file
    if fs::write(path, sample_content).is_ok() {
        println!("Sample file created.");

        // Read entire file
        if let Ok(contents) = read_entire_file(path) {
            println!("File contents:\n{}", contents);
        }

        // Count lines
        if let Ok(count) = count_lines(path) {
            println!("Line count: {}", count);
        }

        // Count words
        if let Ok(count) = count_words(path) {
            println!("Word count: {}", count);
        }

        // Read lines into Vec
        if let Ok(lines) = read_lines(path) {
            println!("Lines: {:?}", lines);
        }

        // Read first 2 lines
        if let Ok(lines) = first_n_lines(path, 2) {
            println!("First 2 lines: {:?}", lines);
        }

        // Clean up
        let _ = fs::remove_file(path);
    }
}
