use entry_api::*;
use std::collections::HashMap;

// ==================== count_words tests ====================

#[test]
fn count_words_single_word() {
    let counts = count_words("hello");
    assert_eq!(counts.len(), 1);
    assert_eq!(counts["hello"], 1);
}

#[test]
fn count_words_multiple_different_words() {
    let counts = count_words("hello world");
    assert_eq!(counts.len(), 2);
    assert_eq!(counts["hello"], 1);
    assert_eq!(counts["world"], 1);
}

#[test]
fn count_words_with_duplicates() {
    let counts = count_words("the quick brown fox jumps over the lazy dog");
    assert_eq!(counts["the"], 2);
    assert_eq!(counts["quick"], 1);
    assert_eq!(counts["fox"], 1);
}

#[test]
fn count_words_case_insensitive() {
    let counts = count_words("Hello HELLO hello");
    assert_eq!(counts.len(), 1);
    assert_eq!(counts["hello"], 3);
}

#[test]
fn count_words_empty_string() {
    let counts = count_words("");
    assert!(counts.is_empty());
}

#[test]
fn count_words_whitespace_only() {
    let counts = count_words("   \t\n  ");
    assert!(counts.is_empty());
}

#[test]
fn count_words_many_duplicates() {
    let counts = count_words("a a a a a b b b c");
    assert_eq!(counts["a"], 5);
    assert_eq!(counts["b"], 3);
    assert_eq!(counts["c"], 1);
}

// ==================== group_by_length tests ====================

#[test]
fn group_by_length_single_word() {
    let grouped = group_by_length(&["hello"]);
    assert_eq!(grouped.len(), 1);
    assert_eq!(grouped[&5], vec!["hello"]);
}

#[test]
fn group_by_length_same_length() {
    let grouped = group_by_length(&["hello", "world", "again"]);
    assert_eq!(grouped.len(), 1);
    assert_eq!(grouped[&5], vec!["hello", "world", "again"]);
}

#[test]
fn group_by_length_different_lengths() {
    let grouped = group_by_length(&["hi", "hello", "hey", "world"]);
    assert_eq!(grouped[&2], vec!["hi"]);
    assert_eq!(grouped[&5], vec!["hello", "world"]);
    assert_eq!(grouped[&3], vec!["hey"]);
}

#[test]
fn group_by_length_empty_slice() {
    let grouped = group_by_length(&[]);
    assert!(grouped.is_empty());
}

#[test]
fn group_by_length_with_empty_string() {
    let grouped = group_by_length(&["", "a", "ab"]);
    assert_eq!(grouped[&0], vec![""]);
    assert_eq!(grouped[&1], vec!["a"]);
    assert_eq!(grouped[&2], vec!["ab"]);
}

#[test]
fn group_by_length_preserves_order() {
    let grouped = group_by_length(&["cat", "dog", "bat"]);
    assert_eq!(grouped[&3], vec!["cat", "dog", "bat"]);
}

// ==================== get_or_compute tests ====================

#[test]
fn get_or_compute_inserts_new_value() {
    let mut cache: HashMap<String, i32> = HashMap::new();
    let value = get_or_compute(&mut cache, "key", || 42);
    assert_eq!(value, 42);
    assert_eq!(cache["key"], 42);
}

#[test]
fn get_or_compute_returns_existing() {
    let mut cache: HashMap<String, i32> = HashMap::new();
    cache.insert("key".to_string(), 100);
    let value = get_or_compute(&mut cache, "key", || 42);
    assert_eq!(value, 100);
}

#[test]
fn get_or_compute_lazy_evaluation() {
    let mut cache: HashMap<String, i32> = HashMap::new();
    cache.insert("key".to_string(), 100);
    let mut called = false;
    let value = get_or_compute(&mut cache, "key", || {
        called = true;
        42
    });
    assert_eq!(value, 100);
    assert!(!called, "Compute function should not be called for existing key");
}

#[test]
fn get_or_compute_calls_closure_for_missing() {
    let mut cache: HashMap<String, i32> = HashMap::new();
    let mut call_count = 0;
    let _ = get_or_compute(&mut cache, "key", || {
        call_count += 1;
        42
    });
    assert_eq!(call_count, 1);
}

