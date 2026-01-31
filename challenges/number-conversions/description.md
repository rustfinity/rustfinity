# Number Conversions

When working with numeric types in Rust, you'll often need to convert between different integer sizes (like `i32` to `i64`) or perform arithmetic that might overflow. While Rust's `as` keyword provides basic casts, it can silently truncate or wrap values, leading to subtle bugs. The standard library provides safer alternatives.

## Safe Type Conversions

Rust's `TryFrom` and `TryInto` traits allow fallible conversions that return `Result` instead of silently changing values:

```rust
use std::convert::TryFrom;

let big: i64 = 1000;
let small: Result<i16, _> = i16::try_from(big);  // Ok(1000)

let too_big: i64 = 100_000;
// Err(...)
let overflow: Result<i16, _> = i16::try_from(too_big);
```

## Safe Arithmetic Operations

Rust integers provide methods for safe arithmetic that handle overflow explicitly:

- **Checked operations** return `Option<T>`, giving `None` on overflow:

  ```rust
  let x: u8 = 250;
  assert_eq!(x.checked_add(10), None);  // Would overflow
  assert_eq!(x.checked_add(5), Some(255));
  ```

- **Saturating operations** clamp at the type's bounds:

  ```rust
  let x: u8 = 250;
  assert_eq!(x.saturating_add(10), 255);  // Clamped to max
  assert_eq!(x.saturating_sub(255), 0);   // Clamped to min
  ```

- **Wrapping operations** wrap around on overflow (like C behavior):
  ```rust
  let x: u8 = 250;
  assert_eq!(x.wrapping_add(10), 4);  // 260 wraps to 4
  ```

## Your Task

Implement the following functions to demonstrate safe numeric conversions and arithmetic:

1. **`safe_i32_to_i16(value: i32) -> Option<i16>`**
   - Convert an `i32` to `i16`, returning `None` if the
     value doesn't fit.

2. **`safe_u64_to_u32(value: u64) -> Option<u32>`**
   - Convert a `u64` to `u32`, returning `None` if the
     value doesn't fit.

3. **`safe_i64_to_usize(value: i64) -> Option<usize>`**
   - Convert an `i64` to `usize`, returning `None` if
     the value is negative or too large.

4. **`checked_multiply(a: i32, b: i32) -> Option<i32>`**
   - Multiply two `i32` values, returning `None` on
     overflow.

5. **`checked_power(base: u32, exp: u32) -> Option<u32>`**
   - Calculate `base^exp`, returning `None` on overflow.

6. **`saturating_sum(numbers: &[i32]) -> i32`**
   - Sum all numbers using saturating arithmetic
     (clamp at `i32::MIN` or `i32::MAX`).

7. **`wrapping_factorial(n: u32) -> u32`** - Calculate `n!` using wrapping arithmetic (allows overflow to wrap around).

8. **`safe_average(numbers: &[i64]) -> Option<i64>`** - Calculate the average of numbers, returning `None` if the sum would overflow or the slice is empty.

## Examples

```rust
// Safe conversions
assert_eq!(safe_i32_to_i16(1000), Some(1000));
assert_eq!(safe_i32_to_i16(100_000), None);

// Checked arithmetic
assert_eq!(checked_multiply(1000, 1000), Some(1_000_000));
assert_eq!(checked_multiply(100_000, 100_000), None);  // Overflow

// Saturating arithmetic
assert_eq!(saturating_sum(&[i32::MAX, 1]), i32::MAX);
assert_eq!(saturating_sum(&[i32::MIN, -1]), i32::MIN);

// Wrapping arithmetic
assert_eq!(wrapping_factorial(5), 120);
// Wrapped value
assert_eq!(wrapping_factorial(20), 2_192_834_560);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `i16::try_from(value).ok()` to convert `TryFrom` results to `Option`.
- For `safe_i64_to_usize`, remember that negative values and values larger than `usize::MAX` should both return `None`.
- The `checked_pow` method works like `checked_mul` but for exponentiation.
- For `saturating_sum`, use `Iterator::fold` with `saturating_add`.
- For `safe_average`, first check if the slice is empty, then use checked arithmetic to compute the sum before dividing.
- Remember that `wrapping_mul` continues even when overflow occurs.

</details>
