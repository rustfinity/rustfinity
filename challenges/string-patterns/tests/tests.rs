use string_patterns::*;

// ============================================================================
// has_prefix tests
// ============================================================================

#[test]
fn test_has_prefix_basic() {
    assert!(has_prefix("hello world", "hello"));
    assert!(has_prefix("hello world", "hello "));
    assert!(has_prefix("hello world", "hello world"));
}

#[test]
fn test_has_prefix_negative() {
    assert!(!has_prefix("hello world", "world"));
    assert!(!has_prefix("hello world", "ello"));
    assert!(!has_prefix("hello", "hello world"));
}

#[test]
fn test_has_prefix_empty() {
    assert!(has_prefix("hello", ""));
    assert!(has_prefix("", ""));
    assert!(!has_prefix("", "hello"));
}

#[test]
fn test_has_prefix_single_char() {
    assert!(has_prefix("hello", "h"));
    assert!(!has_prefix("hello", "e"));
}

#[test]
fn test_has_prefix_unicode() {
    assert!(has_prefix("café", "caf"));
    assert!(has_prefix("日本語", "日本"));
    assert!(!has_prefix("日本語", "語"));
}

// ============================================================================
// has_suffix tests
// ============================================================================

#[test]
fn test_has_suffix_basic() {
    assert!(has_suffix("hello world", "world"));
    assert!(has_suffix("hello world", " world"));
    assert!(has_suffix("hello world", "hello world"));
}

#[test]
fn test_has_suffix_negative() {
    assert!(!has_suffix("hello world", "hello"));
    assert!(!has_suffix("hello world", "worl"));
    assert!(!has_suffix("world", "hello world"));
}

#[test]
fn test_has_suffix_empty() {
    assert!(has_suffix("hello", ""));
    assert!(has_suffix("", ""));
    assert!(!has_suffix("", "hello"));
}

#[test]
fn test_has_suffix_file_extension() {
    assert!(has_suffix("main.rs", ".rs"));
    assert!(has_suffix("image.png", ".png"));
    assert!(!has_suffix("main.rs", ".txt"));
}

#[test]
fn test_has_suffix_unicode() {
    assert!(has_suffix("café", "fé"));
    assert!(has_suffix("日本語", "本語"));
    assert!(!has_suffix("日本語", "日"));
}

// ============================================================================
// find_first tests
// ============================================================================

#[test]
fn test_find_first_basic() {
    assert_eq!(find_first("hello world", "hello"), Some(0));
    assert_eq!(find_first("hello world", "world"), Some(6));
    assert_eq!(find_first("hello world", " "), Some(5));
}

#[test]
fn test_find_first_multiple_occurrences() {
    assert_eq!(find_first("hello hello", "hello"), Some(0));
    assert_eq!(find_first("abcabc", "abc"), Some(0));
    assert_eq!(find_first("abcabc", "bc"), Some(1));
}

#[test]
fn test_find_first_not_found() {
    assert_eq!(find_first("hello world", "xyz"), None);
    assert_eq!(find_first("hello", "hello world"), None);
    assert_eq!(find_first("", "hello"), None);
}

#[test]
fn test_find_first_empty_pattern() {
    assert_eq!(find_first("hello", ""), Some(0));
    assert_eq!(find_first("", ""), Some(0));
}

#[test]
fn test_find_first_unicode() {
    assert_eq!(find_first("café", "é"), Some(3)); // byte index
    assert_eq!(find_first("日本語", "本"), Some(3)); // byte index
}

#[test]
fn test_find_first_single_char() {
    assert_eq!(find_first("hello", "l"), Some(2));
    assert_eq!(find_first("hello", "o"), Some(4));
}

// ============================================================================
// find_last tests
// ============================================================================

#[test]
fn test_find_last_basic() {
    assert_eq!(find_last("hello world", "o"), Some(7));
    assert_eq!(find_last("hello world", "l"), Some(9));
}

#[test]
fn test_find_last_multiple_occurrences() {
    assert_eq!(find_last("hello hello", "hello"), Some(6));
    assert_eq!(find_last("abcabc", "abc"), Some(3));
    assert_eq!(find_last("abcabc", "bc"), Some(4));
}

#[test]
fn test_find_last_single_occurrence() {
    assert_eq!(find_last("hello world", "world"), Some(6));
    assert_eq!(find_last("unique", "que"), Some(3));
}

#[test]
fn test_find_last_not_found() {
    assert_eq!(find_last("hello world", "xyz"), None);
    assert_eq!(find_last("", "hello"), None);
}

#[test]
fn test_find_last_empty_pattern() {
    assert_eq!(find_last("hello", ""), Some(5)); // at end
    assert_eq!(find_last("", ""), Some(0));
}

#[test]
fn test_find_last_unicode() {
    assert_eq!(find_last("caféé", "é"), Some(5)); // last é
    assert_eq!(find_last("日本語日本", "本"), Some(12)); // byte index (each char is 3 bytes)
}

// ============================================================================
// count_occurrences tests
// ============================================================================

