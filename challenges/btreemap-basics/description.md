`BTreeMap<K, V>` is Rust's implementation of an ordered map based on a B-Tree data structure. Unlike `HashMap`, which stores entries in arbitrary order for O(1) average lookups, `BTreeMap` keeps its keys sorted and provides O(log n) operations. This ordering comes with a powerful advantage: you can efficiently iterate over keys in sorted order and perform range queries.

The `BTreeMap` type is found in `std::collections` and requires that keys implement the `Ord` trait (total ordering). This means you can use it with types that have a natural ordering like integers, strings, and any custom types that implement `Ord`.

One of the most useful features of `BTreeMap` is the `.range()` method, which allows you to iterate over a subset of entries within a specified key range. This is particularly useful for time-series data, finding entries between two values, or implementing features like autocomplete with prefix matching.

```rust
use std::collections::BTreeMap;

let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 87);
scores.insert("Charlie", 92);

// Keys are always sorted when iterating
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
// Prints: Alice: 95, Bob: 87, Charlie: 92 (alphabetical order)

// Range queries
use std::ops::Bound::{Included, Excluded};
for (name, score) in scores.range("B".."D") {
    println!("{}: {}", name, score);
}
// Prints: Bob: 87, Charlie: 92
```

## Your Task

Implement the following functions for working with `BTreeMap`:

1. `create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32>` - Create a BTreeMap from a slice of key-value pairs

2. `get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32>` - Get a value by key, returning `None` if the key doesn't exist

3. `get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String>` - Return all keys in sorted order

4. `get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32>` - Return all values in the order of their sorted keys

5. `get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)>` - Return all key-value pairs where the key is >= start and < end (half-open range)

6. `get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)>` - Return the first (smallest key) entry

7. `get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)>` - Return the last (largest key) entry

## Examples

```rust
use std::collections::BTreeMap;

// create_sorted_map
let pairs = vec![
    ("cherry".to_string(), 3),
    ("apple".to_string(), 1),
    ("banana".to_string(), 2),
];
let map = create_sorted_map(&pairs);
assert_eq!(map.len(), 3);

// get_value
assert_eq!(get_value(&map, "apple"), Some(1));
assert_eq!(get_value(&map, "grape"), None);

// get_keys_in_order - returns keys sorted alphabetically
let keys = get_keys_in_order(&map);
assert_eq!(keys, vec!["apple", "banana", "cherry"]);

// get_values_in_key_order - returns values in order of sorted keys
let values = get_values_in_key_order(&map);
assert_eq!(values, vec![1, 2, 3]);

// get_range - half-open range [start, end)
let range = get_range(&map, "apple", "cherry");
assert_eq!(range, vec![
    ("apple".to_string(), 1),
    ("banana".to_string(), 2),
]);

// get_first - smallest key
assert_eq!(
    get_first(&map),
    Some(("apple".to_string(), 1))
);

// get_last - largest key
assert_eq!(
    get_last(&map),
    Some(("cherry".to_string(), 3))
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `create_sorted_map`, you can iterate over the slice and insert each pair, or use `.iter().cloned().collect()`
- For `get_value`, use the `.get()` method which returns an `Option<&V>`, then use `.copied()` to get `Option<V>`
- For `get_keys_in_order`, iterate over `.keys()` and collect into a Vec
- For `get_values_in_key_order`, iterate over `.values()` and collect into a Vec
- For `get_range`, use the `.range()` method with a range pattern like `start..end`
- For `get_first` and `get_last`, use `.first_key_value()` and `.last_key_value()` methods, which return `Option<(&K, &V)>`
- Remember that `BTreeMap` iteration methods return references, so you may need to clone values when collecting

</details>
