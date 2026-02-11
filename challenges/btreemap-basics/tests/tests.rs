use btreemap_basics::*;
use std::collections::BTreeMap;

// ============================================================================
// Tests for create_sorted_map
// ============================================================================

#[test]
fn test_create_sorted_map_basic() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("apple"), Some(&1));
    assert_eq!(map.get("banana"), Some(&2));
    assert_eq!(map.get("cherry"), Some(&3));
}

#[test]
fn test_create_sorted_map_empty() {
    let pairs: Vec<(String, i32)> = vec![];
    let map = create_sorted_map(&pairs);
    assert!(map.is_empty());
}

#[test]
fn test_create_sorted_map_single() {
    let pairs = vec![("only".to_string(), 42)];
    let map = create_sorted_map(&pairs);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("only"), Some(&42));
}

#[test]
fn test_create_sorted_map_duplicate_keys() {
    // Later duplicate overwrites earlier
    let pairs = vec![
        ("key".to_string(), 1),
        ("key".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key"), Some(&2));
}

#[test]
fn test_create_sorted_map_maintains_order() {
    let pairs = vec![
        ("zebra".to_string(), 4),
        ("alpha".to_string(), 1),
        ("mango".to_string(), 3),
        ("beta".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec!["alpha", "beta", "mango", "zebra"]);
}

// ============================================================================
// Tests for get_value
// ============================================================================

#[test]
fn test_get_value_exists() {
    let pairs = vec![
        ("apple".to_string(), 100),
        ("banana".to_string(), 200),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_value(&map, "apple"), Some(100));
    assert_eq!(get_value(&map, "banana"), Some(200));
}

#[test]
fn test_get_value_not_exists() {
    let pairs = vec![("apple".to_string(), 100)];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_value(&map, "grape"), None);
    assert_eq!(get_value(&map, ""), None);
}

#[test]
fn test_get_value_empty_map() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    assert_eq!(get_value(&map, "anything"), None);
}

#[test]
fn test_get_value_negative_values() {
    let pairs = vec![
        ("negative".to_string(), -42),
        ("zero".to_string(), 0),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_value(&map, "negative"), Some(-42));
    assert_eq!(get_value(&map, "zero"), Some(0));
}

// ============================================================================
// Tests for get_keys_in_order
// ============================================================================

#[test]
fn test_get_keys_in_order_basic() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_keys_in_order(&map), vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_get_keys_in_order_empty() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    let keys: Vec<String> = get_keys_in_order(&map);
    assert!(keys.is_empty());
}

#[test]
fn test_get_keys_in_order_single() {
    let pairs = vec![("single".to_string(), 1)];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_keys_in_order(&map), vec!["single"]);
}

#[test]
fn test_get_keys_in_order_numbers_as_strings() {
    let pairs = vec![
        ("10".to_string(), 10),
        ("2".to_string(), 2),
        ("1".to_string(), 1),
    ];
    let map = create_sorted_map(&pairs);
    // String comparison: "1" < "10" < "2" (lexicographic)
    assert_eq!(get_keys_in_order(&map), vec!["1", "10", "2"]);
}

// ============================================================================
// Tests for get_values_in_key_order
// ============================================================================

#[test]
fn test_get_values_in_key_order_basic() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    // Values in order of sorted keys: apple(1), banana(2), cherry(3)
    assert_eq!(get_values_in_key_order(&map), vec![1, 2, 3]);
}

#[test]
fn test_get_values_in_key_order_empty() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    let values: Vec<i32> = get_values_in_key_order(&map);
    assert!(values.is_empty());
}

#[test]
fn test_get_values_in_key_order_same_values() {
    let pairs = vec![
        ("c".to_string(), 5),
        ("a".to_string(), 5),
        ("b".to_string(), 5),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_values_in_key_order(&map), vec![5, 5, 5]);
}

#[test]
fn test_get_values_in_key_order_mixed() {
    let pairs = vec![
        ("z".to_string(), -10),
        ("a".to_string(), 100),
        ("m".to_string(), 0),
    ];
    let map = create_sorted_map(&pairs);
    // Order: a(100), m(0), z(-10)
    assert_eq!(get_values_in_key_order(&map), vec![100, 0, -10]);
}

// ============================================================================
// Tests for get_range
// ============================================================================

#[test]
fn test_get_range_basic() {
    let pairs = vec![
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
        ("cherry".to_string(), 3),
        ("date".to_string(), 4),
    ];
    let map = create_sorted_map(&pairs);
    let range = get_range(&map, "banana", "date");
    assert_eq!(range, vec![
        ("banana".to_string(), 2),
        ("cherry".to_string(), 3),
    ]);
}

#[test]
fn test_get_range_full() {
    let pairs = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ];
    let map = create_sorted_map(&pairs);
    let range = get_range(&map, "a", "d");
    assert_eq!(range, vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ]);
}

