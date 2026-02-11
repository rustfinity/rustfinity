While Rust's `HashMap` provides excellent performance out of the box, understanding its advanced features can help you write more efficient code for performance-critical applications. This challenge explores capacity management, custom hashers, and performance optimization techniques.

By default, `HashMap` dynamically grows as you add elements, but this growth involves memory allocation and rehashing existing entries - both expensive operations. When you know approximately how many elements you'll store, you can pre-allocate capacity with `HashMap::with_capacity()` or `reserve()`. Conversely, after removing many elements, `shrink_to_fit()` releases unused memory.

The default hasher (`RandomState`) is designed to be secure against HashDoS attacks, but for scenarios where security isn't a concern (like integer keys or performance-critical code), a faster hasher can significantly improve performance. Rust allows you to specify custom hashers using `HashMap::with_hasher()` or by building a `HashMap` with a `BuildHasher` implementation.

```rust
use std::collections::HashMap;

// Pre-allocating capacity when you know the size
let mut scores: HashMap<String, i32> =
    HashMap::with_capacity(100);
assert!(scores.capacity() >= 100);

// Adding capacity to an existing map
// Reserve space for at least 50 more entries
scores.reserve(50);

// Reclaiming memory after removing elements
for i in 0..100 {
    scores.insert(format!("player_{}", i), i);
}
for i in 0..90 {
    scores.remove(&format!("player_{}", i));
}
// Release unused memory
scores.shrink_to_fit();
```

## Your Task

Implement the following functions demonstrating HashMap performance optimization:

1. `create_with_capacity(capacity: usize) -> HashMap<String, i32>` - Create a HashMap with at least the specified capacity. The actual capacity may be larger due to implementation details.

2. `reserve_additional(map: &mut HashMap<String, i32>, additional: usize)` - Reserve space for at least `additional` more elements without reallocating if there's already enough capacity.

3. `shrink_map(map: &mut HashMap<String, i32>)` - Shrink the map's capacity to fit its current number of elements as closely as possible.

4. `bulk_insert(items: &[(&str, i32)]) -> HashMap<String, i32>` - Efficiently insert multiple items by pre-allocating the right capacity. Return the populated HashMap.

5. `get_capacity_stats(map: &HashMap<String, i32>) -> (usize, usize)` - Return a tuple of (length, capacity) for the given map.

6. `clear_and_shrink(map: &mut HashMap<String, i32>)` - Remove all elements and shrink the capacity to minimum.

7. `group_by_key<F>(items: &[(String, i32)], key_fn: F) -> HashMap<String, Vec<i32>>` - Group items by a key function, pre-allocating capacity. The `key_fn` takes a reference to the string key and returns the grouping key.

8. `merge_with_capacity(maps: Vec<HashMap<String, i32>>) -> HashMap<String, i32>` - Merge multiple HashMaps into one, pre-allocating capacity for efficiency. If a key appears in multiple maps, sum the values.

## Examples

```rust
use std::collections::HashMap;

// create_with_capacity
let map = create_with_capacity(100);
assert!(map.capacity() >= 100);
assert_eq!(map.len(), 0);

// reserve_additional
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("a".to_string(), 1);
reserve_additional(&mut map, 100);
assert!(map.capacity() >= 101);

// shrink_map
let mut map: HashMap<String, i32> =
    HashMap::with_capacity(1000);
map.insert("a".to_string(), 1);
map.insert("b".to_string(), 2);
shrink_map(&mut map);
assert!(map.capacity() < 1000);

// bulk_insert
let items = [("a", 1), ("b", 2), ("c", 3)];
let map = bulk_insert(&items);
assert_eq!(map.len(), 3);
assert_eq!(map["a"], 1);

// get_capacity_stats
let map = create_with_capacity(50);
let (len, cap) = get_capacity_stats(&map);
assert_eq!(len, 0);
assert!(cap >= 50);

// clear_and_shrink
let mut map = bulk_insert(
    &[("a", 1), ("b", 2), ("c", 3)]
);
clear_and_shrink(&mut map);
assert_eq!(map.len(), 0);

// group_by_key
let items = vec![
    ("apple".to_string(), 1),
    ("apricot".to_string(), 2),
    ("banana".to_string(), 3),
];
let grouped = group_by_key(
    &items,
    |s| s.chars().next().unwrap().to_string()
);
assert_eq!(grouped["a"], vec![1, 2]);
assert_eq!(grouped["b"], vec![3]);

// merge_with_capacity
let map1: HashMap<String, i32> =
    [("a".to_string(), 1)].into();
let map2: HashMap<String, i32> = [
    ("a".to_string(), 2),
    ("b".to_string(), 3)
].into();
let merged = merge_with_capacity(vec![map1, map2]);
assert_eq!(merged["a"], 3);  // 1 + 2
assert_eq!(merged["b"], 3);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `HashMap::with_capacity(n)` to create a map with pre-allocated space for at least `n` elements
- The `reserve(additional)` method ensures space for `additional` more elements beyond the current length
- After `shrink_to_fit()`, the capacity will be close to the length, but may not be exactly equal
- Use `capacity()` to get the current capacity and `len()` to get the number of elements
- `clear()` removes all elements but doesn't change capacity - combine with `shrink_to_fit()` to release memory
- When pre-allocating for a bulk insert, the exact capacity is `items.len()`
- For `group_by_key`, you can estimate the number of groups or just let it grow naturally while focusing on the grouping logic
- For `merge_with_capacity`, sum the lengths of all input maps for a good capacity estimate

</details>
