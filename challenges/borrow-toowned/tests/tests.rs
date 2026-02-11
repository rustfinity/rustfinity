use borrow_toowned::*;
use std::borrow::Cow;
use std::collections::HashMap;

// ==================== lookup tests ====================

#[test]
fn test_lookup_string_key_with_str() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("hello".to_string(), 42);
    map.insert("world".to_string(), 100);

    assert_eq!(lookup(&map, "hello"), Some(&42));
    assert_eq!(lookup(&map, "world"), Some(&100));
}

#[test]
fn test_lookup_not_found() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("hello".to_string(), 42);

    assert_eq!(lookup(&map, "missing"), None);
}

#[test]
fn test_lookup_empty_map() {
    let map: HashMap<String, i32> = HashMap::new();
    assert_eq!(lookup(&map, "anything"), None);
}

#[test]
fn test_lookup_with_string_key() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("hello".to_string(), 42);

    let key = String::from("hello");
    assert_eq!(lookup(&map, key.as_str()), Some(&42));
}

#[test]
fn test_lookup_with_empty_string() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("".to_string(), 0);

    assert_eq!(lookup(&map, ""), Some(&0));
}

#[test]
fn test_lookup_unicode_keys() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("\u{1F44B}".to_string(), 1);  // 👋 waving hand
    map.insert("\u{1F600}".to_string(), 2);  // 😀 grinning face

    assert_eq!(lookup(&map, "\u{1F44B}"), Some(&1));
    assert_eq!(lookup(&map, "\u{1F600}"), Some(&2));
}

// ==================== make_owned tests ====================

#[test]
fn test_make_owned_simple() {
    let result = make_owned("hello");
    assert_eq!(result, String::from("hello"));
}

#[test]
fn test_make_owned_empty() {
    let result = make_owned("");
    assert_eq!(result, String::new());
}

#[test]
fn test_make_owned_unicode() {
    let result = make_owned("Hello, ");
    assert_eq!(result, String::from("Hello, "));
}

#[test]
fn test_make_owned_whitespace() {
    let result = make_owned("  hello  ");
    assert_eq!(result, String::from("  hello  "));
}

#[test]
fn test_make_owned_newlines() {
    let result = make_owned("line1\nline2\nline3");
    assert_eq!(result, String::from("line1\nline2\nline3"));
}

// ==================== normalize_whitespace tests ====================

#[test]
fn test_normalize_whitespace_no_change_needed() {
    let result = normalize_whitespace("hello world");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_double_spaces() {
    let result = normalize_whitespace("hello  world");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_multiple_spaces() {
    let result = normalize_whitespace("hello     world");
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_tabs() {
    let result = normalize_whitespace("hello\t\tworld");
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_mixed() {
    let result = normalize_whitespace("hello \t \n world");
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_empty() {
    let result = normalize_whitespace("");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "");
}

#[test]
fn test_normalize_whitespace_single_word() {
    let result = normalize_whitespace("hello");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello");
}

#[test]
fn test_normalize_whitespace_leading_trailing() {
    let result = normalize_whitespace("  hello  world  ");
    // This normalizes multiple spaces
    assert!(result.contains("hello world"));
}

#[test]
fn test_normalize_whitespace_all_spaces() {
    let result = normalize_whitespace("     ");
    assert_eq!(result.trim(), "");
}

// ==================== ensure_prefix tests ====================

#[test]
fn test_ensure_prefix_already_has_prefix() {
    let result = ensure_prefix("hello_world", "hello_");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello_world");
}

#[test]
fn test_ensure_prefix_needs_prefix() {
    let result = ensure_prefix("world", "hello_");
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result, "hello_world");
}

#[test]
fn test_ensure_prefix_empty_string() {
    let result = ensure_prefix("", "prefix_");
    assert_eq!(result, "prefix_");
}

#[test]
fn test_ensure_prefix_empty_prefix() {
    let result = ensure_prefix("hello", "");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "hello");
}

#[test]
fn test_ensure_prefix_partial_match() {
    let result = ensure_prefix("helloworld", "hello_");
    // "helloworld" does not start with "hello_"
    assert_eq!(result, "hello_helloworld");
}

#[test]
fn test_ensure_prefix_exact_match() {
    let result = ensure_prefix("prefix", "prefix");
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn test_ensure_prefix_unicode() {
    let result = ensure_prefix("world", "_");
    assert_eq!(result, "_world");
}

// ==================== CaseInsensitiveString tests ====================

#[test]
fn test_case_insensitive_equality() {
    let a = CaseInsensitiveString::new("Hello");
    let b = CaseInsensitiveString::new("HELLO");
    let c = CaseInsensitiveString::new("hello");

    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
fn test_case_insensitive_not_equal() {
    let a = CaseInsensitiveString::new("Hello");
    let b = CaseInsensitiveString::new("World");

    assert_ne!(a, b);
}

#[test]
fn test_case_insensitive_hash_map() {
    let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
    map.insert(CaseInsensitiveString::new("Hello"), 42);

    assert_eq!(map.get(&CaseInsensitiveString::new("Hello")), Some(&42));
    assert_eq!(map.get(&CaseInsensitiveString::new("HELLO")), Some(&42));
    assert_eq!(map.get(&CaseInsensitiveString::new("hello")), Some(&42));
    assert_eq!(map.get(&CaseInsensitiveString::new("HeLLo")), Some(&42));
}

#[test]
fn test_case_insensitive_hash_map_overwrite() {
    let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
    map.insert(CaseInsensitiveString::new("hello"), 1);
    map.insert(CaseInsensitiveString::new("HELLO"), 2);

    // Should only have one entry
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&CaseInsensitiveString::new("hello")), Some(&2));
}

#[test]
fn test_case_insensitive_as_str() {
    let s = CaseInsensitiveString::new("Hello World");
    assert_eq!(s.as_str(), "Hello World");
}

#[test]
fn test_case_insensitive_empty() {
    let a = CaseInsensitiveString::new("");
    let b = CaseInsensitiveString::new("");
    assert_eq!(a, b);
}

#[test]
fn test_case_insensitive_unicode() {
    // Note: case insensitivity for Unicode follows Rust's to_lowercase
    let a = CaseInsensitiveString::new("STRA\u{00DF}E"); // German eszett
    let b = CaseInsensitiveString::new("stra\u{00DF}e");
    assert_eq!(a, b);
}

// ==================== append_if_missing tests ====================

#[test]
fn test_append_if_missing_already_exists() {
    let items = [1, 2, 3];
    let result = append_if_missing(&items, 2);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3]);
}