#[test]
fn test_get_range_empty_result() {
    let pairs = vec![
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    // Range that doesn't include any keys
    let range = get_range(&map, "cherry", "date");
    assert!(range.is_empty());
}

#[test]
fn test_get_range_single_element() {
    let pairs = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ];
    let map = create_sorted_map(&pairs);
    let range = get_range(&map, "b", "c");
    assert_eq!(range, vec![("b".to_string(), 2)]);
}

#[test]
fn test_get_range_start_equals_end() {
    let pairs = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    // Empty range when start == end
    let range = get_range(&map, "a", "a");
    assert!(range.is_empty());
}

#[test]
fn test_get_range_empty_map() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    let range = get_range(&map, "a", "z");
    assert!(range.is_empty());
}

#[test]
fn test_get_range_partial_match() {
    let pairs = vec![
        ("cat".to_string(), 1),
        ("dog".to_string(), 2),
        ("elephant".to_string(), 3),
    ];
    let map = create_sorted_map(&pairs);
    // Range includes some existing keys
    let range = get_range(&map, "cow", "duck");
    assert_eq!(range, vec![("dog".to_string(), 2)]);
}

// ============================================================================
// Tests for get_first
// ============================================================================

#[test]
fn test_get_first_basic() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_first(&map), Some(("apple".to_string(), 1)));
}

#[test]
fn test_get_first_empty() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    assert_eq!(get_first(&map), None);
}

#[test]
fn test_get_first_single() {
    let pairs = vec![("only".to_string(), 99)];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_first(&map), Some(("only".to_string(), 99)));
}

#[test]
fn test_get_first_special_characters() {
    let pairs = vec![
        ("beta".to_string(), 2),
        ("!special".to_string(), 0),
        ("alpha".to_string(), 1),
    ];
    let map = create_sorted_map(&pairs);
    // '!' comes before letters in ASCII
    assert_eq!(get_first(&map), Some(("!special".to_string(), 0)));
}

// ============================================================================
// Tests for get_last
// ============================================================================

#[test]
fn test_get_last_basic() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_last(&map), Some(("cherry".to_string(), 3)));
}

#[test]
fn test_get_last_empty() {
    let map: BTreeMap<String, i32> = BTreeMap::new();
    assert_eq!(get_last(&map), None);
}

#[test]
fn test_get_last_single() {
    let pairs = vec![("only".to_string(), 99)];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_last(&map), Some(("only".to_string(), 99)));
}

#[test]
fn test_get_last_unicode() {
    let pairs = vec![
        ("alpha".to_string(), 1),
        ("beta".to_string(), 2),
        ("zeta".to_string(), 3),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_last(&map), Some(("zeta".to_string(), 3)));
}

// ============================================================================
// Tests for first and last on same map
// ============================================================================

#[test]
fn test_first_and_last_same_element() {
    let pairs = vec![("only".to_string(), 42)];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_first(&map), get_last(&map));
    assert_eq!(get_first(&map), Some(("only".to_string(), 42)));
}

#[test]
fn test_first_and_last_different() {
    let pairs = vec![
        ("a".to_string(), 1),
        ("z".to_string(), 26),
    ];
    let map = create_sorted_map(&pairs);
    assert_eq!(get_first(&map), Some(("a".to_string(), 1)));
    assert_eq!(get_last(&map), Some(("z".to_string(), 26)));
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_workflow_example() {
    // Simulate a score tracking system
    let scores = vec![
        ("player_c".to_string(), 150),
        ("player_a".to_string(), 200),
        ("player_b".to_string(), 175),
        ("player_d".to_string(), 125),
    ];
    let map = create_sorted_map(&scores);

    // Keys are sorted
    assert_eq!(
        get_keys_in_order(&map),
        vec!["player_a", "player_b", "player_c", "player_d"]
    );

    // Get a specific player's score
    assert_eq!(get_value(&map, "player_b"), Some(175));

    // Get range of players
    let range = get_range(&map, "player_a", "player_c");
    assert_eq!(range.len(), 2);

    // First player alphabetically
    assert_eq!(get_first(&map), Some(("player_a".to_string(), 200)));

    // Last player alphabetically
    assert_eq!(get_last(&map), Some(("player_d".to_string(), 125)));
}

#[test]
fn test_large_map() {
    let pairs: Vec<(String, i32)> = (0..100)
        .map(|i| (format!("key_{:03}", i), i))
        .collect();
    let map = create_sorted_map(&pairs);

    assert_eq!(map.len(), 100);
    assert_eq!(get_first(&map), Some(("key_000".to_string(), 0)));
    assert_eq!(get_last(&map), Some(("key_099".to_string(), 99)));

    let range = get_range(&map, "key_050", "key_055");
    assert_eq!(range.len(), 5);
    assert_eq!(range[0], ("key_050".to_string(), 50));
    assert_eq!(range[4], ("key_054".to_string(), 54));
}