#[test]
fn get_or_compute_multiple_keys() {
    let mut cache: HashMap<String, i32> = HashMap::new();
    let v1 = get_or_compute(&mut cache, "a", || 1);
    let v2 = get_or_compute(&mut cache, "b", || 2);
    let v3 = get_or_compute(&mut cache, "a", || 100);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 1); // Should return cached value, not 100
}

// ==================== increment_or_init tests ====================

#[test]
fn increment_or_init_inserts_initial() {
    let mut map: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut map, "key", 100);
    assert_eq!(map["key"], 100);
}

#[test]
fn increment_or_init_increments_existing() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key".to_string(), 50);
    increment_or_init(&mut map, "key", 100);
    assert_eq!(map["key"], 51);
}

#[test]
fn increment_or_init_multiple_increments() {
    let mut map: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut map, "key", 0);
    increment_or_init(&mut map, "key", 0);
    increment_or_init(&mut map, "key", 0);
    assert_eq!(map["key"], 2); // Init + 2 increments
}

#[test]
fn increment_or_init_different_keys() {
    let mut map: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut map, "a", 10);
    increment_or_init(&mut map, "b", 20);
    increment_or_init(&mut map, "a", 10);
    assert_eq!(map["a"], 11);
    assert_eq!(map["b"], 20);
}

#[test]
fn increment_or_init_negative_init() {
    let mut map: HashMap<String, i32> = HashMap::new();
    increment_or_init(&mut map, "key", -5);
    assert_eq!(map["key"], -5);
    increment_or_init(&mut map, "key", -5);
    assert_eq!(map["key"], -4);
}

// ==================== merge_maps tests ====================

#[test]
fn merge_maps_disjoint() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = [("b".to_string(), 2)].into();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["a"], 1);
    assert_eq!(merged["b"], 2);
}

#[test]
fn merge_maps_overlapping() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1), ("b".to_string(), 2)].into();
    let map2: HashMap<String, i32> = [("b".to_string(), 3), ("c".to_string(), 4)].into();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["a"], 1);
    assert_eq!(merged["b"], 5);
    assert_eq!(merged["c"], 4);
}

#[test]
fn merge_maps_empty_first() {
    let map1: HashMap<String, i32> = HashMap::new();
    let map2: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["a"], 1);
}

#[test]
fn merge_maps_empty_second() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = HashMap::new();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["a"], 1);
}

#[test]
fn merge_maps_both_empty() {
    let map1: HashMap<String, i32> = HashMap::new();
    let map2: HashMap<String, i32> = HashMap::new();
    let merged = merge_maps(map1, map2);
    assert!(merged.is_empty());
}

#[test]
fn merge_maps_negative_values() {
    let map1: HashMap<String, i32> = [("a".to_string(), -5)].into();
    let map2: HashMap<String, i32> = [("a".to_string(), 3)].into();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["a"], -2);
}

#[test]
fn merge_maps_same_keys() {
    let map1: HashMap<String, i32> = [("x".to_string(), 10), ("y".to_string(), 20)].into();
    let map2: HashMap<String, i32> = [("x".to_string(), 5), ("y".to_string(), 15)].into();
    let merged = merge_maps(map1, map2);
    assert_eq!(merged["x"], 15);
    assert_eq!(merged["y"], 35);
}

// ==================== first_occurrence tests ====================

#[test]
fn first_occurrence_all_unique() {
    let firsts = first_occurrence(&["a", "b", "c"]);
    assert_eq!(firsts["a"], 0);
    assert_eq!(firsts["b"], 1);
    assert_eq!(firsts["c"], 2);
}

#[test]
fn first_occurrence_with_duplicates() {
    let firsts = first_occurrence(&["a", "b", "a", "c", "b"]);
    assert_eq!(firsts["a"], 0);
    assert_eq!(firsts["b"], 1);
    assert_eq!(firsts["c"], 3);
}

#[test]
fn first_occurrence_all_same() {
    let firsts = first_occurrence(&["x", "x", "x"]);
    assert_eq!(firsts.len(), 1);
    assert_eq!(firsts["x"], 0);
}

