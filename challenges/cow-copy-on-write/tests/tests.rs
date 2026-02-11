use cow_copy_on_write::*;
use std::borrow::Cow;

// =============================================================================
// Part 1: Basic Cow Usage - maybe_uppercase
// =============================================================================

#[test]
fn maybe_uppercase_already_uppercase() {
    let result = maybe_uppercase("HELLO");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "HELLO");
}

#[test]
fn maybe_uppercase_all_lowercase() {
    let result = maybe_uppercase("hello");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "HELLO");
}

#[test]
fn maybe_uppercase_mixed_case() {
    let result = maybe_uppercase("HeLLo");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "HELLO");
}

#[test]
fn maybe_uppercase_empty() {
    let result = maybe_uppercase("");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "");
}

#[test]
fn maybe_uppercase_numbers_only() {
    let result = maybe_uppercase("12345");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "12345");
}

#[test]
fn maybe_uppercase_special_chars() {
    let result = maybe_uppercase("!@#$%");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "!@#$%");
}

#[test]
fn maybe_uppercase_unicode() {
    let result = maybe_uppercase("ÜBER");
    assert!(matches!(result, Cow::Borrowed(_)));

    let result = maybe_uppercase("über");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "ÜBER");
}

#[test]
fn maybe_uppercase_single_lowercase() {
    let result = maybe_uppercase("HELLo");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "HELLO");
}

// =============================================================================
// Part 1: Basic Cow Usage - ensure_suffix
// =============================================================================

#[test]
fn ensure_suffix_already_present() {
    let result = ensure_suffix("file.txt", ".txt");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "file.txt");
}

#[test]
fn ensure_suffix_needs_suffix() {
    let result = ensure_suffix("file", ".txt");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "file.txt");
}

#[test]
fn ensure_suffix_empty_string() {
    let result = ensure_suffix("", ".txt");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, ".txt");
}

#[test]
fn ensure_suffix_empty_suffix() {
    let result = ensure_suffix("file.txt", "");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "file.txt");
}

#[test]
fn ensure_suffix_partial_match() {
    let result = ensure_suffix("file.tx", ".txt");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "file.tx.txt");
}

#[test]
fn ensure_suffix_longer_suffix() {
    let result = ensure_suffix("txt", "file.txt");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "txtfile.txt");
}

#[test]
fn ensure_suffix_multiple_occurrences() {
    let result = ensure_suffix(".txt.txt", ".txt");
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn ensure_suffix_slash() {
    let result = ensure_suffix("/path/to/dir", "/");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "/path/to/dir/");

    let result = ensure_suffix("/path/to/dir/", "/");
    assert!(matches!(result, Cow::Borrowed(_)));
}

// =============================================================================
// Part 1: Basic Cow Usage - trim_and_lowercase
// =============================================================================

#[test]
fn trim_and_lowercase_no_change_needed() {
    let result = trim_and_lowercase("hello");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_needs_trimming() {
    let result = trim_and_lowercase("  hello  ");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_needs_lowercasing() {
    let result = trim_and_lowercase("HELLO");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_needs_both() {
    let result = trim_and_lowercase("  HELLO  ");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_empty() {
    let result = trim_and_lowercase("");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "");
}

#[test]
fn trim_and_lowercase_whitespace_only() {
    let result = trim_and_lowercase("   ");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "");
}

#[test]
fn trim_and_lowercase_leading_whitespace() {
    let result = trim_and_lowercase("  hello");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_trailing_whitespace() {
    let result = trim_and_lowercase("hello  ");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello");
}

#[test]
fn trim_and_lowercase_unicode() {
    let result = trim_and_lowercase("  ÜBER  ");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "über");
}

#[test]
fn trim_and_lowercase_mixed_content() {
    let result = trim_and_lowercase("HeLLo WoRLD");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello world");
}

// =============================================================================
// Part 2: Cow with Collections - remove_zeros
// =============================================================================

#[test]
fn remove_zeros_no_zeros() {
    let data = [1, 2, 3, 4, 5];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3, 4, 5]);
}

#[test]
fn remove_zeros_has_zeros() {
    let data = [0, 1, 0, 2, 0];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2]);
}

#[test]
fn remove_zeros_all_zeros() {
    let data = [0, 0, 0];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert!(result.is_empty());
}

#[test]
fn remove_zeros_empty() {
    let data: [i32; 0] = [];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert!(result.is_empty());
}

#[test]
fn remove_zeros_single_zero() {
    let data = [0];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert!(result.is_empty());
}

#[test]
fn remove_zeros_single_nonzero() {
    let data = [42];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[42]);
}

#[test]
fn remove_zeros_negative_values() {
    let data = [-1, 0, 1, -2, 0, 2];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[-1, 1, -2, 2]);
}

