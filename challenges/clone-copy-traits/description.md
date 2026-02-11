In Rust, the `Clone` and `Copy` traits control how values are duplicated. Understanding these traits is essential because they directly impact ownership and how your code behaves when you assign or pass values around.

The `Copy` trait marks types that can be duplicated simply by copying their bits in memory. When a type implements `Copy`, assignment creates a copy rather than a move, meaning you can continue using the original value. Primitive types like integers, floats, and booleans are `Copy` by default. The `Clone` trait, on the other hand, provides explicit cloning via the `.clone()` method and can involve arbitrary code, including heap allocations.

The key relationship: `Copy` is a subtrait of `Clone`, meaning every `Copy` type must also be `Clone`. However, not every `Clone` type is `Copy`. Types like `String` and `Vec<T>` are `Clone` but not `Copy` because copying them would require duplicating heap-allocated data.

## The Clone and Copy Traits

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}

pub trait Copy: Clone { }  // Marker trait - no methods
```

## When to Use Each

- **Copy**: For small, simple types that are cheap to copy bit-by-bit (like primitives)
- **Clone**: For types where duplication requires explicit action or is expensive

## Deriving Clone and Copy

```rust
// Simple struct with all Copy fields - can derive both
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Struct with non-Copy field - can only derive Clone
#[derive(Clone)]
struct Person {
    name: String,  // String is not Copy
    age: u32,
}
```

## Your Task

Implement the following types demonstrating `Clone` and `Copy`:

### 1. Color (Copy type)

Create a `Color` struct with `r`, `g`, `b` fields (all `u8`). This type should be `Copy` since it only contains primitive fields.

- Derive both `Clone` and `Copy`
- Derive `Debug` and `PartialEq` for testing

### 2. Dimensions (Copy type)

Create a `Dimensions` struct with `width` and `height` fields (both `f64`). This should also be `Copy`.

- Derive both `Clone` and `Copy`
- Derive `Debug` and `PartialEq` for testing

### 3. Label (Clone-only type)

Create a `Label` struct with a `text` field (`String`). Since `String` is not `Copy`, this type can only be `Clone`.

- Derive `Clone` (but NOT `Copy`)
- Derive `Debug` and `PartialEq` for testing

### 4. Document (Clone-only type)

Create a `Document` struct with:

- `title`: `String`
- `pages`: `Vec<String>`

This type contains heap-allocated data, so it should only be `Clone`.

- Derive `Clone` (but NOT `Copy`)
- Derive `Debug` and `PartialEq` for testing

### 5. `TaggedValue<T>` (Generic Clone)

Create a generic `TaggedValue<T>` struct with:

- `tag`: `String`
- `value`: `T`

Implement `Clone` for `TaggedValue<T>` where `T: Clone`. Since it contains a `String`, it cannot be `Copy`.

- Derive `Clone` with appropriate bounds
- Derive `Debug` and `PartialEq` with appropriate bounds

### 6. Utility Functions

Implement these functions to demonstrate the difference between `Clone` and `Copy`:

- `duplicate_copy<T: Copy>(value: T) -> (T, T)` - Returns a tuple with two copies of the value
- `duplicate_clone<T: Clone>(value: &T) -> T` - Returns a clone of the referenced value
- `clone_vec<T: Clone>(items: &[T]) -> Vec<T>` - Clones all items in a slice into a new Vec

## Examples

```rust
// Copy types - original still usable after assignment
let color1 = Color { r: 255, g: 128, b: 0 };
let color2 = color1;  // Copy happens here
assert_eq!(color1.r, 255);  // color1 still valid!

let dims = Dimensions { width: 10.0, height: 20.0 };
let (d1, d2) = duplicate_copy(dims);
assert_eq!(d1.width, d2.width);

// Clone types - must explicitly clone
let label1 = Label { text: String::from("Hello") };
let label2 = label1.clone();  // Explicit clone needed
// label1 is still valid because we cloned, not moved

let doc = Document {
    title: String::from("My Doc"),
    pages: vec![String::from("Page 1")],
};
let doc_copy = duplicate_clone(&doc);
assert_eq!(doc.title, doc_copy.title);

// Generic clone
let tagged = TaggedValue {
    tag: String::from("number"),
    value: 42,
};
let tagged_clone = tagged.clone();
assert_eq!(tagged.value, tagged_clone.value);

// Clone a slice
let numbers = vec![1, 2, 3];
let cloned = clone_vec(&numbers);
assert_eq!(cloned, vec![1, 2, 3]);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `#[derive(Clone, Copy)]` for types where all fields implement `Copy`
- Use `#[derive(Clone)]` (without `Copy`) for types containing `String`, `Vec`, or other heap-allocated types
- For generic types, you may need trait bounds like `#[derive(Clone)]` with `T: Clone`
- The `duplicate_copy` function takes ownership but can return two copies since `T: Copy`
- The `duplicate_clone` function takes a reference and returns a clone
- Use `.iter().cloned().collect()` or `.to_vec()` to clone a slice into a Vec

</details>
