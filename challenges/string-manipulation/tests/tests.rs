use string_manipulation::*;

// Tests for clean_string

#[test]
fn test_clean_string_basic() {
    assert_eq!(clean_string("  Hello World  "), "hello world");
}

#[test]
fn test_clean_string_already_clean() {
    assert_eq!(clean_string("hello"), "hello");
}

#[test]
fn test_clean_string_uppercase() {
    assert_eq!(clean_string("RUST IS AWESOME"), "rust is awesome");
}

#[test]
fn test_clean_string_mixed_whitespace() {
    assert_eq!(clean_string("\t  Hello \n"), "hello");
}

#[test]
fn test_clean_string_empty() {
    assert_eq!(clean_string(""), "");
    assert_eq!(clean_string("   "), "");
}

// Tests for contains_word

#[test]
fn test_contains_word_exact() {
    assert!(contains_word("hello world", "world"));
}

#[test]
fn test_contains_word_case_insensitive() {
    assert!(contains_word("Rust is awesome", "AWESOME"));
    assert!(contains_word("RUST IS AWESOME", "rust"));
    assert!(contains_word("rust is awesome", "Is"));
}

#[test]
fn test_contains_word_not_found() {
    assert!(!contains_word("Rust is awesome", "boring"));
    assert!(!contains_word("hello", "world"));
}

#[test]
fn test_contains_word_empty() {
    assert!(contains_word("hello", "")); // empty string is contained in any string
    assert!(!contains_word("", "hello"));
}

#[test]
fn test_contains_word_partial() {
    assert!(contains_word("hello world", "ello"));
}

// Tests for replace_word

#[test]
fn test_replace_word_single() {
    assert_eq!(replace_word("hello world", "world", "Rust"), "hello Rust");
}

#[test]
fn test_replace_word_multiple() {
    assert_eq!(replace_word("world world world", "world", "Rust"), "Rust Rust Rust");
}

#[test]
fn test_replace_word_not_found() {
    assert_eq!(replace_word("hello world", "foo", "bar"), "hello world");
}

#[test]
fn test_replace_word_empty_replacement() {
    assert_eq!(replace_word("hello world", "world", ""), "hello ");
}

#[test]
fn test_replace_word_longer_replacement() {
    assert_eq!(replace_word("hi", "hi", "hello there"), "hello there");
}

// Tests for split_and_trim

#[test]
fn test_split_and_trim_basic() {
    assert_eq!(
        split_and_trim("apple,banana,cherry", ','),
        vec!["apple", "banana", "cherry"]
    );
}

#[test]
fn test_split_and_trim_with_spaces() {
    assert_eq!(
        split_and_trim("apple , banana , cherry", ','),
        vec!["apple", "banana", "cherry"]
    );
}

#[test]
fn test_split_and_trim_extra_whitespace() {
    assert_eq!(
        split_and_trim("  apple  ,  banana  ,  cherry  ", ','),
        vec!["apple", "banana", "cherry"]
    );
}

#[test]
fn test_split_and_trim_single_element() {
    assert_eq!(split_and_trim("apple", ','), vec!["apple"]);
}

#[test]
fn test_split_and_trim_empty_parts() {
    assert_eq!(split_and_trim("a,,b", ','), vec!["a", "", "b"]);
}

#[test]
fn test_split_and_trim_different_delimiter() {
    assert_eq!(
        split_and_trim("one:two:three", ':'),
        vec!["one", "two", "three"]
    );
}

// Tests for normalize_whitespace

#[test]
fn test_normalize_whitespace_multiple_spaces() {
    assert_eq!(normalize_whitespace("hello    world"), "hello world");
}

#[test]
fn test_normalize_whitespace_leading_trailing() {
    assert_eq!(normalize_whitespace("  hello world  "), "hello world");
}

#[test]
fn test_normalize_whitespace_mixed() {
    assert_eq!(normalize_whitespace("  hello    world  "), "hello world");
}

#[test]
fn test_normalize_whitespace_tabs_newlines() {
    assert_eq!(normalize_whitespace("hello\t\tworld\n\nfoo"), "hello world foo");
}

#[test]
fn test_normalize_whitespace_already_normal() {
    assert_eq!(normalize_whitespace("hello world"), "hello world");
}

#[test]
fn test_normalize_whitespace_empty() {
    assert_eq!(normalize_whitespace(""), "");
    assert_eq!(normalize_whitespace("   "), "");
}

#[test]
fn test_normalize_whitespace_single_word() {
    assert_eq!(normalize_whitespace("  hello  "), "hello");
}
