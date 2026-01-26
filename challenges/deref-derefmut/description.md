# Deref and DerefMut Traits

The `Deref` and `DerefMut` traits are fundamental to Rust's smart pointer pattern. They allow your custom types to behave like references, enabling automatic dereferencing through the `*` operator and enabling **deref coercion** - one of Rust's most convenient ergonomic features.

When you implement `Deref` for a type, you're telling Rust how to convert a reference to your type into a reference to another type. This is what allows `Box<T>`, `Rc<T>`, `Arc<T>`, and `String` to seamlessly work with functions expecting references to their inner types.

## Deref Coercion

Deref coercion is a compile-time convenience that automatically converts references when types implement `Deref`. For example:
- `&String` can be used where `&str` is expected
- `&Box<T>` can be used where `&T` is expected
- `&Vec<T>` can be used where `&[T]` is expected

This happens automatically in function arguments, method calls, and other contexts where Rust needs to match types.

## Your Task

Implement wrapper types that demonstrate `Deref` and `DerefMut` patterns:

1. **`MyBox<T>`**: A simple wrapper type that implements `Deref` and `DerefMut`, allowing access to the inner value.

2. **`CachedValue<T>`**: A wrapper that holds a value and tracks how many times it has been accessed (via deref). It should:
   - Implement `Deref<Target = T>` to provide read access while incrementing an access counter
   - Provide a method `access_count(&self) -> usize` to retrieve the count
   - Note: Due to `Deref` requiring `&self`, you'll need interior mutability (`Cell`)

3. **`NonEmptyVec<T>`**: A wrapper around `Vec<T>` that guarantees at least one element. It should:
   - Have a constructor `new(first: T) -> Self`
   - Have a method `push(&mut self, value: T)`
   - Implement `Deref<Target = [T]>` for slice operations
   - Implement `DerefMut` for mutable slice operations

4. **`UppercaseString`**: A string wrapper that always stores its content in uppercase. It should:
   - Have `new(s: &str) -> Self` that converts to uppercase
   - Implement `Deref<Target = str>` for string operations
   - Do NOT implement `DerefMut` (to prevent bypassing the uppercase invariant)

5. **Function `describe_length`**: A generic function demonstrating deref coercion:
   ```rust
   pub fn describe_length<T: Deref<Target = str>>(s: &T) -> String
   ```
   This should work with `&String`, `&Box<str>`, `&UppercaseString`, etc., returning a description like `"Length: 5"`.

## Examples

```rust
use deref_derefmut::*;

// MyBox usage
let mut b = MyBox::new(42);
assert_eq!(*b, 42);
*b = 100;
assert_eq!(*b, 100);

// CachedValue usage
let cached = CachedValue::new("hello");
assert_eq!(cached.access_count(), 0);
let _ = cached.len();  // Deref to &str, increments counter
assert_eq!(cached.access_count(), 1);

// NonEmptyVec usage - guaranteed non-empty
let mut nev = NonEmptyVec::new(1);
nev.push(2);
nev.push(3);
assert_eq!(nev.len(), 3);       // Deref to &[T]
assert_eq!(nev.first(), Some(&1));
nev[0] = 10;                    // DerefMut to &mut [T]
assert_eq!(nev[0], 10);

// UppercaseString usage
let upper = UppercaseString::new("hello");
assert_eq!(&*upper, "HELLO");
assert!(upper.starts_with("HE"));  // Deref coercion to &str

// describe_length with different types
let s = String::from("hello");
assert_eq!(describe_length(&s), "Length: 5");
let boxed: Box<str> = "world".into();
assert_eq!(describe_length(&boxed), "Length: 5");
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- The `Deref` trait requires implementing:
  ```rust
  type Target = T;
  fn deref(&self) -> &Self::Target { ... }
  ```

- `DerefMut` requires `Deref` to be implemented first, and adds:
  ```rust
  fn deref_mut(&mut self) -> &mut Self::Target { ... }
  ```

- For `CachedValue`, use `std::cell::Cell<usize>` to track accesses since `deref` takes `&self`

- For `NonEmptyVec`, you can deref to `&[T]` by calling `as_slice()` on the inner `Vec`

- Remember that `DerefMut` should NOT be implemented if it would violate type invariants (like `UppercaseString`)

- Deref coercion works through multiple levels: `&MyBox<String>` can coerce to `&str`

</details>
