use hashmap_advanced::*;
use std::collections::HashMap;

// ==================== create_with_capacity tests ====================

#[test]
fn create_with_capacity_zero() {
    let map = create_with_capacity(0);
    assert_eq!(map.len(), 0);
}

#[test]
fn create_with_capacity_small() {
    let map = create_with_capacity(10);
    assert!(map.capacity() >= 10);
    assert_eq!(map.len(), 0);
}

#[test]
fn create_with_capacity_large() {
    let map = create_with_capacity(1000);
    assert!(map.capacity() >= 1000);
    assert_eq!(map.len(), 0);
}

#[test]
fn create_with_capacity_is_empty() {
    let map = create_with_capacity(100);
    assert!(map.is_empty());
}

// ==================== reserve_additional tests ====================

#[test]
fn reserve_additional_empty_map() {
    let mut map: HashMap<String, i32> = HashMap::new();
    reserve_additional(&mut map, 100);
    assert!(map.capacity() >= 100);
}

#[test]
fn reserve_additional_with_existing_elements() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    reserve_additional(&mut map, 100);
    assert!(map.capacity() >= 102);
    assert_eq!(map.len(), 2);
}

#[test]
fn reserve_additional_zero() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    let old_capacity = map.capacity();
    reserve_additional(&mut map, 0);
    assert!(map.capacity() >= old_capacity);
}

#[test]
fn reserve_additional_preserves_data() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key".to_string(), 42);
    reserve_additional(&mut map, 1000);
    assert_eq!(map["key"], 42);
}

// ==================== shrink_map tests ====================

#[test]
fn shrink_map_large_to_small() {
    let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
    map.insert("a".to_string(), 1);
    let old_capacity = map.capacity();
    shrink_map(&mut map);
    assert!(map.capacity() < old_capacity);
}

#[test]
fn shrink_map_preserves_data() {
    let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);
    shrink_map(&mut map);
    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);
    assert_eq!(map["c"], 3);
}

#[test]
fn shrink_map_empty() {
    let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
    shrink_map(&mut map);
    // Should still work on empty map
    assert!(map.is_empty());
}

#[test]
fn shrink_map_already_minimal() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    shrink_map(&mut map);
    // Should not crash on already-small map
    assert_eq!(map.len(), 1);
}

// ==================== bulk_insert tests ====================

#[test]
fn bulk_insert_empty() {
    let items: [(&str, i32); 0] = [];
    let map = bulk_insert(&items);
    assert!(map.is_empty());
}

#[test]
fn bulk_insert_single() {
    let items = [("a", 1)];
    let map = bulk_insert(&items);
    assert_eq!(map.len(), 1);
    assert_eq!(map["a"], 1);
}

#[test]
fn bulk_insert_multiple() {
    let items = [("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5)];
    let map = bulk_insert(&items);
    assert_eq!(map.len(), 5);
    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);
    assert_eq!(map["c"], 3);
    assert_eq!(map["d"], 4);
    assert_eq!(map["e"], 5);
}

#[test]
fn bulk_insert_duplicate_keys() {
    let items = [("a", 1), ("a", 2)];
    let map = bulk_insert(&items);
    assert_eq!(map.len(), 1);
    assert_eq!(map["a"], 2); // Last value wins
}

#[test]
fn bulk_insert_negative_values() {
    let items = [("neg", -100), ("pos", 100)];
    let map = bulk_insert(&items);
    assert_eq!(map["neg"], -100);
    assert_eq!(map["pos"], 100);
}

// ==================== get_capacity_stats tests ====================

#[test]
fn get_capacity_stats_empty() {
    let map: HashMap<String, i32> = HashMap::new();
    let (len, _cap) = get_capacity_stats(&map);
    assert_eq!(len, 0);
}

#[test]
fn get_capacity_stats_with_elements() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    let (len, cap) = get_capacity_stats(&map);
    assert_eq!(len, 2);
    assert!(cap >= 2);
}

#[test]
fn get_capacity_stats_with_capacity() {
    let map = create_with_capacity(100);
    let (len, cap) = get_capacity_stats(&map);
    assert_eq!(len, 0);
    assert!(cap >= 100);
}

#[test]
fn get_capacity_stats_after_bulk_insert() {
    let items = [("a", 1), ("b", 2), ("c", 3)];
    let map = bulk_insert(&items);
    let (len, cap) = get_capacity_stats(&map);
    assert_eq!(len, 3);
    assert!(cap >= 3);
}