#[test]
fn test_count_occurrences_basic() {
    assert_eq!(count_occurrences("hello world", "o"), 2);
    assert_eq!(count_occurrences("hello world", "l"), 3);
    assert_eq!(count_occurrences("hello world", "hello"), 1);
}

#[test]
fn test_count_occurrences_multiple() {
    assert_eq!(count_occurrences("abababab", "ab"), 4);
    assert_eq!(count_occurrences("aaaa", "a"), 4);
    assert_eq!(count_occurrences("aaa", "aa"), 1); // non-overlapping
}

#[test]
fn test_count_occurrences_none() {
    assert_eq!(count_occurrences("hello world", "xyz"), 0);
    assert_eq!(count_occurrences("", "hello"), 0);
}

#[test]
fn test_count_occurrences_empty_pattern() {
    // Empty pattern matches at every position including start and end
    assert_eq!(count_occurrences("hi", ""), 3); // before h, between h and i, after i
    assert_eq!(count_occurrences("", ""), 1);
}

#[test]
fn test_count_occurrences_unicode() {
    assert_eq!(count_occurrences("café café café", "café"), 3);
    assert_eq!(count_occurrences("日本語日本語", "日本"), 2);
}

#[test]
fn test_count_occurrences_whole_string() {
    assert_eq!(count_occurrences("hello", "hello"), 1);
    assert_eq!(count_occurrences("", ""), 1);
}

// ============================================================================
// find_all_indices tests
// ============================================================================

#[test]
fn test_find_all_indices_basic() {
    assert_eq!(find_all_indices("abcabc", "abc"), vec![0, 3]);
    assert_eq!(find_all_indices("hello", "l"), vec![2, 3]);
}

#[test]
fn test_find_all_indices_multiple() {
    assert_eq!(find_all_indices("aaaa", "a"), vec![0, 1, 2, 3]);
    assert_eq!(find_all_indices("abababab", "ab"), vec![0, 2, 4, 6]);
}

#[test]
fn test_find_all_indices_none() {
    assert_eq!(find_all_indices("hello", "xyz"), vec![]);
    assert_eq!(find_all_indices("", "hello"), vec![]);
}

#[test]
fn test_find_all_indices_single_match() {
    assert_eq!(find_all_indices("hello world", "world"), vec![6]);
    assert_eq!(find_all_indices("unique", "que"), vec![3]);
}

#[test]
fn test_find_all_indices_unicode() {
    assert_eq!(find_all_indices("café café", "café"), vec![0, 6]);
    assert_eq!(find_all_indices("日本語", "本"), vec![3]); // byte index
}

#[test]
fn test_find_all_indices_overlapping_potential() {
    // .match_indices returns non-overlapping matches
    assert_eq!(find_all_indices("aaa", "aa"), vec![0]); // only first match
}

// ============================================================================
// extract_between tests
// ============================================================================

#[test]
fn test_extract_between_basic() {
    assert_eq!(
        extract_between("<tag>content</tag>", "<tag>", "</tag>"),
        Some("content".to_string())
    );
    assert_eq!(
        extract_between("hello [world] test", "[", "]"),
        Some("world".to_string())
    );
}

#[test]
fn test_extract_between_html() {
    assert_eq!(
        extract_between("<title>My Page</title>", "<title>", "</title>"),
        Some("My Page".to_string())
    );
    assert_eq!(
        extract_between("<div class='foo'>bar</div>", ">", "</"),
        Some("bar".to_string())
    );
}

#[test]
fn test_extract_between_not_found() {
    assert_eq!(extract_between("no markers here", "[", "]"), None);
    assert_eq!(extract_between("only start [", "[", "]"), None);
    assert_eq!(extract_between("only end ]", "[", "]"), None);
}

#[test]
fn test_extract_between_empty_content() {
    assert_eq!(
        extract_between("[]", "[", "]"),
        Some("".to_string())
    );
    assert_eq!(
        extract_between("<></>", "<>", "</>"),
        Some("".to_string())
    );
}

#[test]
fn test_extract_between_multiple_markers() {
    // Should return first match
    assert_eq!(
        extract_between("[first][second]", "[", "]"),
        Some("first".to_string())
    );
}

#[test]
fn test_extract_between_nested() {
    // Inner content may contain start marker (greedy behavior)
    assert_eq!(
        extract_between("[[nested]]", "[", "]"),
        Some("[nested".to_string())
    );
}

#[test]
fn test_extract_between_same_marker() {
    assert_eq!(
        extract_between("|content|rest", "|", "|"),
        Some("content".to_string())
    );
}

#[test]
fn test_extract_between_unicode() {
    assert_eq!(
        extract_between("「日本語」", "「", "」"),
        Some("日本語".to_string())
    );
    assert_eq!(
        extract_between("<<café>>", "<<", ">>"),
        Some("café".to_string())
    );
}

#[test]
fn test_extract_between_multichar_markers() {
    assert_eq!(
        extract_between("START>>>content<<<END", "START>>>", "<<<END"),
        Some("content".to_string())
    );
    assert_eq!(
        extract_between("BEGIN text FINISH", "BEGIN ", " FINISH"),
        Some("text".to_string())
    );
}
