/// Check if a string starts with the given prefix.
pub fn has_prefix(s: &str, prefix: &str) -> bool {
    // TODO: Check for a prefix
    unimplemented!()
}

/// Check if a string ends with the given suffix.
pub fn has_suffix(s: &str, suffix: &str) -> bool {
    // TODO: Check for a suffix
    unimplemented!()
}

/// Find the first occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
pub fn find_first(s: &str, pattern: &str) -> Option<usize> {
    // TODO: Find the first occurrence
    unimplemented!()
}

/// Find the last occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
pub fn find_last(s: &str, pattern: &str) -> Option<usize> {
    // TODO: Find the last occurrence
    unimplemented!()
}

/// Count how many times a pattern appears in the string.
pub fn count_occurrences(s: &str, pattern: &str) -> usize {
    // TODO: Count pattern matches
    unimplemented!()
}

/// Find all byte indices where the pattern occurs.
pub fn find_all_indices(s: &str, pattern: &str) -> Vec<usize> {
    // TODO: Find all positions
    unimplemented!()
}

/// Extract text between the first occurrence of start and end markers.
/// Returns None if either marker is not found or if end comes before start.
pub fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    // TODO: Find the start and end markers, return text between them
    unimplemented!()
}

pub fn main() {
    let text = "hello world, hello universe";

    println!("Text: '{}'", text);
    println!();

    // Test your implementations
    println!("has_prefix('hello'): {}", has_prefix(text, "hello"));
    println!("has_suffix('universe'): {}", has_suffix(text, "universe"));
    println!("find_first('hello'): {:?}", find_first(text, "hello"));
    println!("find_last('hello'): {:?}", find_last(text, "hello"));
    println!("count_occurrences('hello'): {}", count_occurrences(text, "hello"));
    println!("find_all_indices('hello'): {:?}", find_all_indices(text, "hello"));

    let html = "<title>My Page</title>";
    println!();
    println!("HTML: '{}'", html);
    println!(
        "extract_between('<title>', '</title>'): {:?}",
        extract_between(html, "<title>", "</title>")
    );
}
