use collection_conversions::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// ==================== vec_to_hashset tests ====================

#[test]
fn test_vec_to_hashset_basic() {
    let set = vec_to_hashset(vec![1, 2, 3]);
    assert_eq!(set.len(), 3);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_vec_to_hashset_with_duplicates() {
    let set = vec_to_hashset(vec![1, 2, 2, 3, 3, 3]);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_vec_to_hashset_empty() {
    let set: HashSet<i32> = vec_to_hashset(vec![]);
    assert!(set.is_empty());
}

#[test]
fn test_vec_to_hashset_single_element() {
    let set = vec_to_hashset(vec![42]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&42));
}

#[test]
fn test_vec_to_hashset_all_duplicates() {
    let set = vec_to_hashset(vec![5, 5, 5, 5, 5]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&5));
}

#[test]
fn test_vec_to_hashset_strings() {
    let set = vec_to_hashset(vec!["apple", "banana", "apple"]);
    assert_eq!(set.len(), 2);
    assert!(set.contains("apple"));
    assert!(set.contains("banana"));
}

// ==================== vec_to_btreeset tests ====================

#[test]
fn test_vec_to_btreeset_basic() {
    let set = vec_to_btreeset(vec![3, 1, 2]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![1, 2, 3]);
}

#[test]
fn test_vec_to_btreeset_with_duplicates() {
    let set = vec_to_btreeset(vec![3, 1, 2, 2, 1]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![1, 2, 3]);
}

#[test]
fn test_vec_to_btreeset_empty() {
    let set: BTreeSet<i32> = vec_to_btreeset(vec![]);
    assert!(set.is_empty());
}

