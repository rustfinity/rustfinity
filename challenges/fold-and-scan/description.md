The `fold()` and `scan()` methods are powerful iterator adapters that allow you to maintain state across iteration. They're essential for tasks like computing running totals, building complex aggregations, or transforming sequences based on accumulated state.

## Understanding fold()

`fold()` reduces an iterator to a single value by repeatedly applying an accumulator function. It takes an initial value and a closure that combines the accumulator with each element:

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Sum all numbers
let sum = numbers.iter().fold(0, |acc, &x| acc + x);
assert_eq!(sum, 15);

// Product of all numbers
let product = numbers.iter().fold(1, |acc, &x| acc * x);
assert_eq!(product, 120);

// Build a string
let words = vec!["Hello", "World"];
let sentence = words.iter().fold(String::new(), |acc, &word| {
    if acc.is_empty() {
        word.to_string()
    } else {
        format!("{} {}", acc, word)
    }
});
assert_eq!(sentence, "Hello World");
```

## Understanding scan()

`scan()` is similar to `fold()` but yields intermediate values at each step, producing a new iterator. It maintains mutable state and returns `Option<T>` values:

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Running sum - yields cumulative totals
let running_sums: Vec<i32> = numbers
    .iter()
    .scan(0, |state, &x| {
        *state += x;
        Some(*state)
    })
    .collect();
assert_eq!(running_sums, vec![1, 3, 6, 10, 15]);

// Running product
let running_products: Vec<i32> = numbers
    .iter()
    .scan(1, |state, &x| {
        *state *= x;
        Some(*state)
    })
    .collect();
assert_eq!(running_products, vec![1, 2, 6, 24, 120]);
```

## Key Differences

- **`fold()`**: Consumes the entire iterator and returns a single final value
- **`scan()`**: Returns an iterator of intermediate results; can stop early by returning `None`

```rust
// scan can stop early
let numbers = vec![1, 2, 3, 4, 5];
let until_exceeds_5: Vec<i32> = numbers
    .iter()
    .scan(0, |state, &x| {
        *state += x;
        if *state <= 5 {
            Some(*state)
        } else {
            None  // Stop iteration
        }
    })
    .collect();
assert_eq!(until_exceeds_5, vec![1, 3]);  // Stops before exceeding 5
```

## Your Task

Implement the following functions that demonstrate `fold()` and `scan()` patterns:

1. `sum_with_fold(numbers: &[i32]) -> i32` - Sum all numbers using `fold()`
2. `product_with_fold(numbers: &[i32]) -> i64` - Multiply all numbers using `fold()` (use i64 to avoid overflow)
3. `concat_strings(strings: &[&str], separator: &str) -> String` - Join strings with a separator using `fold()`
4. `running_sum(numbers: &[i32]) -> Vec<i32>` - Return cumulative sums using `scan()`
5. `running_max(numbers: &[i32]) -> Vec<i32>` - Return running maximum values using `scan()`
6. `take_while_sum_under(numbers: &[i32], limit: i32) -> Vec<i32>` - Take numbers while running sum stays under limit using `scan()`
7. `count_occurrences<T: PartialEq>(items: &[T], target: &T) -> usize` - Count occurrences using `fold()`
8. `running_average(numbers: &[f64]) -> Vec<f64>` - Return running average at each position using `scan()`

## Examples

```rust
// sum_with_fold
assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);

// product_with_fold
assert_eq!(product_with_fold(&[1, 2, 3, 4]), 24);

// concat_strings
assert_eq!(concat_strings(&["a", "b", "c"], "-"), "a-b-c");

// running_sum
assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);

// running_max
assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);

// take_while_sum_under - takes [1, 2, 3] but stops before adding 4 (sum would be 10 >= 10)
assert_eq!(take_while_sum_under(&[1, 2, 3, 4, 5], 10), vec![1, 2, 3]);

// count_occurrences
assert_eq!(count_occurrences(&[1, 2, 1, 3, 1], &1), 3);

// running_average
assert_eq!(running_average(&[2.0, 4.0, 6.0]), vec![2.0, 3.0, 4.0]);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `fold()`, the closure signature is `|accumulator, element| -> new_accumulator`
- For `scan()`, the closure signature is `|&mut state, element| -> Option<output>`
- In `scan()`, you modify `state` in place using `*state = ...`
- Use `scan()` returning `None` to stop iteration early
- For `running_average`, track both the sum and count in the state (use a tuple)
- `product_with_fold` uses i64 to handle larger products safely
- Empty slices should return sensible defaults (0 for sum, 1 for product, empty string, etc.)

</details>
