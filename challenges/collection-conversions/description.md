One of Rust's most powerful features is the seamless conversion between different collection types through the `Iterator` trait. The `.collect()` method, combined with `FromIterator` and `IntoIterator` traits, enables you to transform data between `Vec`, `HashSet`, `BTreeSet`, `HashMap`, `BTreeMap`, and other collections with minimal boilerplate.

The key insight is that most collections implement `IntoIterator` (allowing them to be iterated) and `FromIterator` (allowing them to be built from an iterator). This means any collection can be converted to any other collection through an iterator intermediate step: `source.into_iter().collect()`. The target type is inferred from context or can be specified with the turbofish syntax `::<TargetType>`.

Understanding collection conversions is essential for writing idiomatic Rust. You'll often need to deduplicate data (Vec to HashSet), sort unique elements (Vec to BTreeSet), or transform key-value data (Vec of tuples to HashMap). These conversions are zero-cost abstractions - the compiler optimizes them into efficient code.

```rust
use std::collections::{HashSet, BTreeSet, HashMap};

// Vec to HashSet (removes duplicates)
let numbers = vec![1, 2, 2, 3, 3, 3];
let unique: HashSet<i32> = numbers.into_iter().collect();
assert_eq!(unique.len(), 3);

// Vec of tuples to HashMap
let pairs = vec![("a", 1), ("b", 2)];
let map: HashMap<&str, i32> = pairs.into_iter().collect();
assert_eq!(map.get("a"), Some(&1));

// HashSet to sorted Vec (via BTreeSet)
let set: HashSet<i32> = [3, 1, 2].into_iter().collect();
let sorted: Vec<i32> = set
    .into_iter()
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
assert_eq!(sorted, vec![1, 2, 3]);
```

## Your Task

Implement the following collection conversion functions:

1. `vec_to_hashset<T>(vec: Vec<T>) -> HashSet<T>`
   - Convert a Vec to a HashSet, removing
     duplicates. T must implement `Eq` and `Hash`.

2. `vec_to_btreeset<T>(vec: Vec<T>) -> BTreeSet<T>`
   - Convert a Vec to a BTreeSet, removing
     duplicates and sorting. T must implement
     `Ord`.

3. `hashset_to_sorted_vec<T>(set: HashSet<T>) -> Vec<T>`
   - Convert a HashSet to a sorted Vec. `T` must
     implement `Ord`.

4. `pairs_to_hashmap<K, V>(
    pairs: Vec<(K, V)>
) -> HashMap<K, V>`
   - Convert a Vec of key-value tuples to a
     HashMap. K must implement `Eq` and `Hash`.
     If duplicate keys exist, later values
     overwrite earlier ones.

5. `pairs_to_btreemap<K, V>(
    pairs: Vec<(K, V)>
) -> BTreeMap<K, V>`
   - Convert a Vec of key-value tuples to a
     BTreeMap (sorted by key). K must implement
     `Ord`. If duplicate keys exist, later values
     overwrite earlier ones.

6. `hashmap_to_pairs<K, V>(
    map: HashMap<K, V>
) -> Vec<(K, V)>`
   - Convert a HashMap to a Vec of key-value
     tuples. Order is not guaranteed.

7. `merge_vecs<T>(vecs: Vec<Vec<T>>) -> Vec<T>` - Flatten multiple Vecs into a single Vec, preserving order.

8. `chain_and_collect<T>(first: Vec<T>, second: Vec<T>) -> Vec<T>` - Chain two Vecs together into one, with first's elements followed by second's.

## Examples

```rust
use std::collections::{
    HashSet,
    BTreeSet,
    HashMap,
    BTreeMap
};

// vec_to_hashset
let unique = vec_to_hashset(vec![1, 2, 2, 3, 3, 3]);
assert_eq!(unique.len(), 3);
assert!(unique.contains(&1));

// vec_to_btreeset
let sorted_unique = vec_to_btreeset(vec![3, 1, 2, 2]);
let as_vec: Vec<_> = sorted_unique
    .into_iter()
    .collect();
assert_eq!(as_vec, vec![1, 2, 3]);

// hashset_to_sorted_vec
let set: HashSet<i32> = [3, 1, 4, 1, 5]
    .into_iter()
    .collect();
let sorted = hashset_to_sorted_vec(set);
assert_eq!(sorted, vec![1, 3, 4, 5]);

// pairs_to_hashmap
let map = pairs_to_hashmap(vec![("a", 1), ("b", 2)]);
assert_eq!(map.get("a"), Some(&1));

// pairs_to_btreemap
let map = pairs_to_btreemap(vec![
    ("c", 3),
    ("a", 1),
    ("b", 2)
]);
let keys: Vec<_> = map.keys().collect();
// sorted!
assert_eq!(keys, vec![&"a", &"b", &"c"]);

// hashmap_to_pairs
let mut map = HashMap::new();
map.insert("x", 10);
let pairs = hashmap_to_pairs(map);
assert_eq!(pairs.len(), 1);

// merge_vecs
let merged = merge_vecs(vec![
    vec![1, 2],
    vec![3, 4],
    vec![5]
]);
assert_eq!(merged, vec![1, 2, 3, 4, 5]);

// chain_and_collect
let chained = chain_and_collect(
    vec![1, 2],
    vec![3, 4]
);
assert_eq!(chained, vec![1, 2, 3, 4]);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For all conversions, the pattern is usually
  `source.into_iter().collect()`
- Use `.into_iter()` when you want to consume
  the source collection
- Type inference usually works, but you can use
  turbofish `::<TargetType>` when needed
- For `hashset_to_sorted_vec`, you'll need to
  sort after collecting to a Vec, or collect
  through a BTreeSet first
- For `merge_vecs`, consider using
  `.into_iter().flatten().collect()`
- For `chain_and_collect`, look at the `.chain()` iterator adapter
- Remember: `HashMap` and `HashSet` require
  `Hash + Eq` bounds, while `BTreeMap` and
  `BTreeSet` require `Ord`

</details>
