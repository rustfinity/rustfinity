use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

/// Read the entire contents of a file into a String.
///
/// # Examples
///
/// ```no_run
/// use reading_files::read_entire_file;
///
/// let contents = read_entire_file("example.txt").unwrap();
/// println!("{}", contents);
/// ```
pub fn read_entire_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Count the number of lines in a file.
///
/// # Examples
///
/// ```no_run
/// use reading_files::count_lines;
///
/// let count = count_lines("example.txt").unwrap();
/// println!("File has {} lines", count);
/// ```
pub fn count_lines(path: &str) -> Result<usize, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let _ = line?;
        count += 1;
    }
    Ok(count)
}

/// Count the number of words in a file.
/// Words are defined as whitespace-separated sequences of characters.
///
/// # Examples
///
/// ```no_run
/// use reading_files::count_words;
///
/// let count = count_words("example.txt").unwrap();
/// println!("File has {} words", count);
/// ```
pub fn count_words(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.split_whitespace().count())
}

/// Read all lines from a file into a Vec, with trailing newlines removed.
///
/// # Examples
///
/// ```no_run
/// use reading_files::read_lines;
///
/// let lines = read_lines("example.txt").unwrap();
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Read only the first n lines from a file.
///
/// # Examples
///
/// ```no_run
/// use reading_files::first_n_lines;
///
/// let lines = first_n_lines("example.txt", 5).unwrap();
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
pub fn first_n_lines(path: &str, n: usize) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().take(n).collect()
}

pub fn main() {
    // Example: Create a sample file and read it
    use std::fs;

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
