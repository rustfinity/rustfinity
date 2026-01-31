The Entry API is one of Rust's most elegant patterns for working with maps (`HashMap` and `BTreeMap`). It solves a common problem: when you want to insert a value into a map, but only if the key doesn't already exist, or when you want to modify an existing value, the naive approach requires two separate lookups - one to check if the key exists, and another to insert or modify. The Entry API combines these operations into a single, efficient lookup.

When you call `.entry(key)` on a map, you get an `Entry` enum that represents either a vacant slot (the key doesn't exist) or an occupied slot (the key exists with a value). This enum provides methods like `.or_insert()`, `.or_insert_with()`, `.or_default()`, and `.and_modify()` that let you handle both cases elegantly without redundant lookups.

The Entry API is particularly useful for building frequency counters, caches with computed values, and any scenario where you need to accumulate or aggregate values by key. It's a fundamental pattern that every Rust developer should master.

```rust
use std::collections::HashMap;

let mut word_counts: HashMap<&str, u32> = HashMap::new();
let words = [
    "apple", "banana", "apple", "cherry", "banana", "apple"
];

// Without Entry API (inefficient - two lookups per word):
for word in &words {
    if word_counts.contains_key(word) {
        *word_counts.get_mut(word).unwrap() += 1;
    } else {
        word_counts.insert(word, 1);
    }
}

// With Entry API (efficient - single lookup per word):
let mut word_counts: HashMap<&str, u32> = HashMap::new();
for word in &words {
    *word_counts.entry(word).or_insert(0) += 1;
}

assert_eq!(word_counts["apple"], 3);
assert_eq!(word_counts["banana"], 2);
assert_eq!(word_counts["cherry"], 1);
```

## Your Task

Implement the following functions that demonstrate various Entry API patterns:

1. `count_words(text: &str) -> HashMap<String, u32>` - Count the frequency of each word in the text. Words should be lowercased. Split on whitespace.

2. `group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>>` - Group words by their length. Each key is a length, and the value is a vector of words with that length.

3. `get_or_compute<F>(cache: &mut HashMap<String, i32>, key: &str, compute: F) -> i32` - Get a cached value or compute and cache it. The `compute` function should only be called if the key doesn't exist. Return the value (either cached or newly computed).

4. `increment_or_init(map: &mut HashMap<String, i32>, key: &str, init: i32)` - If the key exists, increment its value by 1. If it doesn't exist, insert the initial value.

5. `merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32>` - Merge two maps. If a key exists in both, sum the values.

6. `first_occurrence(items: &[&str]) -> HashMap<String, usize>` - Create a map where each key is an item and the value is the index of its first occurrence. Don't overwrite if the item was already seen.

7. `update_or_default(map: &mut HashMap<String, Vec<i32>>, key: &str, value: i32)` - Add a value to the vector at the given key. If the key doesn't exist, create a new vector with that value.

## Examples

```rust
use std::collections::HashMap;

// count_words
let counts = count_words(
    "the quick brown fox jumps over the lazy dog"
);
assert_eq!(counts["the"], 2);
assert_eq!(counts["quick"], 1);

// group_by_length
let grouped = group_by_length(
    &["hi", "hello", "hey", "world"]
);
assert_eq!(grouped[&2], vec!["hi"]);
assert_eq!(grouped[&5], vec!["hello", "world"]);
assert_eq!(grouped[&3], vec!["hey"]);

// get_or_compute
let mut cache: HashMap<String, i32> = HashMap::new();
let value = get_or_compute(
    &mut cache,
    "answer",
    || 42
);
assert_eq!(value, 42);
assert_eq!(cache["answer"], 42);

// increment_or_init
let mut scores: HashMap<String, i32> = HashMap::new();
// Inserts 100
increment_or_init(&mut scores, "alice", 100);
// Increments to 101
increment_or_init(&mut scores, "alice", 100);
assert_eq!(scores["alice"], 101);

// merge_maps
let map1: HashMap<String, i32> = [
    ("a".to_string(), 1),
    ("b".to_string(), 2)
].into();
let map2: HashMap<String, i32> = [
    ("b".to_string(), 3),
    ("c".to_string(), 4)
].into();
let merged = merge_maps(map1, map2);
assert_eq!(merged["a"], 1);
assert_eq!(merged["b"], 5);  // 2 + 3
assert_eq!(merged["c"], 4);

// first_occurrence
let firsts = first_occurrence(
    &["a", "b", "a", "c", "b"]
);
assert_eq!(firsts["a"], 0);
assert_eq!(firsts["b"], 1);
assert_eq!(firsts["c"], 3);

// update_or_default
let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
update_or_default(&mut groups, "evens", 2);
update_or_default(&mut groups, "evens", 4);
assert_eq!(groups["evens"], vec![2, 4]);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `count_words`, use `.entry(word).or_insert(0)` and then dereference and increment
- For `group_by_length`, use `.entry(len).or_default()` (Vec has a Default implementation) and then push
- For `get_or_compute`, use `.or_insert_with(compute)` to lazily evaluate the closure only when needed
- For `increment_or_init`, use `.and_modify(|v| *v += 1).or_insert(init)` to handle both cases
- For `merge_maps`, iterate over one map and use the Entry API to merge into the result
- For `first_occurrence`, use `.or_insert(index)` which only inserts if the key is vacant
- For `update_or_default`, use `.or_default()` to get an empty Vec, then push the value
- The `Entry::or_insert` method returns a mutable reference to the value, which you can then modify

</details>
