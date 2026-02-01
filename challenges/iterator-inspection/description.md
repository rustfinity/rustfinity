When working with iterators, you often need to inspect or track the state of iteration without fundamentally changing the data. Rust provides several powerful methods for this: `enumerate()` adds indices to your elements, `peekable()` lets you look ahead without consuming, and `inspect()` allows side effects for debugging.

## Understanding Iterator Inspection Methods

These methods are invaluable for debugging iterator pipelines, implementing parsers that need lookahead, or simply tracking positions within a sequence.

```rust
let words = vec!["hello", "world", "rust"];

// enumerate() adds indices
for (index, word) in words.iter().enumerate() {
    println!("{}: {}", index, word);
}
// Output:
// 0: hello
// 1: world
// 2: rust

// peekable() allows looking ahead
let mut iter = words.iter().peekable();
while let Some(word) = iter.next() {
    if let Some(&next_word) = iter.peek() {
        println!("{} is followed by {}", word, next_word);
    } else {
        println!("{} is the last word", word);
    }
}
```

## Key Methods

- **`enumerate()`** - Wraps each element with its index as `(index, element)` tuples
- **`peekable()`** - Creates an iterator that can look at the next element via `peek()` without consuming it
- **`inspect(closure)`** - Calls a closure on each element for side effects (e.g., debugging) without modifying the iterator

```rust
// inspect() for debugging pipelines
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .inspect(|x| {
        println!("Before filter: {}", x)
    })
    .filter(|x| *x % 2 == 0)
    .inspect(|x| {
        println!("After filter: {}", x)
    })
    .sum();
// Prints debugging info while computing sum = 6
```

## Your Task

Implement the following functions that use iterator inspection methods:

1. `indexed_elements<T: Clone>(
    items: &[T]
) -> Vec<(usize, T)>`
   - Return elements with their indices using
     `enumerate()`
2. `find_index<T: PartialEq>(
    items: &[T],
    target: &T
) -> Option<usize>`
   - Find the index of the first occurrence of
     target
3. `elements_with_next<T: Clone>(
    items: &[T]
) -> Vec<(T, Option<T>)>`
   - Return each element paired with the next
     element (or None if last) using `peekable()`
4. `group_consecutive_duplicates<
   T: Clone + PartialEq
   > (items: &[T]) -> Vec<(T, usize)>`
   - Group consecutive identical elements and
     count them using `peekable()`
5. `find_first_repeated<T: Clone + PartialEq>(
    items: &[T]
) -> Option<T>`
   - Find the first element that equals its next
     neighbor using `peekable()`
6. `collect_with_trace<
   T: Clone + std::fmt::Debug
   > (items: &[T], trace: &mut Vec<String>) -> Vec<T>`
   - Collect elements while recording a trace of
     each element using `inspect()`
7. `sum_with_running_total(
    numbers: &[i32],
    totals: &mut Vec<i32>
) -> i32`
   - Sum numbers while recording running totals at
     each step using `inspect()`

## Examples

```rust
// indexed_elements
assert_eq!(
    indexed_elements(&["a", "b", "c"]),
    vec![(0, "a"), (1, "b"), (2, "c")]
);

// find_index
assert_eq!(find_index(&[10, 20, 30, 40], &30), Some(2));
assert_eq!(find_index(&[10, 20, 30], &50), None);

// elements_with_next
assert_eq!(
    elements_with_next(&[1, 2, 3]),
    vec![(1, Some(2)), (2, Some(3)), (3, None)]
);

// group_consecutive_duplicates
assert_eq!(
    group_consecutive_duplicates(
        &[1, 1, 2, 3, 3, 3]
    ),
    vec![(1, 2), (2, 1), (3, 3)]
);

// find_first_repeated
assert_eq!(find_first_repeated(&[1, 2, 2, 3]), Some(2));
assert_eq!(find_first_repeated(&[1, 2, 3]), None);

// collect_with_trace
let mut trace = Vec::new();
let result = collect_with_trace(&[1, 2, 3], &mut trace);
assert_eq!(result, vec![1, 2, 3]);
assert_eq!(trace, vec!["1", "2", "3"]);

// sum_with_running_total
let mut totals = Vec::new();
let sum = sum_with_running_total(
    &[10, 20, 30],
    &mut totals
);
assert_eq!(sum, 60);
assert_eq!(totals, vec![10, 30, 60]);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `indexed_elements`, use
  `.iter().enumerate()` and clone the elements
  when collecting
- For `find_index`, use `enumerate()` with
  `find()` to locate the target, then extract
  the index
- For `elements_with_next`, use `peekable()`
  with `peek()` to look at the next element
  before calling `next()`
- For `group_consecutive_duplicates`, use
  `peekable()` to compare current with next and
  count runs
- For `find_first_repeated`, use `peekable()`
  to compare each element with its successor via
  `peek()`
- For `collect_with_trace`, use `inspect()` to
  push formatted elements into the trace vector
- For `sum_with_running_total`, use `inspect()`
  with a running total that gets updated and
  pushed
- Remember: `peek()` returns `Option<&T>`, so
  you may need to handle references carefully

</details>
