/// Convert a string into a vector of its characters.
///
/// # Examples
///
/// ```
/// use string_iterators::chars_to_vec;
///
/// assert_eq!(chars_to_vec("hi"), vec!['h', 'i']);
/// assert_eq!(chars_to_vec(""), Vec::<char>::new());
/// ```
pub fn chars_to_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

/// Split a string on whitespace and collect into owned strings.
///
/// # Examples
///
/// ```
/// use string_iterators::words_to_vec;
///
/// assert_eq!(words_to_vec("hello world"), vec!["hello", "world"]);
/// assert_eq!(words_to_vec("  spaces  "), vec!["spaces"]);
/// ```
pub fn words_to_vec(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| w.to_string()).collect()
}

/// Split a string on line endings and collect into owned strings.
///
/// # Examples
///
/// ```
/// use string_iterators::lines_to_vec;
///
/// assert_eq!(lines_to_vec("line1\nline2"), vec!["line1", "line2"]);
/// assert_eq!(lines_to_vec("single"), vec!["single"]);
/// ```
pub fn lines_to_vec(s: &str) -> Vec<String> {
    s.lines().map(|line| line.to_string()).collect()
}

/// Count the number of whitespace-separated words.
///
/// # Examples
///
/// ```
/// use string_iterators::count_words;
///
/// assert_eq!(count_words("one two three"), 3);
/// assert_eq!(count_words(""), 0);
/// ```
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Reverse the order of words in a string (keep words themselves intact).
///
/// # Examples
///
/// ```
/// use string_iterators::reverse_words;
///
/// assert_eq!(reverse_words("hello world"), "world hello");
/// assert_eq!(reverse_words("one"), "one");
/// ```
pub fn reverse_words(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.into_iter().rev().collect::<Vec<_>>().join(" ")
}

/// Capitalize the first letter of each word.
///
/// # Examples
///
/// ```
/// use string_iterators::capitalize_words;
///
/// assert_eq!(capitalize_words("hello world"), "Hello World");
/// assert_eq!(capitalize_words("rust"), "Rust");
/// ```
pub fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().to_string() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
