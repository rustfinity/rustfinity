use string_basics::*;

#[test]
fn test_to_owned_string_simple() {
    assert_eq!(to_owned_string("hello"), String::from("hello"));
}

#[test]
fn test_to_owned_string_empty() {
    assert_eq!(to_owned_string(""), String::new());
}

#[test]
fn test_to_owned_string_unicode() {
    assert_eq!(to_owned_string("caf\u{00E9}"), String::from("caf\u{00E9}"));
}

#[test]
fn test_count_chars_ascii() {
    assert_eq!(count_chars("hello"), 5);
}

#[test]
fn test_count_chars_empty() {
    assert_eq!(count_chars(""), 0);
}

#[test]
fn test_count_chars_unicode() {
    // "\u{1F600}" is a smiley face emoji - 1 Unicode character
    assert_eq!(count_chars("h\u{1F600}i\u{1F600}"), 4);
}

#[test]
fn test_count_chars_accented() {
    // "caf\u{00E9}" has 4 characters (e with accent is one character)
    assert_eq!(count_chars("caf\u{00E9}"), 4);
}

#[test]
fn test_count_bytes_ascii() {
    assert_eq!(count_bytes("hello"), 5);
}

#[test]
fn test_count_bytes_empty() {
    assert_eq!(count_bytes(""), 0);
}

#[test]
fn test_count_bytes_unicode() {
    // Each emoji takes 4 bytes in UTF-8
    assert_eq!(count_bytes("h\u{1F600}i\u{1F600}"), 10); // 'h' (1) + emoji (4) + 'i' (1) + emoji (4)
}

#[test]
fn test_count_bytes_accented() {
    // "caf\u{00E9}" - 'c', 'a', 'f' are 1 byte each, '\u{00E9}' (e with accent) is 2 bytes
    assert_eq!(count_bytes("caf\u{00E9}"), 5);
}

#[test]
fn test_is_ascii_only_true() {
    assert!(is_ascii_only("hello world"));
    assert!(is_ascii_only("Hello, World! 123"));
    assert!(is_ascii_only(""));
}

#[test]
fn test_is_ascii_only_false() {
    assert!(!is_ascii_only("caf\u{00E9}"));
    assert!(!is_ascii_only("hello \u{1F600}"));
    assert!(!is_ascii_only("\u{4E2D}\u{6587}")); // Chinese characters
}

#[test]
fn test_first_char_ascii() {
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char("world"), Some('w'));
}

#[test]
fn test_first_char_empty() {
    assert_eq!(first_char(""), None);
}

#[test]
fn test_first_char_unicode() {
    assert_eq!(first_char("caf\u{00E9}"), Some('c'));
    assert_eq!(first_char("\u{1F600}world"), Some('\u{1F600}'));
}

#[test]
fn test_first_char_single() {
    assert_eq!(first_char("a"), Some('a'));
    assert_eq!(first_char("\u{1F600}"), Some('\u{1F600}'));
}
