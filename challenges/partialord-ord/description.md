# PartialOrd and Ord

In Rust, comparing values for ordering (less than, greater than) is done through two traits: `PartialOrd` and `Ord`. These traits are the ordering counterparts to `PartialEq` and `Eq` that you learned about for equality comparison.

`PartialOrd` defines a **partial ordering** where comparisons may not always produce a result. The classic example is floating-point numbers: comparing `NaN` to any value (including itself) returns `None` because `NaN` has no defined ordering. The trait requires implementing the `partial_cmp` method which returns `Option<Ordering>`.

`Ord` defines a **total ordering** where every pair of values can be compared, and the result is always `Less`, `Equal`, or `Greater`. Types implementing `Ord` must also implement `PartialOrd`, `Eq`, and `PartialEq`. This is the trait used for sorting, binary search, and ordered collections like `BTreeMap`.

## Your Task

Implement the following types and functions demonstrating ordering traits:

1. **`Score`** - A struct wrapping a `u32` that derives `PartialOrd` and `Ord` for natural ordering.

2. **`Version`** - A struct with `major`, `minor`, and `patch` fields (all `u32`). Implement `Ord` manually to compare versions semantically (major first, then minor, then patch).

3. **`Temperature`** - An enum with `Celsius(f64)` and `Fahrenheit(f64)` variants. Implement `PartialOrd` to compare temperatures after normalizing to Celsius. This should NOT implement `Ord` because floating-point values have no total ordering (NaN).

4. **`Priority`** - An enum with `Low`, `Medium`, `High`, and `Critical` variants. Derive `PartialOrd` and `Ord` where the order follows the declaration order (Low < Medium < High < Critical).

5. **`Player`** - A struct with `name: String` and `score: u32`. Implement `Ord` to sort by score descending (higher scores first), then by name ascending as a tiebreaker.

6. **`find_min<T: Ord>`** - Returns a reference to the minimum element in a non-empty slice, or `None` if empty.

7. **`find_max<T: Ord>`** - Returns a reference to the maximum element in a non-empty slice, or `None` if empty.

8. **`is_sorted<T: Ord>`** - Returns `true` if the slice is sorted in ascending order (each element <= the next).

9. **`clamp<T: Ord>`** - Returns a value clamped between min and max bounds (inclusive). If value < min, returns min. If value > max, returns max. Otherwise returns the value itself.

## Examples

```rust
use partialord_ord::*;

// Score - derived ordering
let s1 = Score(100);
let s2 = Score(200);
assert!(s1 < s2);

// Version - semantic versioning order
let v1 = Version::new(1, 9, 0);
let v2 = Version::new(2, 0, 0);
assert!(v1 < v2);

// Temperature - partial ordering across units
let t1 = Temperature::Celsius(0.0);
let t2 = Temperature::Fahrenheit(32.0);  // 0°C
assert!(t1 == t2);  // Equal when normalized

// Priority - enum ordering
assert!(Priority::Low < Priority::Critical);

// Player - custom ordering (score desc, name asc)
let alice = Player::new("Alice", 100);
let bob = Player::new("Bob", 100);
assert!(alice < bob);  // Same score, alphabetical order

// Utility functions
assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(&5));
assert!(is_sorted(&[1, 2, 3, 4, 5]));
assert_eq!(clamp(&5, &1, &10), &5);
assert_eq!(clamp(&0, &1, &10), &1);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `#[derive(PartialOrd, Ord)]` for simple types where field-by-field comparison is correct.
- When implementing `Ord` manually, you must implement the `cmp` method returning `std::cmp::Ordering`.
- The `Ordering` enum has variants: `Less`, `Equal`, `Greater`.
- For `Version`, compare fields in order of importance using `.cmp()` and `.then()` or `.then_with()`.
- For `Temperature`, convert to a common unit before comparing. Remember that `partial_cmp` returns `Option<Ordering>`.
- For `Player` with descending score, reverse the comparison order for scores.
- Use `.cmp()` for `Ord` types and `.partial_cmp()` for `PartialOrd` types.
- Derive macros for enums order variants by their declaration order (first variant is smallest).

</details>