// ==================== clear_and_shrink tests ====================

#[test]
fn clear_and_shrink_basic() {
    let mut map = bulk_insert(&[("a", 1), ("b", 2), ("c", 3)]);
    clear_and_shrink(&mut map);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn clear_and_shrink_large_map() {
    let items: Vec<(&str, i32)> = (0..100).map(|i| ("x", i)).collect();
    let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
    for (k, v) in &items {
        map.insert(k.to_string(), *v);
    }
    let old_capacity = map.capacity();
    clear_and_shrink(&mut map);
    assert!(map.is_empty());
    assert!(map.capacity() < old_capacity);
}

#[test]
fn clear_and_shrink_empty_map() {
    let mut map: HashMap<String, i32> = HashMap::with_capacity(100);
    clear_and_shrink(&mut map);
    assert!(map.is_empty());
}

#[test]
fn clear_and_shrink_can_reuse() {
    let mut map = bulk_insert(&[("a", 1), ("b", 2)]);
    clear_and_shrink(&mut map);
    map.insert("new".to_string(), 100);
    assert_eq!(map.len(), 1);
    assert_eq!(map["new"], 100);
}

// ==================== group_by_key tests ====================

#[test]
fn group_by_key_by_first_char() {
    let items = vec![
        ("apple".to_string(), 1),
        ("apricot".to_string(), 2),
        ("banana".to_string(), 3),
    ];
    let grouped = group_by_key(&items, |s| s.chars().next().unwrap().to_string());
    assert_eq!(grouped["a"], vec![1, 2]);
    assert_eq!(grouped["b"], vec![3]);
}

#[test]
fn group_by_key_by_length() {
    let items = vec![
        ("hi".to_string(), 1),
        ("hello".to_string(), 2),
        ("hey".to_string(), 3),
        ("world".to_string(), 4),
    ];
    let grouped = group_by_key(&items, |s| s.len().to_string());
    assert_eq!(grouped["2"], vec![1]);
    assert_eq!(grouped["5"], vec![2, 4]);
    assert_eq!(grouped["3"], vec![3]);
}

#[test]
fn group_by_key_empty() {
    let items: Vec<(String, i32)> = vec![];
    let grouped = group_by_key(&items, |s| s.to_string());
    assert!(grouped.is_empty());
}

#[test]
fn group_by_key_all_same_group() {
    let items = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
    ];
    let grouped = group_by_key(&items, |_| "same".to_string());
    assert_eq!(grouped["same"], vec![1, 2, 3]);
}

#[test]
fn group_by_key_preserves_order() {
    let items = vec![
        ("x1".to_string(), 1),
        ("x2".to_string(), 2),
        ("x3".to_string(), 3),
    ];
    let grouped = group_by_key(&items, |s| s.chars().next().unwrap().to_string());
    assert_eq!(grouped["x"], vec![1, 2, 3]);
}

#[test]
fn group_by_key_single_item() {
    let items = vec![("only".to_string(), 42)];
    let grouped = group_by_key(&items, |s| s.to_string());
    assert_eq!(grouped["only"], vec![42]);
}

// ==================== merge_with_capacity tests ====================

#[test]
fn merge_with_capacity_disjoint() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = [("b".to_string(), 2)].into();
    let merged = merge_with_capacity(vec![map1, map2]);
    assert_eq!(merged["a"], 1);
    assert_eq!(merged["b"], 2);
}

#[test]
fn merge_with_capacity_overlapping() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = [("a".to_string(), 2)].into();
    let merged = merge_with_capacity(vec![map1, map2]);
    assert_eq!(merged["a"], 3);
}

#[test]
fn merge_with_capacity_three_maps() {
    let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let map2: HashMap<String, i32> = [("a".to_string(), 2), ("b".to_string(), 10)].into();
    let map3: HashMap<String, i32> = [("a".to_string(), 3), ("c".to_string(), 100)].into();
    let merged = merge_with_capacity(vec![map1, map2, map3]);
    assert_eq!(merged["a"], 6);
    assert_eq!(merged["b"], 10);
    assert_eq!(merged["c"], 100);
}

#[test]
fn merge_with_capacity_empty_maps() {
    let map1: HashMap<String, i32> = HashMap::new();
    let map2: HashMap<String, i32> = HashMap::new();
    let merged = merge_with_capacity(vec![map1, map2]);
    assert!(merged.is_empty());
}

