Rust's iterator system provides powerful filtering methods that allow you to selectively process elements from a sequence. These methods are essential for data processing, validation, and transformation pipelines.

## Core Filtering Methods

### filter()

The `filter()` method retains only elements that satisfy a predicate:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// Keep only even numbers
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
assert_eq!(evens, vec![2, 4, 6]);

// Keep only numbers greater than 3
let large: Vec<i32> = numbers.iter().filter(|&x| *x > 3).cloned().collect();
assert_eq!(large, vec![4, 5, 6]);
```

### filter_map()

The `filter_map()` method combines filtering and mapping in one step. It applies a function that returns `Option<T>` and keeps only the `Some` values:

```rust
let strings = vec!["1", "two", "3", "four", "5"];

// Parse to integers, keeping only successful parses
let numbers: Vec<i32> = strings
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
assert_eq!(numbers, vec![1, 3, 5]);
```

### take_while() and skip_while()

These methods process elements based on a condition at the beginning of the sequence:

```rust
let numbers = vec![1, 2, 3, 4, 5, 1, 2];

// Take elements while they're less than 4 (stops at first failure)
let taken: Vec<i32> = numbers.iter().take_while(|&&x| x < 4).cloned().collect();
assert_eq!(taken, vec![1, 2, 3]);

// Skip elements while they're less than 4 (takes the rest after first failure)
let skipped: Vec<i32> = numbers.iter().skip_while(|&&x| x < 4).cloned().collect();
assert_eq!(skipped, vec![4, 5, 1, 2]);
```

Note that `take_while` and `skip_while` only check the condition at the beginning - once the condition fails (for `take_while`) or succeeds (for `skip_while`), the behavior changes for all remaining elements.

## Your Task

Implement the following functions that demonstrate iterator filtering patterns:

1. `filter_even(numbers: &[i32]) -> Vec<i32>` - Return only even numbers
2. `filter_by_predicate<F>(numbers: &[i32], predicate: F) -> Vec<i32>` - Return numbers matching a custom predicate (where `F: Fn(&i32) -> bool`)
3. `parse_valid_numbers(strings: &[&str]) -> Vec<i32>` - Parse strings to integers, keeping only valid parses
4. `filter_map_with<T, U, F>(items: &[T], f: F) -> Vec<U>` - Generic filter_map (where `T: Clone`, `F: Fn(T) -> Option<U>`)
5. `take_while_positive(numbers: &[i32]) -> Vec<i32>` - Take numbers while they're positive (> 0)
6. `skip_while_negative(numbers: &[i32]) -> Vec<i32>` - Skip numbers while they're negative (< 0)
7. `filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32>` - Keep numbers within range [min, max] inclusive
8. `first_matching<T: Clone, F>(items: &[T], predicate: F) -> Option<T>` - Find first element matching predicate (where `F: Fn(&T) -> bool`)

## Examples

```rust
// filter_even
assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);

// filter_by_predicate
assert_eq!(
    filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x > 3),
    vec![4, 5]
);

// parse_valid_numbers
assert_eq!(
    parse_valid_numbers(&["1", "hello", "3", "world"]),
    vec![1, 3]
);

// filter_map_with
let doubled: Vec<i32> = filter_map_with(
    &[1, 2, 3],
    |x| if x % 2 == 0 { Some(x * 2) } else { None }
);
// Only 2 is even, doubled to 4
assert_eq!(doubled, vec![4]);

// take_while_positive
assert_eq!(
    take_while_positive(&[3, 5, -1, 2, 4]),
    vec![3, 5]
);

// skip_while_negative
assert_eq!(
    skip_while_negative(&[-3, -1, 2, -4, 5]),
    vec![2, -4, 5]
);

// filter_in_range
assert_eq!(
    filter_in_range(&[1, 5, 10, 15, 20], 5, 15),
    vec![5, 10, 15]
);

// first_matching
assert_eq!(
    first_matching(&[1, 2, 3, 4, 5], |&x| x > 3),
    Some(4)
);
assert_eq!(
    first_matching(&[1, 2, 3], |&x| x > 10),
    None
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- `filter()` takes a closure that receives `&&T` when iterating over `&[T]`, so use `|&&x|` or `|&x|` patterns
- `filter_map()` combines filter and map - return `Some(value)` to keep, `None` to skip
- `take_while()` stops at the first element that doesn't match the predicate
- `skip_while()` skips until predicate fails, then takes all remaining elements
- Use `.cloned()` to convert from `&T` to `T` when collecting
- For `first_matching`, consider using `.find()` or `.filter().next()`
- Remember that `parse::<i32>()` returns a `Result`, use `.ok()` to convert to `Option`

</details>
