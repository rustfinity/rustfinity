When working with nested data structures like vectors of vectors, matrices, or trees, you often need to process all elements as a single flat sequence. Rust's iterator methods `.flatten()` and `.flat_map()` provide elegant solutions for these common patterns.

## flatten()

The `flatten()` method takes an iterator over nested iterables and produces a single flat iterator over all inner elements:

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];

// Flatten nested vectors into a single sequence
let flat: Vec<i32> = nested.into_iter().flatten().collect();
assert_eq!(flat, vec![1, 2, 3, 4, 5]);
```

`flatten()` also works with `Option` and `Result` iterators, effectively filtering out `None` and `Err` values:

```rust
let maybe_values = vec![
    Some(1),
    None,
    Some(3),
    None,
    Some(5)
];
let values: Vec<i32> = maybe_values
    .into_iter()
    .flatten()
    .collect();
assert_eq!(values, vec![1, 3, 5]);
```

## flat_map()

The `flat_map()` method combines `map()` and `flatten()` into a single operation. It applies a function that returns an iterator (or something that can be converted to one) and flattens the results:

```rust
let words = vec!["hello", "world"];

// Split each word into characters
let chars: Vec<char> = words
    .iter()
    .flat_map(|s| s.chars())
    .collect();
assert_eq!(
    chars,
    vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
);
```

This is equivalent to but more efficient than `.map(...).flatten()`.

## Your Task

Implement the following functions that demonstrate iterator flattening patterns:

1. `flatten_nested(nested: Vec<Vec<i32>>) -> Vec<i32>`
   - Flatten a vector of vectors into a single vector
2. `flatten_options(options: Vec<Option<i32>>) -> Vec<i32>`
   - Extract all `Some` values, discarding `None`
3. `flatten_results(results: Vec<Result<i32, &str>>) -> Vec<i32>`
   - Extract all `Ok` values, discarding `Err`
4. `chars_from_words(words: &[&str]) -> Vec<char>` - Get all characters from a slice of string slices
5. `expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32>`
   - Expand (start, end) tuples into sequences
     (inclusive ranges)
6. `flatten_to_depth_one(nested: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>>` - Flatten only the outer layer, keeping inner vectors intact
7. `words_from_lines(lines: &[&str]) -> Vec<String>` - Split each line into words, collecting all words
8. `flatten_and_filter<T, F>(nested: Vec<Vec<T>>,
                           predicate: F) -> Vec<T>`
   - Flatten then filter with a predicate
     (where `T: Clone`, `F: Fn(&T) -> bool`)

## Examples

```rust
// flatten_nested
assert_eq!(
    flatten_nested(vec![
        vec![1, 2],
        vec![3],
        vec![4, 5, 6]
    ]),
    vec![1, 2, 3, 4, 5, 6]
);

// flatten_options
assert_eq!(
    flatten_options(vec![Some(1), None, Some(3)]),
    vec![1, 3]
);

// flatten_results
let results: Vec<Result<i32, &str>> = vec![
    Ok(1),
    Err("bad"),
    Ok(3)
];
assert_eq!(flatten_results(results), vec![1, 3]);

// chars_from_words
assert_eq!(
    chars_from_words(&["hi", "there"]),
    vec!['h', 'i', 't', 'h', 'e', 'r', 'e']
);

// expand_ranges
assert_eq!(
    expand_ranges(&[(1, 3), (5, 6)]),
    vec![1, 2, 3, 5, 6]
);

// flatten_to_depth_one
let deep = vec![vec![vec![1, 2], vec![3]], vec![vec![4]]];
assert_eq!(
    flatten_to_depth_one(deep),
    vec![vec![1, 2], vec![3], vec![4]]
);

// words_from_lines
assert_eq!(
    words_from_lines(&["hello world", "foo bar"]),
    vec!["hello", "world", "foo", "bar"]
);

// flatten_and_filter
let nested = vec![vec![1, 2, 3], vec![4, 5, 6]];
assert_eq!(
    flatten_and_filter(nested, |&x| x % 2 == 0),
    vec![2, 4, 6]
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- `flatten()` works on any iterator whose items implement `IntoIterator`
- `flat_map(f)` is equivalent to `map(f).flatten()` but more efficient
- For `expand_ranges`, use `flat_map` with a range expression `start..=end`
- `Option<T>` and `Result<T, E>` implement `IntoIterator`, so `flatten()` filters them naturally
- For `flatten_to_depth_one`, call `flatten()` only once on the outer iterator
- Use `split_whitespace()` to split strings into words
- Remember that `into_iter()` takes ownership while `iter()` borrows

</details>