#[test]
fn remove_zeros_leading_zero() {
    let data = [0, 1, 2, 3];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3]);
}

#[test]
fn remove_zeros_trailing_zero() {
    let data = [1, 2, 3, 0];
    let result = remove_zeros(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3]);
}

// =============================================================================
// Part 2: Cow with Collections - deduplicate_sorted
// =============================================================================

#[test]
fn deduplicate_sorted_no_duplicates() {
    let data = [1, 2, 3, 4, 5];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3, 4, 5]);
}

#[test]
fn deduplicate_sorted_has_duplicates() {
    let data = [1, 1, 2, 2, 3];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3]);
}

#[test]
fn deduplicate_sorted_all_same() {
    let data = [5, 5, 5, 5, 5];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[5]);
}

#[test]
fn deduplicate_sorted_empty() {
    let data: [i32; 0] = [];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert!(result.is_empty());
}

#[test]
fn deduplicate_sorted_single() {
    let data = [42];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[42]);
}

#[test]
fn deduplicate_sorted_two_same() {
    let data = [1, 1];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1]);
}

#[test]
fn deduplicate_sorted_two_different() {
    let data = [1, 2];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[1, 2]);
}

#[test]
fn deduplicate_sorted_strings() {
    let data = ["a", "a", "b", "c", "c"];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &["a", "b", "c"]);
}

#[test]
fn deduplicate_sorted_multiple_runs() {
    let data = [1, 1, 1, 2, 2, 3, 3, 3, 3];
    let result = deduplicate_sorted(&data);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3]);
}

// =============================================================================
// Part 2: Cow with Collections - clamp_values
// =============================================================================

#[test]
fn clamp_values_all_in_range() {
    let data = [5, 6, 7, 8];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[5, 6, 7, 8]);
}

#[test]
fn clamp_values_some_out_of_range() {
    let data = [-5, 5, 15];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[0, 5, 10]);
}

#[test]
fn clamp_values_all_below() {
    let data = [-10, -5, -1];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[0, 0, 0]);
}

#[test]
fn clamp_values_all_above() {
    let data = [15, 20, 100];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[10, 10, 10]);
}

#[test]
fn clamp_values_empty() {
    let data: [i32; 0] = [];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert!(result.is_empty());
}

#[test]
fn clamp_values_at_boundaries() {
    let data = [0, 5, 10];
    let result = clamp_values(&data, 0, 10);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[0, 5, 10]);
}

#[test]
fn clamp_values_negative_range() {
    let data = [-15, -5, 5];
    let result = clamp_values(&data, -10, 0);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[-10, -5, 0]);
}

#[test]
fn clamp_values_single_value_range() {
    let data = [0, 5, 10];
    let result = clamp_values(&data, 5, 5);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[5, 5, 5]);
}

// =============================================================================
// Part 3: Modifying Cow - ensure_capacity
// =============================================================================

#[test]
fn ensure_capacity_already_long() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let result = ensure_capacity(cow, 3, '!');
    assert_eq!(result, "hello");
}

#[test]
fn ensure_capacity_needs_padding() {
    let cow: Cow<str> = Cow::Borrowed("hi");
    let result = ensure_capacity(cow, 5, '!');
    assert_eq!(result, "hi!!!");
}

#[test]
fn ensure_capacity_exact_length() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let result = ensure_capacity(cow, 5, '!');
    assert_eq!(result, "hello");
}

#[test]
fn ensure_capacity_empty_string() {
    let cow: Cow<str> = Cow::Borrowed("");
    let result = ensure_capacity(cow, 3, 'x');
    assert_eq!(result, "xxx");
}

#[test]
fn ensure_capacity_zero_min() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let result = ensure_capacity(cow, 0, '!');
    assert_eq!(result, "hello");
}

#[test]
fn ensure_capacity_owned_input() {
    let cow: Cow<str> = Cow::Owned(String::from("hi"));
    let result = ensure_capacity(cow, 4, '.');
    assert_eq!(result, "hi..");
}

#[test]
fn ensure_capacity_unicode_padding() {
    let cow: Cow<str> = Cow::Borrowed("hi");
    let result = ensure_capacity(cow, 5, '★');
    assert_eq!(result, "hi★★★");
}

#[test]
fn ensure_capacity_unicode_string() {
    let cow: Cow<str> = Cow::Borrowed("日本");
    let result = ensure_capacity(cow, 4, '語');
    assert_eq!(result, "日本語語");
}

// =============================================================================
// Part 3: Modifying Cow - modify_if_needed
// =============================================================================

#[test]
fn modify_if_needed_predicate_false() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let result = modify_if_needed(cow, |s| s.contains('X'), |c| c.to_ascii_uppercase());
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello");
}

