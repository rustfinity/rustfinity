# AsRef and AsMut: Borrowing Conversions

The `AsRef` and `AsMut` traits enable **cheap reference-to-reference conversions**. Unlike `From` and `Into` which transfer ownership, these traits allow you to borrow data as a different type without copying or moving it. This is incredibly useful for writing generic functions that can accept multiple types.

For example, a function that works with paths might want to accept `&str`, `String`, `Path`, or `PathBuf`. Instead of writing four different functions or forcing callers to convert their data, you can use `AsRef<Path>` to accept any of these types.

```rust
pub trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

pub trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}
```

The `?Sized` bound allows these traits to work with dynamically-sized types like `str` and `[T]`.

## Standard Library Examples

```rust
// String implements AsRef<str>, AsRef<[u8]>, and AsRef<Path>
fn print_length<S: AsRef<str>>(s: S) {
    println!("Length: {}", s.as_ref().len());
}

print_length("hello");           // &str
print_length(String::from("hi")); // String

// Vec<T> implements AsRef<[T]>
fn sum_slice<V: AsRef<[i32]>>(v: V) -> i32 {
    v.as_ref().iter().sum()
}

sum_slice(&[1, 2, 3]);           // &[i32]
sum_slice(vec![1, 2, 3]);        // Vec<i32>
```

## Your Task

Implement the following functions and types using `AsRef` and `AsMut`:

### 1. Generic String Length Function

Implement `string_length` that accepts any type that can be borrowed as a string slice:

```rust
pub fn string_length<S: AsRef<str>>(s: S) -> usize
```

### 2. Generic Slice Sum Function

Implement `slice_sum` that sums all elements in anything that can be borrowed as an `i32` slice:

```rust
pub fn slice_sum<V: AsRef<[i32]>>(values: V) -> i32
```

### 3. Generic Contains Function

Implement `contains_element` that checks if a slice contains a specific element:

```rust
pub fn contains_element<V: AsRef<[i32]>>(values: V, target: i32) -> bool
```

### 4. Double All Elements (AsMut)

Implement `double_all` that doubles every element in a mutable slice:

```rust
pub fn double_all<V: AsMut<[i32]>>(values: &mut V)
```

### 5. Custom Text Type with AsRef

Create a `Text` struct that wraps a `String` and implements:

- `AsRef<str>` - borrow as a string slice
- `AsRef<[u8]>` - borrow as a byte slice
- `AsMut<String>` - mutable access to the inner string

```rust
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(s: impl Into<String>) -> Self { ... }
}
```

### 6. Generic Print Function

Implement `print_as_uppercase` that prints any string-like type in uppercase (return the uppercase string, don't actually print):

```rust
pub fn print_as_uppercase<S: AsRef<str>>(s: S) -> String
```

### 7. Append to Buffer (AsMut)

Implement `append_value` that appends an `i32` to anything that can be mutably borrowed as a `Vec<i32>`:

```rust
pub fn append_value<V: AsMut<Vec<i32>>>(buffer: &mut V, value: i32)
```

### 8. Path-like Operations

Implement `get_extension` that extracts a file extension from any path-like type:

```rust
pub fn get_extension<P: AsRef<std::path::Path>>(path: P) -> Option<String>
```

## Examples

```rust
use asref_asmut::*;

// String length works with &str and String
assert_eq!(string_length("hello"), 5);
assert_eq!(string_length(String::from("world")), 5);

// Slice sum works with arrays, slices, and vectors
assert_eq!(slice_sum(&[1, 2, 3]), 6);
assert_eq!(slice_sum(vec![10, 20]), 30);

// Contains element
assert!(contains_element(&[1, 2, 3], 2));
assert!(!contains_element(vec![1, 2, 3], 5));

// Double all elements
let mut arr = [1, 2, 3];
double_all(&mut arr);
assert_eq!(arr, [2, 4, 6]);

let mut vec = vec![5, 10];
double_all(&mut vec);
assert_eq!(vec, vec![10, 20]);

// Text type
let text = Text::new("Hello");
let s: &str = text.as_ref();
assert_eq!(s, "Hello");

let bytes: &[u8] = text.as_ref();
assert_eq!(bytes, b"Hello");

// Uppercase
assert_eq!(print_as_uppercase("hello"), "HELLO");
assert_eq!(print_as_uppercase(String::from("World")), "WORLD");

// Append value
let mut vec = vec![1, 2];
append_value(&mut vec, 3);
assert_eq!(vec, vec![1, 2, 3]);

// Path extension
assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
assert_eq!(
    get_extension(std::path::PathBuf::from("image.png")),
    Some("png".to_string())
);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `s.as_ref()` to get a reference of the target type from a generic parameter
- For `AsRef<str>`, you can call string methods like `.len()`, `.to_uppercase()` on the result of `.as_ref()`
- For `AsMut<[i32]>`, use `.as_mut()` to get a mutable slice, then iterate with `.iter_mut()`
- When implementing `AsRef` for your own type, return a reference to the inner data
- `Vec<T>` implements both `AsRef<[T]>` and `AsMut<[T]>`
- `String` implements `AsRef<str>`, `AsRef<[u8]>`, and `AsRef<Path>`
- For `get_extension`, use `Path::extension()` which returns `Option<&OsStr>`, then convert to `String`
- The `?Sized` bound in the trait definition allows working with unsized types like `str` and `[T]`

</details>
