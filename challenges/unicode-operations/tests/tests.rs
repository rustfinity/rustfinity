use unicode_operations::*;

// ============================================
// Tests for char_count
// ============================================

#[test]
fn char_count_ascii() {
    assert_eq!(char_count("Hello"), 5);
    assert_eq!(char_count(""), 0);
    assert_eq!(char_count("a"), 1);
}

#[test]
fn char_count_cyrillic() {
    assert_eq!(char_count("Привет"), 6);
    assert_eq!(char_count("Здравствуйте"), 12);
}

#[test]
fn char_count_chinese() {
    assert_eq!(char_count("你好"), 2);
    assert_eq!(char_count("世界"), 2);
}

#[test]
fn char_count_emojis() {
    assert_eq!(char_count("🎉"), 1);
    assert_eq!(char_count("🎉🎊🎁"), 3);
}

#[test]
fn char_count_mixed() {
    assert_eq!(char_count("Hello, 世界!"), 10);
    assert_eq!(char_count("Rust 🦀"), 6);
}

// ============================================
// Tests for byte_count
// ============================================

#[test]
fn byte_count_ascii() {
    assert_eq!(byte_count("Hello"), 5);
    assert_eq!(byte_count(""), 0);
    assert_eq!(byte_count("a"), 1);
}

#[test]
fn byte_count_cyrillic() {
    // Cyrillic characters are 2 bytes each in UTF-8
    assert_eq!(byte_count("Привет"), 12);
}

#[test]
fn byte_count_chinese() {
    // Chinese characters are 3 bytes each in UTF-8
    assert_eq!(byte_count("你好"), 6);
    assert_eq!(byte_count("世界"), 6);
}

#[test]
fn byte_count_emojis() {
    // Most emojis are 4 bytes in UTF-8
    assert_eq!(byte_count("🎉"), 4);
    assert_eq!(byte_count("🎉🎊🎁"), 12);
}

#[test]
fn byte_count_mixed() {
    // "Hello, " = 7 bytes, "世界" = 6 bytes, "!" = 1 byte
    assert_eq!(byte_count("Hello, 世界!"), 14);
}

// ============================================
// Tests for safe_substring
// ============================================

#[test]
fn safe_substring_ascii() {
    assert_eq!(safe_substring("Hello", 0, 3), Some("Hel".to_string()));
    assert_eq!(safe_substring("Hello", 1, 4), Some("ell".to_string()));
    assert_eq!(safe_substring("Hello", 0, 5), Some("Hello".to_string()));
}

#[test]
fn safe_substring_cyrillic() {
    assert_eq!(safe_substring("Привет", 0, 2), Some("Пр".to_string()));
    assert_eq!(safe_substring("Привет", 2, 5), Some("иве".to_string()));
}

#[test]
fn safe_substring_chinese() {
    assert_eq!(safe_substring("你好世界", 0, 2), Some("你好".to_string()));
    assert_eq!(safe_substring("你好世界", 2, 4), Some("世界".to_string()));
}

#[test]
fn safe_substring_empty() {
    assert_eq!(safe_substring("Hello", 2, 2), Some("".to_string()));
    assert_eq!(safe_substring("", 0, 0), Some("".to_string()));
}

#[test]
fn safe_substring_out_of_bounds() {
    assert_eq!(safe_substring("Hello", 0, 10), None);
    assert_eq!(safe_substring("Hello", 6, 8), None);
}

#[test]
fn safe_substring_invalid_range() {
    assert_eq!(safe_substring("Hello", 3, 2), None);
    assert_eq!(safe_substring("Hello", 5, 3), None);
}

#[test]
fn safe_substring_emojis() {
    assert_eq!(safe_substring("🎉🎊🎁", 0, 2), Some("🎉🎊".to_string()));
    assert_eq!(safe_substring("🎉🎊🎁", 1, 3), Some("🎊🎁".to_string()));
}

// ============================================
// Tests for char_at
// ============================================

#[test]
fn char_at_ascii() {
    assert_eq!(char_at("Hello", 0), Some('H'));
    assert_eq!(char_at("Hello", 1), Some('e'));
    assert_eq!(char_at("Hello", 4), Some('o'));
}

