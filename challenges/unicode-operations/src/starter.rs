/// Count the number of Unicode characters in a string.
///
/// This counts Unicode scalar values, not bytes or grapheme clusters.
pub fn char_count(s: &str) -> usize {
    // TODO: Use .chars() to iterate over Unicode characters
    // Hint: .len() returns bytes, not characters!
    unimplemented!()
}

/// Count the number of bytes in the UTF-8 encoding of a string.
pub fn byte_count(s: &str) -> usize {
    // TODO: Return the byte length of the string
    unimplemented!()
}

/// Extract a substring by character indices (not byte indices).
///
/// Returns `None` if indices are out of bounds or if start > end.
pub fn safe_substring(s: &str, start: usize, end: usize) -> Option<String> {
    // TODO: Implement safe substring extraction
    // 1. Check if start > end (return None)
    // 2. Collect characters into a Vec<char>
    // 3. Check if end is within bounds
    // 4. Extract and return the substring
    unimplemented!()
}

/// Get the character at a specific index (by character position, not byte position).
///
/// Returns `None` if the index is out of bounds.
pub fn char_at(s: &str, index: usize) -> Option<char> {
    // TODO: Use .chars().nth(index) to get the character at position
    unimplemented!()
}

/// Check if a string contains exactly one Unicode character.
pub fn is_single_char(s: &str) -> bool {
    // TODO: Check if the character count equals 1
    unimplemented!()
}

pub fn main() {
    // Example usage
    let examples = ["Hello", "Привет", "你好", "🎉"];

    for text in &examples {
        println!(
            "\"{}\" - chars: {}, bytes: {}",
            text,
            char_count(text),
            byte_count(text)
        );
    }

    // Safe substring demonstration
    let text = "Hello, 世界!";
    println!("\nSubstring [0..5]: {:?}", safe_substring(text, 0, 5));
    println!("Char at index 7: {:?}", char_at(text, 7));
    println!("Is '好' single char? {}", is_single_char("好"));
}
