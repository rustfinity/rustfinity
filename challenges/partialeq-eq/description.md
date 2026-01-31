# PartialEq and Eq Traits

In Rust, comparing values for equality is done through two related traits: `PartialEq` and `Eq`. These traits are fundamental to how Rust handles equality comparisons and are used extensively throughout the standard library.

`PartialEq` is a trait that allows you to compare two values of the same type (or different types) for equality using the `==` and `!=` operators. The "partial" in `PartialEq` refers to the fact that not all values of a type may be comparable—the most common example being floating-point numbers, where `NaN != NaN`. For most types, however, all values can be compared.

`Eq` is a marker trait (it has no methods of its own) that indicates a type has "total equality"—meaning that for any two values `a` and `b`, the equality comparison is reflexive (`a == a`), symmetric (`a == b` implies `b == a`), and transitive (`a == b` and `b == c` implies `a == c`). Types that implement `Eq` guarantee that every value equals itself.

## Your Task

Implement the `PartialEq` and `Eq` traits for several types to understand how equality works in Rust:

1. **`Point`** - A 2D point with `x` and `y` coordinates (both `i32`). Derive both `PartialEq` and `Eq`.

2. **`CaseInsensitiveString`** - A string wrapper where equality comparison ignores case. Implement `PartialEq` manually so that `"Hello"` equals `"HELLO"`.

3. **`ApproximateFloat`** - A wrapper around `f64` that compares values within a small epsilon (0.0001). Implement `PartialEq` manually but NOT `Eq` (since floating-point equality is not total).

4. **`UserId`** - A newtype wrapper around `u64` that should be usable as a HashMap key. Derive `PartialEq`, `Eq`, and `Hash`.

5. **`Person`** - A struct with `name` (String) and `id` (u64) where equality is based ONLY on the `id` field. Implement `PartialEq` and `Eq` manually.

6. **`Status`** - An enum with variants `Active`, `Inactive`, and `Pending`. Derive `PartialEq` and `Eq`.

Additionally, implement these utility functions:

7. **`are_all_equal<T: Eq>`** - Returns `true` if all elements in a slice are equal to each other.

8. **`count_matches<T: PartialEq>`** - Counts how many elements in a slice equal a given target value.

9. **`find_first_match<T: PartialEq>`** - Returns the index of the first element that equals the target, or `None` if not found.

## Examples

```rust
// Point - derived equality
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
let p3 = Point { x: 3, y: 4 };
assert_eq!(p1, p2);
assert_ne!(p1, p3);

// CaseInsensitiveString - case-insensitive comparison
let s1 = CaseInsensitiveString::new("Hello");
let s2 = CaseInsensitiveString::new("HELLO");
let s3 = CaseInsensitiveString::new("World");
assert_eq!(s1, s2);
assert_ne!(s1, s3);

// ApproximateFloat - epsilon comparison
let f1 = ApproximateFloat(1.0);
let f2 = ApproximateFloat(1.00005);  // Within epsilon
let f3 = ApproximateFloat(1.001);    // Outside epsilon
assert_eq!(f1, f2);
assert_ne!(f1, f3);

// UserId - can be used in HashSet
use std::collections::HashSet;
let mut ids: HashSet<UserId> = HashSet::new();
ids.insert(UserId(1));
ids.insert(UserId(2));
assert!(ids.contains(&UserId(1)));

// Person - equality based only on id
let alice1 = Person { name: String::from("Alice"), id: 1 };
let alice2 = Person { name: String::from("Alice Smith"), id: 1 };  // Same id
let bob = Person { name: String::from("Bob"), id: 2 };
assert_eq!(alice1, alice2);  // Same id, so equal
assert_ne!(alice1, bob);

// Status enum
assert_eq!(Status::Active, Status::Active);
assert_ne!(Status::Active, Status::Inactive);

// Utility functions
assert!(are_all_equal(&[1, 1, 1, 1]));
assert!(!are_all_equal(&[1, 2, 1, 1]));

assert_eq!(count_matches(&[1, 2, 1, 3, 1], &1), 3);
assert_eq!(find_first_match(&[10, 20, 30, 40], &30), Some(2));
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `#[derive(PartialEq, Eq)]` when all fields should participate in equality and standard comparison is sufficient.

- For `CaseInsensitiveString`, convert both strings to lowercase (or uppercase) before comparing in your `PartialEq` implementation:

  ```rust
  impl PartialEq for CaseInsensitiveString {
      fn eq(&self, other: &Self) -> bool {
          self.0.to_lowercase() == other.0.to_lowercase()
      }
  }
  ```

- For `ApproximateFloat`, use `(self.0 - other.0).abs() < EPSILON` for the comparison. Remember that `f64` cannot implement `Eq` because of `NaN`.

- When implementing `PartialEq` manually, you only need to implement `eq(&self, other: &Self) -> bool`. The `ne` method has a default implementation.

- For `UserId` to be used in `HashSet`/`HashMap`, it needs `Eq + Hash`. You can derive all three: `#[derive(PartialEq, Eq, Hash)]`.

- For `Person`, implement `PartialEq` to compare only the `id` field, ignoring `name`. Since `id` is `u64` (which implements `Eq`), `Person` can also implement `Eq`.

- For `are_all_equal`, handle the empty slice case (return `true` since there are no unequal elements), then compare all elements to the first.

- The `Eq` trait is a marker trait—just add it after implementing `PartialEq`:
  ```rust
  impl Eq for MyType {}
  ```

</details>