#[test]
fn first_occurrence_empty() {
    let firsts = first_occurrence(&[]);
    assert!(firsts.is_empty());
}

#[test]
fn first_occurrence_single() {
    let firsts = first_occurrence(&["only"]);
    assert_eq!(firsts["only"], 0);
}

#[test]
fn first_occurrence_late_duplicate() {
    let firsts = first_occurrence(&["a", "b", "c", "d", "e", "a"]);
    assert_eq!(firsts["a"], 0);
    assert_eq!(firsts["e"], 4);
}

// ==================== update_or_default tests ====================

#[test]
fn update_or_default_new_key() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut map, "key", 42);
    assert_eq!(map["key"], vec![42]);
}

#[test]
fn update_or_default_existing_key() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    map.insert("key".to_string(), vec![1, 2]);
    update_or_default(&mut map, "key", 3);
    assert_eq!(map["key"], vec![1, 2, 3]);
}

#[test]
fn update_or_default_multiple_adds() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut map, "nums", 1);
    update_or_default(&mut map, "nums", 2);
    update_or_default(&mut map, "nums", 3);
    assert_eq!(map["nums"], vec![1, 2, 3]);
}

#[test]
fn update_or_default_different_keys() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut map, "a", 1);
    update_or_default(&mut map, "b", 2);
    update_or_default(&mut map, "a", 3);
    assert_eq!(map["a"], vec![1, 3]);
    assert_eq!(map["b"], vec![2]);
}

#[test]
fn update_or_default_negative_values() {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    update_or_default(&mut map, "key", -1);
    update_or_default(&mut map, "key", -2);
    assert_eq!(map["key"], vec![-1, -2]);
}

// ==================== Integration tests ====================

#[test]
fn integration_word_frequency_analysis() {
    let text = "rust is great rust is fast rust is safe";
    let counts = count_words(text);

    assert_eq!(counts["rust"], 3);
    assert_eq!(counts["is"], 3);
    assert_eq!(counts["great"], 1);
    assert_eq!(counts["fast"], 1);
    assert_eq!(counts["safe"], 1);
}

#[test]
fn integration_cache_pattern() {
    let mut cache: HashMap<String, i32> = HashMap::new();

    // First access computes
    let v1 = get_or_compute(&mut cache, "expensive", || 1000);
    assert_eq!(v1, 1000);

    // Second access returns cached
    let v2 = get_or_compute(&mut cache, "expensive", || 9999);
    assert_eq!(v2, 1000);

    // Different key computes new value
    let v3 = get_or_compute(&mut cache, "another", || 2000);
    assert_eq!(v3, 2000);
}

#[test]
fn integration_score_tracking() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Initialize players
    increment_or_init(&mut scores, "alice", 100);
    increment_or_init(&mut scores, "bob", 100);

    // Alice scores multiple times
    increment_or_init(&mut scores, "alice", 0);
    increment_or_init(&mut scores, "alice", 0);

    // Bob scores once
    increment_or_init(&mut scores, "bob", 0);

    assert_eq!(scores["alice"], 102);
    assert_eq!(scores["bob"], 101);
}

#[test]
fn integration_event_grouping() {
    let mut events: HashMap<String, Vec<i32>> = HashMap::new();

    update_or_default(&mut events, "errors", 404);
    update_or_default(&mut events, "errors", 500);
    update_or_default(&mut events, "success", 200);
    update_or_default(&mut events, "errors", 403);

    assert_eq!(events["errors"], vec![404, 500, 403]);
    assert_eq!(events["success"], vec![200]);
}

#[test]
fn integration_complex_merge() {
    let daily: HashMap<String, i32> = [
        ("views".to_string(), 100),
        ("clicks".to_string(), 10),
    ].into();

    let hourly: HashMap<String, i32> = [
        ("views".to_string(), 50),
        ("shares".to_string(), 5),
    ].into();

    let total = merge_maps(daily, hourly);
    assert_eq!(total["views"], 150);
    assert_eq!(total["clicks"], 10);
    assert_eq!(total["shares"], 5);
}