#[test]
fn modify_if_needed_predicate_true() {
    let cow: Cow<str> = Cow::Borrowed("hello");
    let result = modify_if_needed(cow, |s| s.contains('e'), |c| c.to_ascii_uppercase());
    assert_eq!(result, "HELLO");
}

#[test]
fn modify_if_needed_empty_string() {
    let cow: Cow<str> = Cow::Borrowed("");
    let result = modify_if_needed(cow, |_| true, |c| c.to_ascii_uppercase());
    assert_eq!(result, "");
}

#[test]
fn modify_if_needed_owned_input() {
    let cow: Cow<str> = Cow::Owned(String::from("hello"));
    let result = modify_if_needed(cow, |s| s.len() > 3, |c| c.to_ascii_uppercase());
    assert_eq!(result, "HELLO");
}

#[test]
fn modify_if_needed_custom_transform() {
    let cow: Cow<str> = Cow::Borrowed("abc");
    let result = modify_if_needed(cow, |_| true, |c| {
        if c == 'a' {
            'x'
        } else {
            c
        }
    });
    assert_eq!(result, "xbc");
}

#[test]
fn modify_if_needed_length_check() {
    let cow: Cow<str> = Cow::Borrowed("hi");
    let result = modify_if_needed(cow, |s| s.len() >= 5, |c| c.to_ascii_uppercase());
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hi");
}

// =============================================================================
// Part 4: TextProcessor
// =============================================================================

#[test]
fn text_processor_new_borrowed() {
    let processor = TextProcessor::new("hello");
    assert!(processor.is_borrowed());
    assert_eq!(processor.as_str(), "hello");
}

#[test]
fn text_processor_from_owned() {
    let processor = TextProcessor::from_owned(String::from("hello"));
    assert!(!processor.is_borrowed());
    assert_eq!(processor.as_str(), "hello");
}

#[test]
fn text_processor_process_no_change() {
    let mut processor = TextProcessor::new("hello");
    processor.process();
    assert_eq!(processor.as_str(), "hello");
}

#[test]
fn text_processor_process_trim() {
    let mut processor = TextProcessor::new("  hello  ");
    assert!(processor.is_borrowed());
    processor.process();
    assert!(!processor.is_borrowed());
    assert_eq!(processor.as_str(), "hello");
}

#[test]
fn text_processor_process_normalize_whitespace() {
    let mut processor = TextProcessor::new("hello   world");
    processor.process();
    assert_eq!(processor.as_str(), "hello world");
}

#[test]
fn text_processor_process_both() {
    let mut processor = TextProcessor::new("  hello   world  ");
    processor.process();
    assert_eq!(processor.as_str(), "hello world");
}

#[test]
fn text_processor_into_string_borrowed() {
    let processor = TextProcessor::new("hello");
    let s = processor.into_string();
    assert_eq!(s, "hello");
}

#[test]
fn text_processor_into_string_owned() {
    let processor = TextProcessor::from_owned(String::from("hello"));
    let s = processor.into_string();
    assert_eq!(s, "hello");
}

#[test]
fn text_processor_append() {
    let mut processor = TextProcessor::new("hello");
    assert!(processor.is_borrowed());
    processor.append(" world");
    assert!(!processor.is_borrowed());
    assert_eq!(processor.as_str(), "hello world");
}

#[test]
fn text_processor_len() {
    let processor = TextProcessor::new("hello");
    assert_eq!(processor.len(), 5);
}

#[test]
fn text_processor_is_empty() {
    let processor = TextProcessor::new("");
    assert!(processor.is_empty());

    let processor = TextProcessor::new("hello");
    assert!(!processor.is_empty());
}

#[test]
fn text_processor_default() {
    let processor = TextProcessor::default();
    assert!(processor.is_empty());
    assert!(!processor.is_borrowed());
}

#[test]
fn text_processor_clone() {
    let processor = TextProcessor::new("hello");
    let cloned = processor.clone();
    assert_eq!(cloned.as_str(), "hello");
}

#[test]
fn text_processor_unicode() {
    let mut processor = TextProcessor::new("  日本語   テスト  ");
    processor.process();
    assert_eq!(processor.as_str(), "日本語 テスト");
}

#[test]
fn text_processor_tabs_and_newlines() {
    let mut processor = TextProcessor::new("hello\t\tworld\n\ntest");
    processor.process();
    assert_eq!(processor.as_str(), "hello world test");
}

#[test]
fn text_processor_empty_after_trim() {
    let mut processor = TextProcessor::new("   ");
    processor.process();
    assert_eq!(processor.as_str(), "");
}

