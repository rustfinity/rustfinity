`HashSet<T>` is Rust's implementation of a hash set - a collection of unique values with O(1) average-time complexity for insertions, lookups, and removals. Unlike `Vec`, a `HashSet` automatically ensures that no duplicate values exist, making it perfect for deduplication and membership testing.

The `HashSet` type is found in `std::collections` and requires that elements implement both `Hash` and `Eq` traits. Most primitive types and many standard library types already implement these traits, so you can use them directly.

Beyond basic operations like `insert()` and `contains()`, `HashSet` provides powerful set-theoretic operations: `union()` (elements in either set), `intersection()` (elements in both sets), `difference()` (elements in one set but not the other), and `symmetric_difference()` (elements in exactly one set).

```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(2);  // Duplicate, won't be added
assert_eq!(set.len(), 2);
assert!(set.contains(&1));

// Creating from an iterator
let set: HashSet<_> = vec![1, 2, 3, 2, 1]
    .into_iter()
    .collect();
assert_eq!(set.len(), 3);  // Only unique values
```

## Your Task

Implement the following functions for working with `HashSet`:

1. `unique_elements(items: &[i32]) -> HashSet<i32>`
   - Return a set containing only the unique elements from
     the slice

2. `count_unique(items: &[i32]) -> usize`
   - Return the count of unique elements in the slice

3. `find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that appear in both sets (intersection)

4. `find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that appear in either set (union)

5. `find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements in `set1` that are not in `set2` (difference)

6. `find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32>` - Return elements that are in exactly one of the sets (symmetric difference)

7. `is_subset(potential_subset: &HashSet<i32>,
     potential_superset: &HashSet<i32>) -> bool`
   - Check if all elements of `potential_subset` are
     contained in `potential_superset`

## Examples

```rust
use std::collections::HashSet;

// unique_elements
let items = vec![1, 2, 3, 2, 1, 4, 3];
let unique = unique_elements(&items);
assert!(unique.contains(&1));
assert!(unique.contains(&4));
assert_eq!(unique.len(), 4);

// count_unique
assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);

// find_common (intersection)
let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
let common = find_common(&set1, &set2);
assert_eq!(
    common,
    [2, 3].into_iter().collect()
);

// find_all (union)
let all = find_all(&set1, &set2);
assert_eq!(
    all,
    [1, 2, 3, 4].into_iter().collect()
);

// find_difference
// In set1 but not set2
let diff = find_difference(&set1, &set2);
assert_eq!(
    diff,
    [1].into_iter().collect()
);

// find_symmetric_difference
// In exactly one set
let sym_diff = find_symmetric_difference(&set1, &set2);
assert_eq!(
    sym_diff,
    [1, 4].into_iter().collect()
);

// is_subset
let small: HashSet<i32> = [2, 3]
    .into_iter()
    .collect();
let large: HashSet<i32> = [1, 2, 3, 4]
    .into_iter()
    .collect();
assert!(is_subset(&small, &large));
assert!(!is_subset(&large, &small));
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `unique_elements`, you can iterate over the slice and collect into a `HashSet`, or use `.iter().cloned().collect()`
- For `count_unique`, you can create a set and return its length
- For `find_common`, use the `.intersection()` method, which returns an iterator
- For `find_all`, use the `.union()` method
- For `find_difference`, use the `.difference()` method
- For `find_symmetric_difference`, use the `.symmetric_difference()` method
- All set operations return iterators of references, so you'll need to clone or copy the values when collecting
- For `is_subset`, use the `.is_subset()` method

</details>
