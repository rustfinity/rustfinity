Iterators are one of Rust's most powerful abstractions. While you've used many built-in iterators and combinators, creating your own custom iterators unlocks the full potential of Rust's iteration patterns. By implementing the `Iterator` trait, you can create lazy, composable sequences that integrate seamlessly with all of Rust's iterator methods.

The `Iterator` trait requires only one method: `next()`. This method returns `Option<Self::Item>`, yielding `Some(value)` for each element and `None` when the sequence is exhausted. The key to implementing custom iterators is managing internal state—tracking where you are in the sequence and knowing when to stop.

## The Iterator Trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Once you implement `next()`, you get all the iterator adapters (`.map()`, `.filter()`, `.fold()`, etc.) for free!

## Example: A Counter Iterator

```rust
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

// Usage
let counter = Counter::new(3);
let values: Vec<u32> = counter.collect();
assert_eq!(values, vec![0, 1, 2]);
```

## Your Task

Implement the following custom iterators:

### 1. Fibonacci Iterator

Create a `Fibonacci` struct that generates Fibonacci numbers indefinitely:

- `Fibonacci::new() -> Fibonacci` - Create a new Fibonacci iterator starting with 0, 1
- Yields: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...

### 2. StepRange Iterator

Create a `StepRange` struct that iterates from `start` to `end` (exclusive) with a custom step:

- `StepRange::new(start: i32, end: i32, step: i32) -> StepRange`
- Handles positive steps (ascending) and negative steps (descending)
- Yields nothing if step would never reach end (e.g., start=0, end=10, step=-1)

### 3. Cycle Iterator

Create a `CycleN<T>` struct that cycles through a slice a fixed number of times:

- `CycleN::new(items: &[T], times: usize) -> CycleN<T>` where `T: Clone`
- Yields each element in order, repeating the sequence `times` times total

### 4. Collatz Iterator

Create a `Collatz` struct that generates the Collatz sequence starting from a given number:

- `Collatz::new(start: u64) -> Collatz`
- If n is even: next = n / 2
- If n is odd: next = 3n + 1
- Stops when it reaches 1 (yields 1 as the final element)

### 5. Windows Iterator

Create a `Windows<T>` struct that yields overlapping windows of a fixed size:

- `Windows::new(items: &[T], size: usize) -> Windows<T>` where `T: Clone`
- Yields slices (as `Vec<T>`) of length `size` from the items
- Each window overlaps with the previous by `size - 1` elements

### 6. Unfold Iterator

Create an `Unfold<T, F>` struct that generates values from a state and function:

- `Unfold::new(initial: T, f: F) -> Unfold<T, F>` where `F: FnMut(&T) -> Option<T>`
- The function takes the current state and returns `Some(next_state)` or `None` to stop
- Yields each state value (including the initial value)

## Examples

```rust
// Fibonacci
let fib = Fibonacci::new();
let first_ten: Vec<u64> = fib.take(10).collect();
assert_eq!(
    first_ten,
    vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
);

// StepRange
let range = StepRange::new(0, 10, 2);
let evens: Vec<i32> = range.collect();
assert_eq!(evens, vec![0, 2, 4, 6, 8]);

let countdown = StepRange::new(5, 0, -1);
let nums: Vec<i32> = countdown.collect();
assert_eq!(nums, vec![5, 4, 3, 2, 1]);

// CycleN
let cycle = CycleN::new(&[1, 2, 3], 2);
let repeated: Vec<i32> = cycle.collect();
assert_eq!(
    repeated,
    vec![1, 2, 3, 1, 2, 3]
);

// Collatz
let collatz = Collatz::new(6);
let sequence: Vec<u64> = collatz.collect();
assert_eq!(
    sequence,
    vec![6, 3, 10, 5, 16, 8, 4, 2, 1]
);

// Windows
let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
let groups: Vec<Vec<i32>> = windows.collect();
assert_eq!(
    groups,
    vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
);

// Unfold (powers of 2 until > 100)
let powers = Unfold::new(1u32, |&n| {
    let next = n * 2;
    if next <= 100 { Some(next) } else { None }
});
let result: Vec<u32> = powers.collect();
assert_eq!(
    result,
    vec![1, 2, 4, 8, 16, 32, 64]
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `Fibonacci`, store two consecutive numbers and update them each iteration
- For `StepRange`, validate step direction matches the relationship between start and end
- For `CycleN`, track both the current index in the slice and how many complete cycles remain
- For `Collatz`, use a flag or Option to track when you've yielded 1 and should stop
- For `Windows`, track the starting index and yield a slice/clone from that position
- For `Unfold`, the tricky part is yielding the initial value first, then subsequent values
- Remember that `next()` takes `&mut self`—you need mutable state to track progress
- Use `Option` fields when you need to represent "already exhausted" states

</details>