#[test]
fn merge_with_capacity_single_map() {
    let map: HashMap<String, i32> = [("a".to_string(), 1)].into();
    let merged = merge_with_capacity(vec![map]);
    assert_eq!(merged["a"], 1);
}

#[test]
fn merge_with_capacity_no_maps() {
    let merged = merge_with_capacity(vec![]);
    assert!(merged.is_empty());
}

#[test]
fn merge_with_capacity_negative_values() {
    let map1: HashMap<String, i32> = [("a".to_string(), -5)].into();
    let map2: HashMap<String, i32> = [("a".to_string(), 3)].into();
    let merged = merge_with_capacity(vec![map1, map2]);
    assert_eq!(merged["a"], -2);
}

#[test]
fn merge_with_capacity_large_maps() {
    let map1: HashMap<String, i32> = (0..100)
        .map(|i| (format!("key_{}", i), i))
        .collect();
    let map2: HashMap<String, i32> = (50..150)
        .map(|i| (format!("key_{}", i), i))
        .collect();
    let merged = merge_with_capacity(vec![map1, map2]);
    assert_eq!(merged["key_0"], 0);
    assert_eq!(merged["key_50"], 100); // 50 + 50
    assert_eq!(merged["key_149"], 149);
}

// ==================== Integration tests ====================

#[test]
fn integration_capacity_workflow() {
    // Create with capacity
    let mut map = create_with_capacity(100);
    assert!(map.capacity() >= 100);

    // Insert some items
    for i in 0..50 {
        map.insert(format!("key_{}", i), i);
    }
    let (len, cap) = get_capacity_stats(&map);
    assert_eq!(len, 50);
    assert!(cap >= 100);

    // Reserve more
    reserve_additional(&mut map, 200);
    assert!(map.capacity() >= 250);

    // Shrink back
    shrink_map(&mut map);

    // Clear and shrink
    clear_and_shrink(&mut map);
    assert!(map.is_empty());
}

#[test]
fn integration_bulk_operations() {
    let items: Vec<(&str, i32)> = vec![
        ("alpha", 1),
        ("beta", 2),
        ("gamma", 3),
        ("delta", 4),
    ];
    let map = bulk_insert(&items);
    let (len, _) = get_capacity_stats(&map);
    assert_eq!(len, 4);
}

#[test]
fn integration_grouping_and_merging() {
    // Create grouped data
    let items1 = vec![
        ("team_a".to_string(), 10),
        ("team_b".to_string(), 20),
    ];
    let items2 = vec![
        ("team_a".to_string(), 15),
        ("team_c".to_string(), 25),
    ];

    let grouped1 = group_by_key(&items1, |s| s.split('_').next().unwrap().to_string());
    let grouped2 = group_by_key(&items2, |s| s.split('_').next().unwrap().to_string());

    assert_eq!(grouped1["team"], vec![10, 20]);
    assert_eq!(grouped2["team"], vec![15, 25]);
}

#[test]
fn integration_merge_multiple_sources() {
    // Simulate merging data from multiple sources
    let source1: HashMap<String, i32> = [
        ("sales".to_string(), 100),
        ("returns".to_string(), 10),
    ].into();

    let source2: HashMap<String, i32> = [
        ("sales".to_string(), 150),
        ("expenses".to_string(), 50),
    ].into();

    let source3: HashMap<String, i32> = [
        ("sales".to_string(), 200),
        ("returns".to_string(), 5),
    ].into();

    let total = merge_with_capacity(vec![source1, source2, source3]);
    assert_eq!(total["sales"], 450);
    assert_eq!(total["returns"], 15);
    assert_eq!(total["expenses"], 50);
}

#[test]
fn integration_memory_efficient_processing() {
    // Simulate processing a batch of data efficiently
    let mut map = create_with_capacity(1000);

    // Bulk insert
    for i in 0..500 {
        map.insert(format!("item_{}", i), i);
    }

    // Remove half
    for i in 0..250 {
        map.remove(&format!("item_{}", i));
    }

    // Shrink to reclaim memory
    let (len, cap_before) = get_capacity_stats(&map);
    assert_eq!(len, 250);
    shrink_map(&mut map);
    let (_, cap_after) = get_capacity_stats(&map);
    assert!(cap_after <= cap_before);
}
