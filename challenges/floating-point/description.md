# Floating Point Edge Cases

Floating-point numbers in Rust (`f32` and `f64`) follow the IEEE 754 standard, which includes some surprising behaviors that can trip up even experienced programmers. Understanding these edge cases is crucial for writing robust numerical code.

## Special Values

Floating-point types have special values that represent unusual mathematical concepts:

- **NaN (Not a Number)**: Result of invalid operations like `0.0/0.0` or `(-1.0_f64).sqrt()`. NaN is unique because it's not equal to anything, including itself!
- **Infinity**: Result of overflow or dividing by zero (e.g., `1.0/0.0` gives positive infinity)
- **Negative Zero**: `-0.0` exists and equals `0.0`, but can be distinguished

```rust
let nan = f64::NAN;
assert!(nan != nan);  // NaN is never equal to itself!
assert!(nan.is_nan());

let inf = f64::INFINITY;
assert!(inf.is_infinite());
assert!(inf > f64::MAX);

// Negative zero equals positive zero
assert_eq!(0.0, -0.0);
```

## Checking for Special Values

The standard library provides methods to detect these special values:

```rust
let x = 1.0_f64;
assert!(x.is_finite());      // Not NaN, not infinite
assert!(!x.is_nan());
assert!(!x.is_infinite());
// Not zero, subnormal, infinite, or NaN
assert!(x.is_normal());

let zero = 0.0_f64;
assert!(!zero.is_normal());  // Zero is not "normal"
assert!(zero.is_finite());   // But it is finite
```

## Rounding and Truncation

Rust provides several methods for converting floats to integers or rounding:

```rust
let x = 3.7_f64;
// Round toward negative infinity
assert_eq!(x.floor(), 3.0);
// Round toward positive infinity
assert_eq!(x.ceil(), 4.0);
// Round to nearest, ties away from zero
assert_eq!(x.round(), 4.0);
assert_eq!(x.trunc(), 3.0);   // Round toward zero

let y = -3.7_f64;
assert_eq!(y.floor(), -4.0);  // -4 is "more negative"
assert_eq!(y.trunc(), -3.0);  // Toward zero is -3
```

## Approximate Comparisons

Due to floating-point precision limits, direct equality comparison is often unreliable:

```rust
let a = 0.1 + 0.2;
let b = 0.3;
assert!(a != b);  // Surprise! They're not exactly equal
assert!((a - b).abs() < 1e-10);  // But they're very close
```

## Your Task

Implement the following functions to handle floating-point edge cases:

1. **`is_valid_number(x: f64) -> bool`**
   - Returns `true` if `x` is a finite, non-NaN number
     (i.e., a "normal" number you can do math with).

2. **`classify_float(x: f64) -> &'static str`**
   - Returns a string describing the float: `"nan"`,
     `"infinite"`, `"zero"`, `"normal"`, or `"subnormal"`.

3. **`safe_divide(a: f64, b: f64) -> Option<f64>`**
   - Divides `a` by `b`, returning `None` if `b` is zero
     or if either input is NaN/infinite.

4. **`round_to_places(x: f64, places: u32) -> f64`**
   - Rounds `x` to the specified number of decimal places.

5. **`approx_equal(a: f64, b: f64, epsilon: f64) -> bool`**
   - Returns `true` if `a` and `b` are within `epsilon`
     of each other. Returns `false` if either is NaN.

6. **`clamp_to_range(x: f64, min: f64, max: f64) -> Option<f64>`**
   - Clamps `x` to the range `[min, max]`. Returns `None`
     if any input is NaN or if `min > max`.

7. **`safe_sqrt(x: f64) -> Option<f64>`** - Returns the square root of `x`, or `None` if `x` is negative or NaN.

8. **`sum_finite(numbers: &[f64]) -> f64`** - Sums all finite numbers in the slice, skipping any NaN or infinite values.

## Examples

```rust
// Validation
assert!(is_valid_number(42.0));
assert!(!is_valid_number(f64::NAN));
assert!(!is_valid_number(f64::INFINITY));

// Classification
assert_eq!(classify_float(f64::NAN), "nan");
assert_eq!(classify_float(0.0), "zero");
assert_eq!(classify_float(1.5), "normal");

// Safe division
assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
assert_eq!(safe_divide(10.0, 0.0), None);
assert_eq!(safe_divide(f64::NAN, 2.0), None);

// Rounding
assert_eq!(round_to_places(3.14159, 2), 3.14);
assert_eq!(round_to_places(2.5, 0), 3.0);

// Approximate equality
assert!(approx_equal(0.1 + 0.2, 0.3, 1e-10));
assert!(!approx_equal(1.0, 2.0, 0.5));

// Clamping
assert_eq!(clamp_to_range(5.0, 0.0, 10.0), Some(5.0));
assert_eq!(clamp_to_range(-5.0, 0.0, 10.0), Some(0.0));
assert_eq!(clamp_to_range(f64::NAN, 0.0, 10.0), None);

// Safe sqrt
assert_eq!(safe_sqrt(4.0), Some(2.0));
assert_eq!(safe_sqrt(-1.0), None);

// Sum finite values
assert_eq!(
    sum_finite(&[1.0, f64::NAN, 2.0, f64::INFINITY, 3.0]),
    6.0
);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `x.is_finite()` to check if a number is neither NaN nor infinite.
- Use `x.is_nan()` specifically to check for NaN values.
- The `x.classify()` method returns an `FpCategory` enum with variants `Nan`, `Infinite`, `Zero`, `Subnormal`, and `Normal`.
- For `round_to_places`, multiply by 10^places, round, then divide by 10^places.
- For `approx_equal`, compute `(a - b).abs()` and compare to epsilon. Remember that any comparison with NaN returns false.
- The `x.clamp(min, max)` method exists but doesn't handle NaN the way we want - you'll need custom logic.
- For `safe_sqrt`, check both that `x >= 0.0` and that `x` is not NaN.
- Use `filter` with `is_finite()` before summing in `sum_finite`.

</details>