#[test]
fn test_append_if_missing_not_present() {
    let items = [1, 2, 3];
    let result = append_if_missing(&items, 4);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1, 2, 3, 4]);
}

#[test]
fn test_append_if_missing_empty_slice() {
    let items: [i32; 0] = [];
    let result = append_if_missing(&items, 1);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(result.as_ref(), &[1]);
}

#[test]
fn test_append_if_missing_first_element() {
    let items = [1, 2, 3];
    let result = append_if_missing(&items, 1);
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn test_append_if_missing_last_element() {
    let items = [1, 2, 3];
    let result = append_if_missing(&items, 3);
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn test_append_if_missing_strings() {
    let items = ["a", "b", "c"];
    let result = append_if_missing(&items, "d");
    assert_eq!(result.as_ref(), &["a", "b", "c", "d"]);

    let result = append_if_missing(&items, "b");
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn test_append_if_missing_single_element() {
    let items = [42];
    let result = append_if_missing(&items, 42);
    assert!(matches!(result, Cow::Borrowed(_)));

    let result = append_if_missing(&items, 0);
    assert_eq!(result.as_ref(), &[42, 0]);
}

// ==================== normalize_path tests ====================

#[test]
fn test_normalize_path_no_dots() {
    let result = normalize_path("foo/bar");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "foo/bar");
}

#[test]
fn test_normalize_path_leading_dot_slash() {
    let result = normalize_path("./foo/bar");
    assert_eq!(result, "foo/bar");
}

#[test]
fn test_normalize_path_middle_dot_slash() {
    let result = normalize_path("foo/./bar");
    assert_eq!(result, "foo/bar");
}

#[test]
fn test_normalize_path_multiple_dot_slashes() {
    let result = normalize_path("./foo/./bar/./baz");
    assert_eq!(result, "foo/bar/baz");
}

#[test]
fn test_normalize_path_trailing_dot_slash() {
    let result = normalize_path("foo/bar/.");
    // The trailing . is not "./" so it might be preserved
    // But if followed by nothing, behavior depends on implementation
    assert!(result.starts_with("foo/bar"));
}

#[test]
fn test_normalize_path_only_dot_slash() {
    let result = normalize_path("./");
    // Should normalize to empty or minimal path
    assert!(result.is_empty() || result == ".");
}

#[test]
fn test_normalize_path_empty() {
    let result = normalize_path("");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "");
}

#[test]
fn test_normalize_path_just_filename() {
    let result = normalize_path("file.txt");
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(result, "file.txt");
}

#[test]
fn test_normalize_path_consecutive_dot_slashes() {
    let result = normalize_path("././foo");
    assert_eq!(result, "foo");
}

// ==================== to_owned_vec tests ====================

#[test]
fn test_to_owned_vec_integers() {
    let slice = &[1, 2, 3, 4, 5];
    let vec = to_owned_vec(slice);
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_to_owned_vec_empty() {
    let slice: &[i32] = &[];
    let vec = to_owned_vec(slice);
    assert!(vec.is_empty());
}

#[test]
fn test_to_owned_vec_strings() {
    let slice = &["a".to_string(), "b".to_string()];
    let vec = to_owned_vec(slice);
    assert_eq!(vec, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn test_to_owned_vec_str_slices() {
    let slice = &["hello", "world"];
    let vec = to_owned_vec(slice);
    assert_eq!(vec, vec!["hello", "world"]);
}

#[test]
fn test_to_owned_vec_single_element() {
    let slice = &[42];
    let vec = to_owned_vec(slice);
    assert_eq!(vec, vec![42]);
}

#[test]
fn test_to_owned_vec_chars() {
    let slice = &['a', 'b', 'c'];
    let vec = to_owned_vec(slice);
    assert_eq!(vec, vec!['a', 'b', 'c']);
}

// ==================== Integration tests ====================

#[test]
fn test_integration_lookup_with_owned_and_borrowed() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    // Look up with borrowed str
    assert_eq!(lookup(&map, "key1"), Some(&"value1".to_string()));

    // Look up with String converted to str
    let owned_key = String::from("key2");
    assert_eq!(lookup(&map, &owned_key[..]), Some(&"value2".to_string()));
}

#[test]
fn test_integration_cow_modifications() {
    // Test chaining Cow operations
    let input = "  hello   world  ";
    let normalized = normalize_whitespace(input);

    // Further process the result
    let final_result: String = normalized.trim().to_string();
    assert_eq!(final_result, "hello world");
}

#[test]
fn test_integration_ensure_prefix_chain() {
    let base = "config";
    let result = ensure_prefix(base, "app_");
    assert_eq!(result, "app_config");

    // Using the result of ensure_prefix
    let final_path = format!("{}.yaml", result);
    assert_eq!(final_path, "app_config.yaml");
}

#[test]
fn test_integration_case_insensitive_word_count() {
    let mut word_count: HashMap<CaseInsensitiveString, i32> = HashMap::new();

    let words = ["Hello", "HELLO", "hello", "World", "WORLD"];
    for word in &words {
        *word_count
            .entry(CaseInsensitiveString::new(*word))
            .or_insert(0) += 1;
    }

    assert_eq!(word_count.len(), 2);
    assert_eq!(
        word_count.get(&CaseInsensitiveString::new("hello")),
        Some(&3)
    );
    assert_eq!(
        word_count.get(&CaseInsensitiveString::new("world")),
        Some(&2)
    );
}

#[test]
fn test_integration_append_multiple() {
    let mut items = vec![1, 2, 3];
    let values_to_add = [2, 3, 4, 5];

    for &v in &values_to_add {
        match append_if_missing(&items, v) {
            Cow::Borrowed(_) => {
                // Value already exists, do nothing
            }
            Cow::Owned(new_vec) => {
                items = new_vec;
            }
        }
    }

    assert_eq!(items, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_integration_path_normalization_with_prefix() {
    let path = "./src/./lib.rs";
    let normalized = normalize_path(path);
    assert_eq!(normalized, "src/lib.rs");

    // Ensure it has a prefix
    let prefixed = ensure_prefix(&normalized, "/home/user/");
    assert_eq!(prefixed, "/home/user/src/lib.rs");
}

#[test]
fn test_integration_to_owned_and_modify() {
    let original = &[1, 2, 3];
    let mut owned = to_owned_vec(original);
    owned.push(4);
    owned.push(5);

    assert_eq!(owned, vec![1, 2, 3, 4, 5]);
    assert_eq!(original, &[1, 2, 3]); // Original unchanged
}

#[test]
fn test_borrow_trait_semantics() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    // Verify that CaseInsensitiveString properly implements Borrow
    let cis = CaseInsensitiveString::new("Hello");

    // Hash of the CaseInsensitiveString
    let mut hasher1 = DefaultHasher::new();
    cis.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    // Hash of another CaseInsensitiveString with different case
    let cis2 = CaseInsensitiveString::new("HELLO");
    let mut hasher2 = DefaultHasher::new();
    cis2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    // They should have the same hash
    assert_eq!(hash1, hash2);
}