#[test]
fn test_vec_to_btreeset_already_sorted() {
    let set = vec_to_btreeset(vec![1, 2, 3, 4, 5]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_vec_to_btreeset_reverse_sorted() {
    let set = vec_to_btreeset(vec![5, 4, 3, 2, 1]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_vec_to_btreeset_strings() {
    let set = vec_to_btreeset(vec!["cherry", "apple", "banana"]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec!["apple", "banana", "cherry"]);
}

// ==================== hashset_to_sorted_vec tests ====================

#[test]
fn test_hashset_to_sorted_vec_basic() {
    let set: HashSet<i32> = [3, 1, 4, 1, 5, 9].into_iter().collect();
    let sorted = hashset_to_sorted_vec(set);
    assert_eq!(sorted, vec![1, 3, 4, 5, 9]);
}

#[test]
fn test_hashset_to_sorted_vec_empty() {
    let set: HashSet<i32> = HashSet::new();
    let sorted = hashset_to_sorted_vec(set);
    assert!(sorted.is_empty());
}

#[test]
fn test_hashset_to_sorted_vec_single() {
    let set: HashSet<i32> = [42].into_iter().collect();
    let sorted = hashset_to_sorted_vec(set);
    assert_eq!(sorted, vec![42]);
}

#[test]
fn test_hashset_to_sorted_vec_negative_numbers() {
    let set: HashSet<i32> = [-3, 0, 5, -1, 2].into_iter().collect();
    let sorted = hashset_to_sorted_vec(set);
    assert_eq!(sorted, vec![-3, -1, 0, 2, 5]);
}

#[test]
fn test_hashset_to_sorted_vec_strings() {
    let set: HashSet<&str> = ["zebra", "apple", "mango"].into_iter().collect();
    let sorted = hashset_to_sorted_vec(set);
    assert_eq!(sorted, vec!["apple", "mango", "zebra"]);
}

// ==================== pairs_to_hashmap tests ====================

#[test]
fn test_pairs_to_hashmap_basic() {
    let map = pairs_to_hashmap(vec![("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
    assert_eq!(map.get("c"), Some(&3));
    assert_eq!(map.len(), 3);
}

#[test]
fn test_pairs_to_hashmap_empty() {
    let map: HashMap<&str, i32> = pairs_to_hashmap(vec![]);
    assert!(map.is_empty());
}

#[test]
fn test_pairs_to_hashmap_duplicate_keys() {
    let map = pairs_to_hashmap(vec![("a", 1), ("a", 2), ("a", 3)]);
    assert_eq!(map.len(), 1);
    // Last value should win
    assert_eq!(map.get("a"), Some(&3));
}

#[test]
fn test_pairs_to_hashmap_int_keys() {
    let map = pairs_to_hashmap(vec![(1, "one"), (2, "two"), (3, "three")]);
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&2), Some(&"two"));
}

#[test]
fn test_pairs_to_hashmap_single_pair() {
    let map = pairs_to_hashmap(vec![("only", 100)]);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("only"), Some(&100));
}

// ==================== pairs_to_btreemap tests ====================

#[test]
fn test_pairs_to_btreemap_basic() {
    let map = pairs_to_btreemap(vec![("c", 3), ("a", 1), ("b", 2)]);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&"a", &"b", &"c"]);
}

#[test]
fn test_pairs_to_btreemap_empty() {
    let map: BTreeMap<&str, i32> = pairs_to_btreemap(vec![]);
    assert!(map.is_empty());
}

#[test]
fn test_pairs_to_btreemap_duplicate_keys() {
    let map = pairs_to_btreemap(vec![("x", 1), ("x", 2), ("x", 3)]);
    assert_eq!(map.len(), 1);
    // Last value should win
    assert_eq!(map.get("x"), Some(&3));
}

#[test]
fn test_pairs_to_btreemap_int_keys_sorted() {
    let map = pairs_to_btreemap(vec![(3, "three"), (1, "one"), (2, "two")]);
    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn test_pairs_to_btreemap_values_accessible() {
    let map = pairs_to_btreemap(vec![("z", 26), ("a", 1), ("m", 13)]);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("m"), Some(&13));
    assert_eq!(map.get("z"), Some(&26));
}

// ==================== hashmap_to_pairs tests ====================

#[test]
fn test_hashmap_to_pairs_basic() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let pairs = hashmap_to_pairs(map);
    assert_eq!(pairs.len(), 2);
    assert!(pairs.contains(&("a", 1)));
    assert!(pairs.contains(&("b", 2)));
}

#[test]
fn test_hashmap_to_pairs_empty() {
    let map: HashMap<&str, i32> = HashMap::new();
    let pairs = hashmap_to_pairs(map);
    assert!(pairs.is_empty());
}

#[test]
fn test_hashmap_to_pairs_single() {
    let mut map = HashMap::new();
    map.insert(42, "answer");
    let pairs = hashmap_to_pairs(map);
    assert_eq!(pairs.len(), 1);
    assert!(pairs.contains(&(42, "answer")));
}

#[test]
fn test_hashmap_to_pairs_preserves_all_entries() {
    let mut map = HashMap::new();
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let pairs = hashmap_to_pairs(map);
    assert_eq!(pairs.len(), 10);
    for i in 0..10 {
        assert!(pairs.contains(&(i, i * 10)));
    }
}

// ==================== merge_vecs tests ====================

#[test]
fn test_merge_vecs_basic() {
    let merged = merge_vecs(vec![vec![1, 2], vec![3, 4], vec![5]]);
    assert_eq!(merged, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_merge_vecs_empty_outer() {
    let merged: Vec<i32> = merge_vecs(vec![]);
    assert!(merged.is_empty());
}

#[test]
fn test_merge_vecs_empty_inner() {
    let merged: Vec<i32> = merge_vecs(vec![vec![], vec![], vec![]]);
    assert!(merged.is_empty());
}

#[test]
fn test_merge_vecs_mixed_empty() {
    let merged = merge_vecs(vec![vec![1], vec![], vec![2, 3], vec![]]);
    assert_eq!(merged, vec![1, 2, 3]);
}

#[test]
fn test_merge_vecs_single_vec() {
    let merged = merge_vecs(vec![vec![1, 2, 3, 4, 5]]);
    assert_eq!(merged, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_merge_vecs_strings() {
    let merged = merge_vecs(vec![
        vec!["hello".to_string()],
        vec!["world".to_string()],
    ]);
    assert_eq!(merged, vec!["hello", "world"]);
}

#[test]
fn test_merge_vecs_preserves_order() {
    let merged = merge_vecs(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    assert_eq!(merged, vec![1, 2, 3, 4, 5, 6]);
}

// ==================== chain_and_collect tests ====================

#[test]
fn test_chain_and_collect_basic() {
    let chained = chain_and_collect(vec![1, 2], vec![3, 4]);
    assert_eq!(chained, vec![1, 2, 3, 4]);
}

#[test]
fn test_chain_and_collect_first_empty() {
    let chained = chain_and_collect(vec![], vec![1, 2, 3]);
    assert_eq!(chained, vec![1, 2, 3]);
}

#[test]
fn test_chain_and_collect_second_empty() {
    let chained = chain_and_collect(vec![1, 2, 3], vec![]);
    assert_eq!(chained, vec![1, 2, 3]);
}

#[test]
fn test_chain_and_collect_both_empty() {
    let chained: Vec<i32> = chain_and_collect(vec![], vec![]);
    assert!(chained.is_empty());
}

#[test]
fn test_chain_and_collect_different_lengths() {
    let chained = chain_and_collect(vec![1], vec![2, 3, 4, 5]);
    assert_eq!(chained, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_chain_and_collect_strings() {
    let chained = chain_and_collect(
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string()],
    );
    assert_eq!(chained, vec!["a", "b", "c"]);
}

// ==================== Integration tests ====================

#[test]
fn test_roundtrip_vec_hashset_sorted_vec() {
    let original = vec![5, 2, 8, 1, 9, 3, 5, 2];
    let set = vec_to_hashset(original);
    let sorted = hashset_to_sorted_vec(set);
    assert_eq!(sorted, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_hashmap_roundtrip() {
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map = pairs_to_hashmap(pairs);
    let back_to_pairs = hashmap_to_pairs(map);
    assert_eq!(back_to_pairs.len(), 3);
    assert!(back_to_pairs.contains(&("a", 1)));
    assert!(back_to_pairs.contains(&("b", 2)));
    assert!(back_to_pairs.contains(&("c", 3)));
}

#[test]
fn test_chain_multiple_with_merge() {
    let first = chain_and_collect(vec![1, 2], vec![3, 4]);
    let second = chain_and_collect(vec![5, 6], vec![7, 8]);
    let merged = merge_vecs(vec![first, second]);
    assert_eq!(merged, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_dedupe_and_sort_workflow() {
    // Common workflow: remove duplicates and sort
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let unique_sorted: Vec<_> = vec_to_btreeset(data).into_iter().collect();
    assert_eq!(unique_sorted, vec![1, 2, 3, 4, 5, 6, 9]);
}

#[test]
fn test_group_by_workflow() {
    // Simulate grouping items
    let items = vec![
        ("fruit", "apple"),
        ("vegetable", "carrot"),
        ("fruit", "banana"),
    ];
    let map = pairs_to_hashmap(items);
    // Note: duplicates are overwritten, so "fruit" maps to "banana"
    assert_eq!(map.get("fruit"), Some(&"banana"));
    assert_eq!(map.get("vegetable"), Some(&"carrot"));
}
