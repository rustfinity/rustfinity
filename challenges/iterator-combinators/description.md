Iterator combinators are methods on iterators that allow you to compose and transform sequences of values in powerful ways. They are a cornerstone of functional programming in Rust, enabling you to write concise, expressive, and efficient code without explicit loops.

## Understanding Iterator Combinators

Rust's `Iterator` trait provides dozens of combinator methods. These methods take an iterator and return a new iterator with modified behavior. Because they're lazy, combinators don't actually do any work until you consume the iterator (e.g., with `collect()`, `for_each()`, or `sum()`).

```rust
let numbers = vec![1, 2, 3, 4, 5];

// chain() combines two iterators into one
let more_numbers = vec![6, 7, 8];
let combined: Vec<i32> = numbers.iter()
    .chain(more_numbers.iter())
    .copied()
    .collect();
// combined = [1, 2, 3, 4, 5, 6, 7, 8]

// zip() pairs elements from two iterators
let names = vec!["Alice", "Bob", "Charlie"];
let ages = vec![30, 25, 35];
let people: Vec<_> = names.iter().zip(ages.iter()).collect();
// people = [("Alice", 30), ("Bob", 25), ("Charlie", 35)]
```

## Key Combinators

- **`chain(other)`** - Concatenates two iterators, yielding all elements from the first, then all from the second
- **`zip(other)`** - Pairs up elements from two iterators; stops when either is exhausted
- **`take(n)`** - Yields only the first `n` elements
- **`skip(n)`** - Skips the first `n` elements, yielding the rest
- **`rev()`** - Reverses a double-ended iterator
- **`cycle()`** - Repeats an iterator infinitely

```rust
// take() and skip() for slicing
let nums: Vec<i32> = (1..=10).skip(2).take(5).collect();
// nums = [3, 4, 5, 6, 7]

// rev() to reverse
let reversed: Vec<i32> = vec![1, 2, 3].into_iter().rev().collect();
// reversed = [3, 2, 1]

// cycle() for infinite repetition (use with take!)
let pattern: Vec<i32> = [1, 2].iter().copied().cycle().take(6).collect();
// pattern = [1, 2, 1, 2, 1, 2]
```

## Your Task

Implement the following functions that use iterator combinators:

1. `chain_sequences<T: Clone>(first: &[T], second: &[T]) -> Vec<T>` - Combine two slices into a single Vec using `chain()`
2. `zip_pairs<T: Clone, U: Clone>(first: &[T], second: &[U]) -> Vec<(T, U)>` - Pair elements from two slices using `zip()`
3. `take_first<T: Clone>(items: &[T], n: usize) -> Vec<T>` - Return the first `n` elements using `take()`
4. `skip_first<T: Clone>(items: &[T], n: usize) -> Vec<T>` - Skip the first `n` elements using `skip()`
5. `reverse_sequence<T: Clone>(items: &[T]) -> Vec<T>` - Reverse a sequence using `rev()`
6. `interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T>` - Interleave elements from two slices (alternating elements)
7. `sliding_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)>` - Create pairs of consecutive elements using `zip()` with a skipped version

## Examples

```rust
// chain_sequences
assert_eq!(chain_sequences(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);

// zip_pairs
assert_eq!(
    zip_pairs(&[1, 2, 3], &["a", "b"]),
    vec![(1, "a"), (2, "b")]
);

// take_first
assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);

// skip_first
assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);

// reverse_sequence
assert_eq!(reverse_sequence(&[1, 2, 3]), vec![3, 2, 1]);

// interleave - alternates elements from both slices
assert_eq!(
    interleave(&[1, 3, 5], &[2, 4, 6]),
    vec![1, 2, 3, 4, 5, 6]
);

// sliding_pairs - pairs consecutive elements
assert_eq!(
    sliding_pairs(&[1, 2, 3, 4]),
    vec![(1, 2), (2, 3), (3, 4)]
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `chain_sequences`, use `.iter().chain(other.iter())` and then `.cloned().collect()`
- For `zip_pairs`, use `.iter().zip(other.iter())` - remember zip stops at the shorter iterator
- For `interleave`, use `zip()` with `flat_map()` to flatten tuples into individual elements
- For `sliding_pairs`, zip the original iterator with a version that skips the first element
- Most combinators work with iterators, so start with `.iter()` on the slice
- Use `.cloned()` to convert `&T` to `T` when you need owned values

</details>
