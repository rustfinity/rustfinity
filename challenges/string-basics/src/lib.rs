/// Convert a string slice to an owned String.
///
/// # Examples
///
/// ```
/// use string_basics::to_owned_string;
///
/// let owned = to_owned_string("hello");
/// assert_eq!(owned, String::from("hello"));
/// ```
pub fn to_owned_string(s: &str) -> String {
    s.to_string()
}

/// Count the number of Unicode characters in a string.
///
/// # Examples
///
/// ```
/// use string_basics::count_chars;
///
/// assert_eq!(count_chars("hello"), 5);
/// assert_eq!(count_chars("caf\u{00E9}"), 4); // 4 Unicode scalar values
/// ```
pub fn count_chars(s: &str) -> usize {
    s.chars().count()
}

/// Count the number of bytes in a string.
///
/// # Examples
///
/// ```
/// use string_basics::count_bytes;
///
/// assert_eq!(count_bytes("hello"), 5);
/// assert_eq!(count_bytes("caf\u{00E9}"), 5); // 'c', 'a', 'f' are 1 byte each, accented 'e' is 2 bytes
/// ```
pub fn count_bytes(s: &str) -> usize {
    s.len()
}

/// Check if a string contains only ASCII characters.
///
/// # Examples
///
/// ```
/// use string_basics::is_ascii_only;
///
/// assert!(is_ascii_only("hello"));
/// assert!(!is_ascii_only("caf\u{00E9}")); // accented 'e' is not ASCII
/// ```
pub fn is_ascii_only(s: &str) -> bool {
    s.is_ascii()
}

/// Return the first character of a string, or None if the string is empty.
///
/// # Examples
///
/// ```
/// use string_basics::first_char;
///
/// assert_eq!(first_char("hello"), Some('h'));
/// assert_eq!(first_char(""), None);
/// ```
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn main() {
    let greeting = "Hello, world!";

    println!("Original: {}", greeting);
    println!("As owned String: {}", to_owned_string(greeting));
    println!("Character count: {}", count_chars(greeting));
    println!("Byte count: {}", count_bytes(greeting));
    println!("Is ASCII only: {}", is_ascii_only(greeting));
    println!("First character: {:?}", first_char(greeting));
}
