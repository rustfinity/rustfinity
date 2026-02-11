/// Check if a string starts with the given prefix.
///
/// # Examples
///
/// ```
/// use string_patterns::has_prefix;
///
/// assert!(has_prefix("hello world", "hello"));
/// assert!(!has_prefix("hello world", "world"));
/// ```
pub fn has_prefix(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Check if a string ends with the given suffix.
///
/// # Examples
///
/// ```
/// use string_patterns::has_suffix;
///
/// assert!(has_suffix("hello world", "world"));
/// assert!(!has_suffix("hello world", "hello"));
/// ```
pub fn has_suffix(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Find the first occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
///
/// # Examples
///
/// ```
/// use string_patterns::find_first;
///
/// assert_eq!(find_first("hello hello", "hello"), Some(0));
/// assert_eq!(find_first("hello world", "xyz"), None);
/// ```
pub fn find_first(s: &str, pattern: &str) -> Option<usize> {
    s.find(pattern)
}

/// Find the last occurrence of a pattern in the string.
/// Returns the byte index of the match, or None if not found.
///
/// # Examples
///
/// ```
/// use string_patterns::find_last;
///
/// assert_eq!(find_last("hello hello", "hello"), Some(6));
/// assert_eq!(find_last("hello world", "xyz"), None);
/// ```
pub fn find_last(s: &str, pattern: &str) -> Option<usize> {
    s.rfind(pattern)
}

/// Count how many times a pattern appears in the string.
///
/// # Examples
///
/// ```
/// use string_patterns::count_occurrences;
///
/// assert_eq!(count_occurrences("abababab", "ab"), 4);
/// assert_eq!(count_occurrences("hello world", "o"), 2);
/// ```
pub fn count_occurrences(s: &str, pattern: &str) -> usize {
    s.matches(pattern).count()
}

/// Find all byte indices where the pattern occurs.
///
/// # Examples
///
/// ```
/// use string_patterns::find_all_indices;
///
/// assert_eq!(find_all_indices("abcabc", "abc"), vec![0, 3]);
/// assert_eq!(find_all_indices("hello", "l"), vec![2, 3]);
/// ```
pub fn find_all_indices(s: &str, pattern: &str) -> Vec<usize> {
    s.match_indices(pattern).map(|(idx, _)| idx).collect()
}

/// Extract text between the first occurrence of start and end markers.
/// Returns None if either marker is not found or if end comes before start.
///
/// # Examples
///
/// ```
/// use string_patterns::extract_between;
///
/// assert_eq!(
///     extract_between("<tag>content</tag>", "<tag>", "</tag>"),
///     Some("content".to_string())
/// );
/// assert_eq!(extract_between("no markers", "[", "]"), None);
/// ```
pub fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let start_idx = s.find(start)?;
    let content_start = start_idx + start.len();
    let remaining = &s[content_start..];
    let end_idx = remaining.find(end)?;
    Some(remaining[..end_idx].to_string())
}

pub fn main() {
    let text = "hello world, hello universe";

    println!("Text: '{}'", text);
    println!();

    // Prefix/suffix checks
    println!("has_prefix('hello'): {}", has_prefix(text, "hello"));
    println!("has_prefix('world'): {}", has_prefix(text, "world"));
    println!("has_suffix('universe'): {}", has_suffix(text, "universe"));
    println!("has_suffix('world'): {}", has_suffix(text, "world"));
    println!();

    // Finding patterns
    println!("find_first('hello'): {:?}", find_first(text, "hello"));
    println!("find_last('hello'): {:?}", find_last(text, "hello"));
    println!("find_first('xyz'): {:?}", find_first(text, "xyz"));
    println!();

    // Counting
    println!("count_occurrences('hello'): {}", count_occurrences(text, "hello"));
    println!("count_occurrences('o'): {}", count_occurrences(text, "o"));
    println!();

    // All indices
    println!("find_all_indices('hello'): {:?}", find_all_indices(text, "hello"));
    println!("find_all_indices('o'): {:?}", find_all_indices(text, "o"));
    println!();

    // Extract between markers
    let html = "<title>My Page</title>";
    println!("HTML: '{}'", html);
    println!(
        "extract_between('<title>', '</title>'): {:?}",
        extract_between(html, "<title>", "</title>")
    );
}
