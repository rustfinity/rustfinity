The `Hash` trait in Rust enables types to be hashed, which is essential for using them as keys in `HashMap` and storing them in `HashSet`. When you implement `Hash`, you're defining how a value is converted into a hash code used for efficient lookups.

The `Hash` trait works closely with `Eq`: if two values are equal (`a == b`), they must produce the same hash. This is a critical invariant! If equal values hash differently, collections like `HashMap` will behave incorrectly. The standard library provides a derive macro `#[derive(Hash)]` that automatically implements `Hash` for structs and enums whose fields all implement `Hash`.

Sometimes you need manual implementations, especially when:

- Only some fields should contribute to the hash (matching a custom `PartialEq`)
- You want case-insensitive string hashing
- You need to normalize values before hashing

## Your Task

Implement several types demonstrating `Hash` trait patterns:

1. **`Point`**: A 2D point with `x` and `y` coordinates (`i32`). Use derived `Hash`, `Eq`, and `PartialEq`.

2. **`UserId`**: A newtype wrapper around `u64`.
   Use derived traits to make it usable as a `HashMap` key.

3. **`CaseInsensitiveString`**: A string wrapper where
   equality and hashing are case-insensitive. Implement `Hash`
   manually by hashing the lowercase form of the string.
   This must match the `PartialEq`/`Eq` implementation.

4. **`Document`**: A document with `id` (`u64`), `title` (`String`), and `content` (`String`). Two documents are equal if they have the same `id`, regardless of title or content. Implement `Hash` to only hash the `id` field (matching the equality semantics).

5. **`Rgb`**: A color with `r`, `g`, `b` fields (`u8`). Use derived `Hash` for standard behavior.

Implement the following utility functions:

1. **`count_unique<T: Hash + Eq>(items: &[T]) -> usize`**:
   Returns the count of unique elements using a `HashSet`.

2. **`find_duplicates<T: Hash + Eq + Clone>(items: &[T]) -> Vec<T>`**: Returns a vector of elements that appear more than once.

3. **`group_by_hash<T, K, F>(items: &[T], key_fn: F) -> HashMap<K, Vec<T>>`** where:
   - `T: Hash + Eq + Clone`
   - `K: Hash + Eq`
   - `F: Fn(&T) -> K`

   Groups items by a key function, returning a `HashMap` where each key maps to a vector of items with that key.

## Examples

```rust
use std::collections::{HashMap, HashSet};

// Point with derived Hash
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
let mut set: HashSet<Point> = HashSet::new();
set.insert(p1);
assert!(!set.insert(p2)); // Returns false, p2 is duplicate

// CaseInsensitiveString with manual Hash
let s1 = CaseInsensitiveString::new("Hello");
let s2 = CaseInsensitiveString::new("HELLO");
let mut set: HashSet<CaseInsensitiveString> =
    HashSet::new();
set.insert(s1);
// Same hash due to case-insensitivity
assert!(!set.insert(s2));

// Document with id-only hashing
let doc1 = Document::new(1, "Draft", "Content v1");
let doc2 = Document::new(1, "Final", "Content v2");
let mut set: HashSet<Document> = HashSet::new();
set.insert(doc1);
assert!(!set.insert(doc2)); // Same id, same hash

// Utility functions
assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
assert_eq!(
    find_duplicates(&[1, 2, 2, 3, 3, 3]),
    vec![2, 3]
);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- To implement `Hash` manually, use a `Hasher`:

  ```rust
  use std::hash::{Hash, Hasher};

  impl Hash for MyType {
      fn hash<H: Hasher>(&self, state: &mut H) {
          self.field.hash(state);
      }
  }
  ```

- The `Hash` implementation MUST be consistent with `Eq`: equal values must hash the same.

- For case-insensitive hashing, convert to lowercase before hashing.

- Use `HashSet::from_iter()` or `collect()` to create a set from an iterator.

- For `find_duplicates`, consider using a `HashMap` to count occurrences.

- For `group_by_hash`, use the Entry API: `map.entry(key).or_default().push(item)`.

</details>
