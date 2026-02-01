`BTreeSet` is Rust's ordered set implementation, backed by a B-tree data structure. Unlike `HashSet` which stores elements in an arbitrary order, `BTreeSet` maintains its elements in sorted order according to the `Ord` trait. This ordering enables powerful range-based operations that are not possible with hash-based sets.

The key advantage of `BTreeSet` over `HashSet` is the `.range()` method, which allows you to efficiently iterate over a subset of elements within a specified range. This is particularly useful for time-based queries (finding events within a date range), numeric filtering (values between min and max), or any scenario where you need to work with ordered subsets of data.

Range queries in Rust use the `std::ops::Bound` enum to specify inclusive, exclusive, or unbounded endpoints. You can use the convenient range syntax (`..`, `..=`, `start..end`, etc.) or explicit `Bound::Included`, `Bound::Excluded`, and `Bound::Unbounded` variants for more control.

```rust
use std::collections::BTreeSet;
use std::ops::Bound;

let numbers: BTreeSet<i32> = [
    1, 3, 5, 7, 9, 11, 13
].into_iter().collect();

// Range from 3 to 9 (exclusive end)
let range: Vec<_> = numbers.range(3..9).copied().collect();
assert_eq!(range, vec![3, 5, 7]);

// Range from 5 to 11 (inclusive end)
let range: Vec<_> = numbers
    .range(5..=11)
    .copied()
    .collect();
assert_eq!(range, vec![5, 7, 9, 11]);

// Everything up to 7 (exclusive)
let range: Vec<_> = numbers.range(..7).copied().collect();
assert_eq!(range, vec![1, 3, 5]);

// Everything from 9 onwards
let range: Vec<_> = numbers.range(9..).copied().collect();
assert_eq!(range, vec![9, 11, 13]);
```

## Your Task

Implement the following functions for working with `BTreeSet` ranges:

1. `create_number_set(numbers: &[i32]) -> BTreeSet<i32>`
   - Create a BTreeSet from a slice of numbers.

2. `get_range(
    set: &BTreeSet<i32>,
    start: i32,
    end: i32
) -> Vec<i32>`
   - Return all elements in the range [start, end)
     (inclusive start, exclusive end).

3. `get_range_inclusive(
    set: &BTreeSet<i32>,
    start: i32,
    end: i32
) -> Vec<i32>`
   - Return all elements in the range [start, end]
     (both inclusive).

4. `get_elements_before(
    set: &BTreeSet<i32>,
    threshold: i32
) -> Vec<i32>`
   - Return all elements strictly less than the
     threshold.

5. `get_elements_from(
    set: &BTreeSet<i32>,
    threshold: i32
) -> Vec<i32>`
   - Return all elements greater than or equal to
     the threshold.

6. `count_in_range(
    set: &BTreeSet<i32>,
    start: i32,
    end: i32
) -> usize`
   - Count how many elements fall within the
     range [start, end].

7. `find_closest_less_than(
    set: &BTreeSet<i32>,
    value: i32
) -> Option<i32>`
   - Find the largest element in the set that is
     strictly less than the given value. Return
     `None` if no such element exists.

8. `find_closest_greater_than(
    set: &BTreeSet<i32>,
    value: i32
) -> Option<i32>`
   - Find the smallest element in the set that is
     strictly greater than the given value. Return
     `None` if no such element exists.

## Examples

```rust
use std::collections::BTreeSet;

let set = create_number_set(&[1, 3, 5, 7, 9, 11, 13, 15]);

// get_range: [5, 12) -> 5, 7, 9, 11
assert_eq!(
    get_range(&set, 5, 12),
    vec![5, 7, 9, 11]
);

// get_range_inclusive: [5, 11] -> 5, 7, 9, 11
assert_eq!(
    get_range_inclusive(&set, 5, 11),
    vec![5, 7, 9, 11]
);

// get_elements_before: < 7 -> 1, 3, 5
assert_eq!(
    get_elements_before(&set, 7),
    vec![1, 3, 5]
);

// get_elements_from: >= 9 -> 9, 11, 13, 15
assert_eq!(
    get_elements_from(&set, 9),
    vec![9, 11, 13, 15]
);

// count_in_range: [3, 11] -> 5 elements (3, 5, 7, 9, 11)
assert_eq!(count_in_range(&set, 3, 11), 5);

// find_closest_less_than: largest element < 10 is 9
assert_eq!(
    find_closest_less_than(&set, 10),
    Some(9)
);

// find_closest_greater_than: smallest element > 10 is 11
assert_eq!(
    find_closest_greater_than(&set, 10),
    Some(11)
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `.range(start..end)` for half-open ranges
  and `.range(start..=end)` for inclusive ranges
- The range method returns an iterator, so
  you'll need to `.copied()` or `.cloned()`
  and `.collect()` the results
- For unbounded ranges, use `..threshold` or `threshold..`
- For `count_in_range`, you can use `.range(...).count()` directly
- For `find_closest_less_than`, think about
  using `.range(..value)` and getting the last
  element
- For `find_closest_greater_than`, think about
  using `.range((Bound::Excluded(value),
Bound::Unbounded))` and getting the first
  element
- Remember that `BTreeSet::range` requires the
  bounds to be valid (start <= end for proper
  ranges)

</details>