#[test]
fn text_processor_multiple_appends() {
    let mut processor = TextProcessor::new("a");
    processor.append("b");
    processor.append("c");
    assert_eq!(processor.as_str(), "abc");
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn integration_string_pipeline() {
    // Process a string through multiple Cow operations
    let input = "  HELLO WORLD  ";
    let result = trim_and_lowercase(input);
    assert_eq!(result, "hello world");

    let result = ensure_suffix(&result, "!");
    assert_eq!(result, "hello world!");
}

#[test]
fn integration_collection_pipeline() {
    // Process a collection through multiple operations
    let data = [0, 1, 1, 0, 2, 2, 0, 3];

    // First remove zeros
    let no_zeros = remove_zeros(&data);
    assert_eq!(no_zeros.as_ref(), &[1, 1, 2, 2, 3]);

    // Then deduplicate (data is sorted after removing zeros in this case)
    let unique = deduplicate_sorted(&no_zeros);
    assert_eq!(unique.as_ref(), &[1, 2, 3]);
}

#[test]
fn integration_text_processor_workflow() {
    let mut processor = TextProcessor::new("  HELLO   WORLD  ");
    assert!(processor.is_borrowed());

    processor.process();
    assert!(!processor.is_borrowed());
    assert_eq!(processor.as_str(), "HELLO WORLD");

    processor.append("!");
    assert_eq!(processor.as_str(), "HELLO WORLD!");

    let final_text = processor.into_string();
    assert_eq!(final_text, "HELLO WORLD!");
}

#[test]
fn integration_conditional_modifications() {
    // Test a pattern where we conditionally modify based on content
    let inputs = ["HELLO", "hello", "HeLLo", "12345"];

    for input in &inputs {
        let result = maybe_uppercase(input);
        assert!(result.chars().all(|c| !c.is_lowercase()));
    }
}

#[test]
fn integration_chained_ensure_suffix() {
    let filename = "document";

    // Chain multiple suffix operations
    let with_txt = ensure_suffix(filename, ".txt");
    assert_eq!(with_txt, "document.txt");

    // If we try to add it again, it should be borrowed
    let result = ensure_suffix(&with_txt, ".txt");
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn integration_clamp_and_deduplicate() {
    let data = [-100, 50, 50, 200, 200, 300];

    // First clamp to range
    let clamped = clamp_values(&data, 0, 100);
    assert_eq!(clamped.as_ref(), &[0, 50, 50, 100, 100, 100]);

    // Then deduplicate (already sorted after clamping this data)
    let unique = deduplicate_sorted(&clamped);
    assert_eq!(unique.as_ref(), &[0, 50, 100]);
}

#[test]
fn integration_modify_and_ensure_capacity() {
    let cow: Cow<str> = Cow::Borrowed("hello");

    // First uppercase if contains 'e'
    let result = modify_if_needed(cow, |s| s.contains('e'), |c| c.to_ascii_uppercase());
    assert_eq!(result, "HELLO");

    // Then ensure minimum length
    let result = ensure_capacity(result, 8, '!');
    assert_eq!(result, "HELLO!!!");
}

#[test]
fn integration_borrowed_vs_owned_tracking() {
    // Verify we can track when conversions happen
    let borrowed: Cow<str> = Cow::Borrowed("test");
    assert!(matches!(borrowed, Cow::Borrowed(_)));

    let owned: Cow<str> = Cow::Owned(String::from("test"));
    assert!(matches!(owned, Cow::Owned(_)));

    // Operations that don't need changes should stay borrowed
    let result = maybe_uppercase("TEST");
    assert!(matches!(result, Cow::Borrowed(_)));

    // Operations that need changes should become owned
    let result = maybe_uppercase("test");
    assert!(matches!(result, Cow::Owned(_)));
}

#[test]
fn integration_empty_input_handling() {
    // Test all functions with empty input
    assert!(matches!(maybe_uppercase(""), Cow::Borrowed(_)));
    assert!(matches!(ensure_suffix("", ".txt"), Cow::Owned(_)));
    assert!(matches!(trim_and_lowercase(""), Cow::Borrowed(_)));
    assert!(matches!(remove_zeros(&[]), Cow::Borrowed(_)));
    assert!(matches!(deduplicate_sorted::<i32>(&[]), Cow::Borrowed(_)));
    assert!(matches!(clamp_values(&[], 0, 10), Cow::Borrowed(_)));
}

#[test]
fn integration_processor_reuse() {
    let mut processor = TextProcessor::new("hello");
    processor.append(" world");
    processor.process();

    // After processing, we can still use it
    assert_eq!(processor.len(), 11);
    assert!(!processor.is_empty());

    processor.append("!");
    assert_eq!(processor.into_string(), "hello world!");
}
