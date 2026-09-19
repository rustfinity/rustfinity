/// Count the number of Unicode characters in a string.
///
/// This counts Unicode scalar values, not bytes or grapheme clusters.
///
/// # Examples
///
/// ```
/// use unicode_operations::char_count;
///
/// assert_eq!(char_count("Hello"), 5);
/// assert_eq!(char_count("Привет"), 6);  // Russian "Hello"
/// assert_eq!(char_count("你好"), 2);     // Chinese "Hello"
/// ```
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Count the number of bytes in the UTF-8 encoding of a string.
///
/// # Examples
///
/// ```
/// use unicode_operations::byte_count;
///
/// assert_eq!(byte_count("Hello"), 5);    // ASCII: 1 byte each
/// assert_eq!(byte_count("Привет"), 12);  // Cyrillic: 2 bytes each
/// assert_eq!(byte_count("你好"), 6);      // Chinese: 3 bytes each
/// ```
pub fn byte_count(s: &str) -> usize {
    s.len()
}

/// Extract a substring by character indices (not byte indices).
///
/// Returns `None` if indices are out of bounds or if start > end.
///
/// # Examples
///
/// ```
/// use unicode_operations::safe_substring;
///
/// assert_eq!(safe_substring("Hello", 0, 3), Some("Hel".to_string()));
/// assert_eq!(safe_substring("Привет", 0, 2), Some("Пр".to_string()));
/// assert_eq!(safe_substring("Hello", 0, 10), None);  // Out of bounds
/// ```
pub fn safe_substring(s: &str, start: usize, end: usize) -> Option<String> {
    if start > end {
        return None;
    }

    let chars: Vec<char> = s.chars().collect();

    if end > chars.len() {
        return None;
    }

    Some(chars[start..end].iter().collect())
}

/// Get the character at a specific index (by character position, not byte position).
///
/// Returns `None` if the index is out of bounds.
///
/// # Examples
///
/// ```
/// use unicode_operations::char_at;
///
/// assert_eq!(char_at("Hello", 0), Some('H'));
/// assert_eq!(char_at("Привет", 2), Some('и'));
/// assert_eq!(char_at("Hello", 10), None);
/// ```
pub fn char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

/// Check if a string contains exactly one Unicode character.
///
/// # Examples
///
/// ```
/// use unicode_operations::is_single_char;
///
/// assert_eq!(is_single_char("a"), true);
/// assert_eq!(is_single_char("好"), true);
/// assert_eq!(is_single_char("ab"), false);
/// assert_eq!(is_single_char(""), false);
/// ```
pub fn is_single_char(s: &str) -> bool {
    s.chars().count() == 1
}

pub fn main() {
    // Demonstrate char_count vs byte_count
    println!("=== Character vs Byte Counts ===");
    let examples = ["Hello", "Привет", "你好", "🎉", "👨‍👩‍👧"];

    for text in &examples {
        println!(
            "\"{}\" - chars: {}, bytes: {}",
            text,
            char_count(text),
            byte_count(text)
        );
    }

    // Demonstrate safe_substring
    println!("\n=== Safe Substring ===");
    let text = "Привет мир";
    println!("Original: \"{}\"", text);
    println!("Substring [0..3]: {:?}", safe_substring(text, 0, 3));
    println!("Substring [0..100]: {:?}", safe_substring(text, 0, 100));

    // Demonstrate char_at
    println!("\n=== Character At Index ===");
    let text = "Hello, 世界!";
    for i in 0..=9 {
        println!("char_at(\"{}\", {}) = {:?}", text, i, char_at(text, i));
    }

    // Demonstrate is_single_char
    println!("\n=== Is Single Character ===");
    for s in &["a", "ab", "", "好", "🎉"] {
        println!("is_single_char(\"{}\") = {}", s, is_single_char(s));
    }
}
