Rust provides a rich set of methods for manipulating strings. These methods allow you to search, transform, split, and clean string data efficiently.

## Common String Methods

### Trimming Whitespace

The `.trim()` method removes leading and trailing whitespace from a string:

```rust
let messy = "  hello world  ";
assert_eq!(messy.trim(), "hello world");
```

You can also trim only from one side with `.trim_start()` and `.trim_end()`.

### Case Conversion

Convert strings to lowercase or uppercase:

```rust
let mixed = "Hello World";
assert_eq!(mixed.to_lowercase(), "hello world");
assert_eq!(mixed.to_uppercase(), "HELLO WORLD");
```

### Searching

Check if a string contains a substring:

```rust
let text = "Rust is awesome";
assert!(text.contains("awesome"));
assert!(!text.contains("boring"));
```

### Replacing

Replace occurrences of a pattern with another string:

```rust
let original = "hello world";
assert_eq!(original.replace("world", "Rust"), "hello Rust");
```

### Splitting

Split a string into parts based on a delimiter:

```rust
let csv = "apple,banana,cherry";
let fruits: Vec<&str> = csv.split(',').collect();
assert_eq!(fruits, vec!["apple", "banana", "cherry"]);
```

## Your Task

Implement the following functions:

1. `clean_string(s: &str) -> String` - Remove leading and trailing whitespace, then convert to lowercase
2. `contains_word(text: &str, word: &str) -> bool` - Check if the text contains the given word (case-insensitive)
3. `replace_word(text: &str, from: &str, to: &str) -> String` - Replace all occurrences of `from` with `to`
4. `split_and_trim(s: &str, delimiter: char) -> Vec<String>` - Split the string by the delimiter and trim each part
5. `normalize_whitespace(s: &str) -> String` - Replace all sequences of whitespace with a single space, and trim

## Examples

```rust
// clean_string: trim and lowercase
assert_eq!(
    clean_string("  Hello World  "),
    "hello world"
);

// contains_word: case-insensitive search
assert_eq!(
    contains_word("Rust is awesome", "AWESOME"),
    true
);
assert_eq!(
    contains_word("Rust is awesome", "boring"),
    false
);

// replace_word: simple replacement
assert_eq!(
    replace_word("hello world", "world", "Rust"),
    "hello Rust"
);

// split_and_trim: split and clean each part
assert_eq!(
    split_and_trim("apple , banana , cherry", ','),
    vec!["apple", "banana", "cherry"]
);

// normalize_whitespace: clean up messy whitespace
assert_eq!(
    normalize_whitespace("  hello    world  "),
    "hello world"
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `clean_string`, chain `.trim()` and `.to_lowercase()`
- For `contains_word`, convert both strings to the same case before checking
- For `replace_word`, use the `.replace()` method
- For `split_and_trim`, use `.split()` then `.map()` with `.trim()`, and `.collect()`
- For `normalize_whitespace`, use `.split_whitespace()` to automatically handle multiple spaces, then `.collect::<Vec<_>>().join(" ")`

</details>
