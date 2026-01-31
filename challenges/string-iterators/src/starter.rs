/// Convert a string into a vector of its characters.
pub fn chars_to_vec(s: &str) -> Vec<char> {
    // TODO: Use .chars() and .collect()
    unimplemented!()
}

/// Split a string on whitespace and collect into owned strings.
pub fn words_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .split_whitespace() and collect
    unimplemented!()
}

/// Split a string on line endings and collect into owned strings.
pub fn lines_to_vec(s: &str) -> Vec<String> {
    // TODO: Use .lines() and collect
    unimplemented!()
}

/// Count the number of whitespace-separated words.
pub fn count_words(s: &str) -> usize {
    // TODO: Use .split_whitespace() and .count()
    unimplemented!()
}

/// Reverse the order of words in a string (keep words themselves intact).
pub fn reverse_words(s: &str) -> String {
    // TODO: Split into words, reverse the order, then join with spaces
    unimplemented!()
}

/// Capitalize the first letter of each word.
pub fn capitalize_words(s: &str) -> String {
    // TODO: Make the first character uppercase and rest lowercase
    unimplemented!()
}

pub fn main() {
    let text = "hello world";
    println!("Original: '{}'", text);
    println!("chars_to_vec: {:?}", chars_to_vec(text));
    println!("words_to_vec: {:?}", words_to_vec(text));
    println!("count_words: {}", count_words(text));
    println!("reverse_words: '{}'", reverse_words(text));
    println!("capitalize_words: '{}'", capitalize_words(text));

    let multiline = "line one\nline two\nline three";
    println!("\nMultiline text:");
    println!("{}", multiline);
    println!("lines_to_vec: {:?}", lines_to_vec(multiline));
}