#[test]
fn char_at_cyrillic() {
    assert_eq!(char_at("Привет", 0), Some('П'));
    assert_eq!(char_at("Привет", 2), Some('и'));
    assert_eq!(char_at("Привет", 5), Some('т'));
}

#[test]
fn char_at_chinese() {
    assert_eq!(char_at("你好世界", 0), Some('你'));
    assert_eq!(char_at("你好世界", 1), Some('好'));
    assert_eq!(char_at("你好世界", 3), Some('界'));
}

#[test]
fn char_at_emoji() {
    assert_eq!(char_at("🎉🎊🎁", 0), Some('🎉'));
    assert_eq!(char_at("🎉🎊🎁", 1), Some('🎊'));
    assert_eq!(char_at("🎉🎊🎁", 2), Some('🎁'));
}

#[test]
fn char_at_out_of_bounds() {
    assert_eq!(char_at("Hello", 5), None);
    assert_eq!(char_at("Hello", 100), None);
    assert_eq!(char_at("", 0), None);
}

#[test]
fn char_at_mixed() {
    let text = "Hello, 世界!";
    assert_eq!(char_at(text, 0), Some('H'));
    assert_eq!(char_at(text, 7), Some('世'));
    assert_eq!(char_at(text, 8), Some('界'));
    assert_eq!(char_at(text, 9), Some('!'));
}

// ============================================
// Tests for is_single_char
// ============================================

#[test]
fn is_single_char_ascii() {
    assert!(is_single_char("a"));
    assert!(is_single_char("Z"));
    assert!(is_single_char("!"));
}

#[test]
fn is_single_char_cyrillic() {
    assert!(is_single_char("П"));
    assert!(is_single_char("й"));
}

#[test]
fn is_single_char_chinese() {
    assert!(is_single_char("你"));
    assert!(is_single_char("好"));
}

#[test]
fn is_single_char_emoji() {
    assert!(is_single_char("🎉"));
    assert!(is_single_char("🦀"));
}

#[test]
fn is_single_char_false_cases() {
    assert!(!is_single_char(""));
    assert!(!is_single_char("ab"));
    assert!(!is_single_char("Hello"));
    assert!(!is_single_char("你好"));
    assert!(!is_single_char("🎉🎊"));
}

// ============================================
// Combined/Edge case tests
// ============================================

#[test]
fn test_whitespace_characters() {
    assert_eq!(char_count(" \t\n"), 3);
    assert_eq!(byte_count(" \t\n"), 3);
    assert!(is_single_char(" "));
    assert!(is_single_char("\t"));
}

#[test]
fn test_special_unicode() {
    // Zero-width joiner character (used in emoji sequences)
    let zwj = "\u{200D}";
    assert!(is_single_char(zwj));
    assert_eq!(char_count(zwj), 1);
    assert_eq!(byte_count(zwj), 3);
}

#[test]
fn test_combining_characters() {
    // 'é' can be represented as single char or as 'e' + combining accent
    let single = "é"; // U+00E9
    let combined = "e\u{0301}"; // 'e' + combining acute accent

    assert_eq!(char_count(single), 1);
    assert_eq!(char_count(combined), 2); // Two Unicode scalars

    assert!(is_single_char(single));
    assert!(!is_single_char(combined)); // Two chars, not one
}

#[test]
fn test_full_char_byte_comparison() {
    // Table of different scripts
    let tests = [
        ("Hello", 5, 5),     // ASCII: 1 byte per char
        ("Привет", 6, 12),   // Cyrillic: 2 bytes per char
        ("你好世界", 4, 12), // Chinese: 3 bytes per char
        ("🎉🎊🎁🎄", 4, 16), // Emojis: 4 bytes per char
    ];

    for (text, expected_chars, expected_bytes) in tests {
        assert_eq!(
            char_count(text),
            expected_chars,
            "char_count failed for \"{}\"",
            text
        );
        assert_eq!(
            byte_count(text),
            expected_bytes,
            "byte_count failed for \"{}\"",
            text
        );
    }
}
