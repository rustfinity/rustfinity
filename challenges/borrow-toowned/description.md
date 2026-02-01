# Borrow and ToOwned: Owned vs Borrowed Patterns

The `Borrow` and `ToOwned` traits provide a powerful abstraction for working with borrowed and owned data interchangeably. Unlike `AsRef` which provides simple reference conversions, `Borrow` has a stronger semantic guarantee: the borrowed form must hash and compare the same as the owned form. This makes it essential for collection types like `HashMap` and `HashSet`.

The `ToOwned` trait is the counterpart to `Borrow` - it allows creating owned data from borrowed data. While `Clone` creates an owned copy of `&T` to `T`, `ToOwned` generalizes this to create owned data from any borrowed form (e.g., `&str` to `String`, `&[T]` to `Vec<T>`).

```rust
pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}
```

## The Cow Type: Clone-on-Write

`Cow` (Clone on Write) is a smart pointer that can hold either borrowed or owned data. It's perfect for functions that sometimes need to modify data and sometimes don't. `Cow` defers cloning until mutation is actually needed, providing both flexibility and performance.

```rust
use std::borrow::Cow;

fn process_name(name: &str) -> Cow<str> {
    if name.contains(' ') {
        // Needs modification - return owned
        Cow::Owned(name.replace(' ', "_"))
    } else {
        // No modification - return borrowed
        Cow::Borrowed(name)
    }
}
```

## Your Task

Implement the following functions and types using `Borrow`, `ToOwned`, and `Cow`:

### 1. Generic Lookup Function

Implement `lookup` that searches a `HashMap` using any type that can be borrowed as the key type:

```rust
pub fn lookup<'a, K, V, Q>(
    map: &'a HashMap<K, V>,
    key: &Q
) -> Option<&'a V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
```

### 2. To Owned String

Implement `make_owned` that converts any borrowed string-like data to an owned `String`:

```rust
pub fn make_owned(s: &str) -> String
```

### 3. Normalize Whitespace with Cow

Implement `normalize_whitespace` that normalizes multiple spaces to single spaces. Use `Cow` to avoid allocation if no changes are needed:

```rust
pub fn normalize_whitespace(input: &str) -> Cow<str>
```

### 4. Ensure Prefix with Cow

Implement `ensure_prefix` that ensures a string starts with a given prefix. Use `Cow` to return borrowed data when the prefix already exists:

```rust
pub fn ensure_prefix<'a>(
    s: &'a str,
    prefix: &str
) -> Cow<'a, str>
```

### 5. Custom Type with Borrow

Create a `CaseInsensitiveString` type that implements `Borrow<str>` such that lookups in collections work case-insensitively:

```rust
pub struct CaseInsensitiveString(String);

impl CaseInsensitiveString {
    pub fn new(s: impl Into<String>) -> Self;
}
```

The type should implement `Borrow<str>`, `Hash`, and `Eq` such that "Hello" and "HELLO" are considered equal.

### 6. Clone on Write Vector

Implement `append_if_missing` that appends an element to a slice only if it's not already present. Use `Cow` to avoid cloning the entire vector when the element exists:

```rust
pub fn append_if_missing<'a, T>(
    items: &'a [T],
    value: T
) -> Cow<'a, [T]>
where
    T: Clone + PartialEq,
```

### 7. Process Path with Cow

Implement `normalize_path` that removes redundant `./` from a path. Use `Cow` for efficiency:

```rust
pub fn normalize_path(path: &str) -> Cow<str>
```

### 8. Convert Borrowed to Owned

Implement `to_owned_vec` that demonstrates `ToOwned` by converting a slice to an owned vector:

```rust
pub fn to_owned_vec<T: Clone>(slice: &[T]) -> Vec<T>
```

## Examples

```rust
use borrow_toowned::*;
use std::collections::HashMap;
use std::borrow::Cow;

// Generic lookup with Borrow
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("hello".to_string(), 42);
// &str key for String map
assert_eq!(lookup(&map, "hello"), Some(&42));

// To owned
let owned = make_owned("hello");
assert_eq!(owned, "hello".to_string());

// Normalize whitespace - no allocation when unchanged
let result = normalize_whitespace("hello world");
assert!(matches!(result, Cow::Borrowed(_)));

let result = normalize_whitespace("hello   world");
assert!(matches!(result, Cow::Owned(_)));
assert_eq!(result, "hello world");

// Ensure prefix
let result = ensure_prefix("hello", "hello");
assert!(matches!(result, Cow::Borrowed(_)));

let result = ensure_prefix("world", "hello_");
assert_eq!(result, "hello_world");

// Case insensitive string in HashMap
let mut map: HashMap<CaseInsensitiveString, i32> =
    HashMap::new();
map.insert(
    CaseInsensitiveString::new("Hello"),
    1
);
assert_eq!(
    map.get(&CaseInsensitiveString::new(
        "HELLO"
    )),
    Some(&1)
);

// Append if missing
let items = [1, 2, 3];
let result = append_if_missing(&items, 2);
// Already exists
assert!(matches!(result, Cow::Borrowed(_)));

let result = append_if_missing(&items, 4);
assert_eq!(result.as_ref(), &[1, 2, 3, 4]);

// Normalize path
assert_eq!(normalize_path("./foo/./bar"), "foo/bar");
assert!(matches!(normalize_path("foo/bar"), Cow::Borrowed(_)));

// To owned vec
let slice = &[1, 2, 3];
let vec = to_owned_vec(slice);
assert_eq!(vec, vec![1, 2, 3]);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- For `lookup`, use `HashMap::get()` which accepts
  any type `Q` where `K: Borrow<Q>`
- `ToOwned::to_owned()` is implemented for `str`
  (returns `String`) and `[T]` (returns `Vec<T>`)
- For `Cow`, use `Cow::Borrowed(data)` when no
  modification is needed
- Use `Cow::Owned(modified_data)` when you need
  to return modified data
- For `CaseInsensitiveString`, the key insight
  is that `Hash` and `Eq` must be based on the
  lowercase form
- When implementing `Borrow<str>` for
  `CaseInsensitiveString`, note that the
  `borrow()` method must return `&str` - this
  is the internal string, not the lowercase
  version. The equality and hashing are what
  make lookups work case-insensitively.
- `Cow<'a, str>` implements `Deref<Target = str>`,
  so you can use string methods directly on it
- For checking if modifications are needed, do a
  pass to detect first, then only allocate if
  necessary

</details>
