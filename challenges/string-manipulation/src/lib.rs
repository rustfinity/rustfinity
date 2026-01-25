/// Remove leading and trailing whitespace, then convert to lowercase.
///
/// # Examples
///
/// ```
/// use string_manipulation::clean_string;
///
/// assert_eq!(clean_string("  Hello World  "), "hello world");
/// assert_eq!(clean_string("RUST"), "rust");
/// ```
pub fn clean_string(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Check if the text contains the given word (case-insensitive).
///
/// # Examples
///
/// ```
/// use string_manipulation::contains_word;
///
/// assert!(contains_word("Rust is awesome", "AWESOME"));
/// assert!(!contains_word("Rust is awesome", "boring"));
/// ```
pub fn contains_word(text: &str, word: &str) -> bool {
    text.to_lowercase().contains(&word.to_lowercase())
}

/// Replace all occurrences of `from` with `to`.
///
/// # Examples
///
/// ```
/// use string_manipulation::replace_word;
///
/// assert_eq!(replace_word("hello world", "world", "Rust"), "hello Rust");
/// ```
pub fn replace_word(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

/// Split the string by the delimiter and trim each part.
///
/// # Examples
///
/// ```
/// use string_manipulation::split_and_trim;
///
/// assert_eq!(
///     split_and_trim("apple , banana , cherry", ','),
///     vec!["apple", "banana", "cherry"]
/// );
/// ```
pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter)
        .map(|part| part.trim().to_string())
        .collect()
}

/// Replace all sequences of whitespace with a single space, and trim.
///
/// # Examples
///
/// ```
/// use string_manipulation::normalize_whitespace;
///
/// assert_eq!(normalize_whitespace("  hello    world  "), "hello world");
/// ```
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn main() {
    let messy = "  Hello, World!  ";
    println!("Original: '{}'", messy);
    println!("Cleaned: '{}'", clean_string(messy));

    let text = "Rust is a systems programming language";
    println!("\nText: '{}'", text);
    println!("Contains 'SYSTEMS': {}", contains_word(text, "SYSTEMS"));
    println!("Contains 'Java': {}", contains_word(text, "Java"));

    let original = "hello world world";
    println!("\nOriginal: '{}'", original);
    println!("Replace 'world' with 'Rust': '{}'", replace_word(original, "world", "Rust"));

    let csv = "  apple ,  banana  , cherry ";
    println!("\nCSV: '{}'", csv);
    println!("Split and trim: {:?}", split_and_trim(csv, ','));

    let spaced = "  too   many    spaces   here  ";
    println!("\nSpaced: '{}'", spaced);
    println!("Normalized: '{}'", normalize_whitespace(spaced));
}
