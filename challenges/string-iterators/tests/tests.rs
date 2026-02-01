use string_iterators::*;

// ============== chars_to_vec tests ==============

#[test]
fn chars_to_vec_simple() {
    assert_eq!(chars_to_vec("hi"), vec!['h', 'i']);
}

#[test]
fn chars_to_vec_empty() {
    assert_eq!(chars_to_vec(""), Vec::<char>::new());
}

#[test]
fn chars_to_vec_with_spaces() {
    assert_eq!(chars_to_vec("a b"), vec!['a', ' ', 'b']);
}

#[test]
fn chars_to_vec_unicode() {
    assert_eq!(chars_to_vec("日本"), vec!['日', '本']);
}

#[test]
fn chars_to_vec_emoji() {
    assert_eq!(chars_to_vec("Hi!"), vec!['H', 'i', '!']);
}

// ============== words_to_vec tests ==============

#[test]
fn words_to_vec_simple() {
    assert_eq!(words_to_vec("hello world"), vec!["hello", "world"]);
}

#[test]
fn words_to_vec_extra_spaces() {
    assert_eq!(words_to_vec("  spaces  everywhere  "), vec!["spaces", "everywhere"]);
}

#[test]
fn words_to_vec_single_word() {
    assert_eq!(words_to_vec("word"), vec!["word"]);
}

#[test]
fn words_to_vec_empty() {
    assert_eq!(words_to_vec(""), Vec::<String>::new());
}

#[test]
fn words_to_vec_only_spaces() {
    assert_eq!(words_to_vec("   "), Vec::<String>::new());
}

#[test]
fn words_to_vec_tabs_and_newlines() {
    assert_eq!(words_to_vec("a\tb\nc"), vec!["a", "b", "c"]);
}

// ============== lines_to_vec tests ==============

#[test]
fn lines_to_vec_multiple() {
    assert_eq!(lines_to_vec("line1\nline2\nline3"), vec!["line1", "line2", "line3"]);
}

#[test]
fn lines_to_vec_single() {
    assert_eq!(lines_to_vec("single"), vec!["single"]);
}

#[test]
fn lines_to_vec_empty() {
    assert_eq!(lines_to_vec(""), Vec::<String>::new());
}

#[test]
fn lines_to_vec_windows_line_endings() {
    assert_eq!(lines_to_vec("line1\r\nline2"), vec!["line1", "line2"]);
}

#[test]
fn lines_to_vec_empty_lines() {
    assert_eq!(lines_to_vec("a\n\nb"), vec!["a", "", "b"]);
}

#[test]
fn lines_to_vec_trailing_newline() {
    // Note: .lines() does not include empty trailing line
    assert_eq!(lines_to_vec("a\nb\n"), vec!["a", "b"]);
}

// ============== count_words tests ==============

#[test]
fn count_words_three() {
    assert_eq!(count_words("one two three"), 3);
}

#[test]
fn count_words_empty() {
    assert_eq!(count_words(""), 0);
}

#[test]
fn count_words_only_whitespace() {
    assert_eq!(count_words("   "), 0);
}

#[test]
fn count_words_single() {
    assert_eq!(count_words("word"), 1);
}

#[test]
fn count_words_with_tabs() {
    assert_eq!(count_words("a\tb\tc"), 3);
}

#[test]
fn count_words_mixed_whitespace() {
    assert_eq!(count_words("  hello \t world \n test  "), 3);
}

// ============== reverse_words tests ==============

#[test]
fn reverse_words_two() {
    assert_eq!(reverse_words("hello world"), "world hello");
}

#[test]
fn reverse_words_three() {
    assert_eq!(reverse_words("one two three"), "three two one");
}

#[test]
fn reverse_words_single() {
    assert_eq!(reverse_words("single"), "single");
}

#[test]
fn reverse_words_empty() {
    assert_eq!(reverse_words(""), "");
}

#[test]
fn reverse_words_with_extra_spaces() {
    // Extra spaces are normalized
    assert_eq!(reverse_words("  hello   world  "), "world hello");
}

#[test]
fn reverse_words_sentence() {
    assert_eq!(reverse_words("I love Rust"), "Rust love I");
}

// ============== capitalize_words tests ==============

#[test]
fn capitalize_words_simple() {
    assert_eq!(capitalize_words("hello world"), "Hello World");
}

#[test]
fn capitalize_words_already_caps() {
    assert_eq!(capitalize_words("HELLO WORLD"), "Hello World");
}

#[test]
fn capitalize_words_mixed_case() {
    assert_eq!(capitalize_words("hElLo WoRlD"), "Hello World");
}

#[test]
fn capitalize_words_single() {
    assert_eq!(capitalize_words("rust"), "Rust");
}

#[test]
fn capitalize_words_empty() {
    assert_eq!(capitalize_words(""), "");
}

#[test]
fn capitalize_words_with_numbers() {
    assert_eq!(capitalize_words("hello 123 world"), "Hello 123 World");
}

#[test]
fn capitalize_words_extra_spaces() {
    // Extra spaces are normalized
    assert_eq!(capitalize_words("  hello   world  "), "Hello World");
}

#[test]
fn capitalize_words_single_letters() {
    assert_eq!(capitalize_words("a b c"), "A B C");
}
