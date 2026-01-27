# Cow: Copy-on-Write Smart Pointer

`Cow` (Clone-on-Write) is a smart pointer that provides efficient handling of borrowed vs. owned data. It's defined as:

```rust
pub enum Cow<'a, B: ?Sized + ToOwned> {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

The key insight is that `Cow` allows you to work with data that might be borrowed or owned, and only clones when mutation is actually needed. This is a powerful optimization pattern when:

- You often receive data that doesn't need modification
- Cloning is expensive (large strings, vectors)
- You want a unified API that handles both borrowed and owned data

## Key Operations

- `Cow::Borrowed(data)` - Wrap borrowed data
- `Cow::Owned(data)` - Wrap owned data
- `cow.to_mut()` - Get a mutable reference, cloning if borrowed
- `cow.into_owned()` - Convert to owned, cloning if borrowed
- `Cow::from(data)` - Create from &str, String, &[T], Vec<T>, etc.

## When to Use Cow

1. **Functions returning conditionally modified data**: Return borrowed when unchanged, owned when modified
2. **APIs accepting both borrowed and owned**: Single function handles &str and String
3. **Caching/memoization**: Store results that might be borrowed or computed
4. **Configuration defaults**: Use default (borrowed) unless overridden (owned)

## Your Task

Implement the following functions demonstrating `Cow` patterns:

### Part 1: Basic Cow Usage

1. **`maybe_uppercase`** - Convert to uppercase only if any lowercase letters exist
   - Returns `Cow::Borrowed` if already all uppercase (or empty)
   - Returns `Cow::Owned` if conversion was needed

2. **`ensure_suffix`** - Ensure string ends with suffix
   - Returns `Cow::Borrowed` if suffix already present
   - Returns `Cow::Owned` with suffix appended otherwise

3. **`trim_and_lowercase`** - Trim whitespace and convert to lowercase
   - Returns `Cow::Borrowed` only if string is already trimmed and lowercase
   - Returns `Cow::Owned` otherwise

### Part 2: Cow with Collections

4. **`remove_zeros`** - Remove all zeros from a slice
   - Returns `Cow::Borrowed` if no zeros exist
   - Returns `Cow::Owned` with zeros filtered out

5. **`deduplicate_sorted`** - Remove consecutive duplicates from sorted slice
   - Returns `Cow::Borrowed` if no duplicates
   - Returns `Cow::Owned` with duplicates removed

6. **`clamp_values`** - Clamp all values to a range [min, max]
   - Returns `Cow::Borrowed` if all values already in range
   - Returns `Cow::Owned` with clamped values

### Part 3: Modifying Cow with to_mut()

7. **`ensure_capacity`** - Ensure string has at least N characters, padding with char if needed
   - Uses `to_mut()` to modify only when necessary
   - Returns the modified/unmodified Cow

8. **`modify_if_needed`** - Apply a transformation function only if predicate returns true
   - Generic over the transformation
   - Uses `to_mut()` for efficient in-place modification when already owned

### Part 4: Cow in Data Structures

9. **`TextProcessor`** - A struct demonstrating Cow in a real-world scenario
   - Stores text as `Cow<'a, str>`
   - `new(text: &'a str)` - Create with borrowed text
   - `from_owned(text: String)` - Create with owned text
   - `process(&mut self)` - Apply processing (trim + normalize whitespace)
   - `into_string(self)` - Convert to String
   - `is_borrowed(&self)` - Check if currently borrowing

## Examples

```rust
use cow_copy_on_write::*;
use std::borrow::Cow;

// Basic string operations
let result = maybe_uppercase("HELLO");
assert!(matches!(result, Cow::Borrowed(_))); // Already uppercase

let result = maybe_uppercase("Hello");
assert_eq!(result, "HELLO"); // Needed conversion

// Suffix handling
let result = ensure_suffix("file.txt", ".txt");
assert!(matches!(result, Cow::Borrowed(_))); // Already has suffix

let result = ensure_suffix("file", ".txt");
assert_eq!(result, "file.txt");

// Collection operations
let data = [1, 2, 3, 4];
let result = remove_zeros(&data);
assert!(matches!(result, Cow::Borrowed(_))); // No zeros

let data = [0, 1, 0, 2];
let result = remove_zeros(&data);
assert_eq!(result.as_ref(), &[1, 2]);

// Using to_mut() for efficient modification
let cow: Cow<str> = Cow::Borrowed("hi");
let result = ensure_capacity(cow, 5, '!');
assert_eq!(result, "hi!!!");

// TextProcessor
let mut processor = TextProcessor::new("  hello   world  ");
assert!(processor.is_borrowed());
processor.process();
assert!(!processor.is_borrowed()); // Now owned after processing
assert_eq!(processor.into_string(), "hello world");
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `matches!(cow, Cow::Borrowed(_))` to check if borrowed
- `to_mut()` returns `&mut B::Owned` and clones if currently borrowed
- `into_owned()` consumes the Cow and returns owned data
- For strings: `Cow<'a, str>` with `Cow::Borrowed(&str)` and `Cow::Owned(String)`
- For slices: `Cow<'a, [T]>` with `Cow::Borrowed(&[T])` and `Cow::Owned(Vec<T>)`
- Check conditions BEFORE creating Cow to decide borrowed vs owned
- The `Cow::from()` trait creates `Cow::Borrowed` from references and `Cow::Owned` from owned types
- When returning Cow from a function, the lifetime ties to the input reference

</details>
