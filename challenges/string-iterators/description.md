Rust strings provide several powerful iterator methods that let you process text character by character, word by word, or line by line. Understanding these iteration patterns is fundamental to text processing in Rust.

## String Iterators Overview

Rust's `str` type provides several methods that return iterators:

1. **`.chars()`** - Iterates over Unicode scalar values (characters)
2. **`.split_whitespace()`** - Splits on Unicode whitespace, skipping empty segments
3. **`.lines()`** - Splits on line endings (`\n` or `\r\n`)
4. **`.split(pattern)`** - Splits on a custom delimiter

These iterators are lazy - they don't do any work until you consume them with methods like `.collect()`, `.count()`, or a `for` loop.

```rust
let text = "hello world";

// Iterate over characters
for ch in text.chars() {
    println!("{}", ch);
}

// Iterate over words
for word in text.split_whitespace() {
    println!("{}", word);
}

// Collect into a Vec
let chars: Vec<char> = text.chars().collect();
let words: Vec<&str> = text.split_whitespace().collect();
```

## Transforming While Iterating

You can chain iterator methods to transform data as you iterate:

```rust
let text = "  hello  world  ";

// Get uppercase words
let upper_words: Vec<String> = text
    .split_whitespace()
    .map(|w| w.to_uppercase())
    .collect();
assert_eq!(upper_words, vec!["HELLO", "WORLD"]);
```

## Your Task

Implement the following functions that demonstrate string iteration patterns:

1. `chars_to_vec(s: &str) -> Vec<char>` - Convert a string into a vector of its characters
2. `words_to_vec(s: &str) -> Vec<String>` - Split a string on whitespace and collect into owned strings
3. `lines_to_vec(s: &str) -> Vec<String>` - Split a string on line endings and collect into owned strings
4. `count_words(s: &str) -> usize` - Count the number of whitespace-separated words
5. `reverse_words(s: &str) -> String` - Reverse the order of words in a string (keep words themselves intact)
6. `capitalize_words(s: &str) -> String` - Capitalize the first letter of each word

## Examples

```rust
// chars_to_vec
assert_eq!(chars_to_vec("hi"), vec!['h', 'i']);
assert_eq!(chars_to_vec(""), Vec::<char>::new());

// words_to_vec
assert_eq!(words_to_vec("hello world"), vec!["hello", "world"]);
assert_eq!(words_to_vec("  spaces  everywhere  "), vec!["spaces", "everywhere"]);

// lines_to_vec
assert_eq!(lines_to_vec("line1\nline2\nline3"), vec!["line1", "line2", "line3"]);
assert_eq!(lines_to_vec("single"), vec!["single"]);

// count_words
assert_eq!(count_words("one two three"), 3);
assert_eq!(count_words(""), 0);
assert_eq!(count_words("   "), 0);

// reverse_words
assert_eq!(reverse_words("hello world"), "world hello");
assert_eq!(reverse_words("one two three"), "three two one");

// capitalize_words
assert_eq!(capitalize_words("hello world"), "Hello World");
assert_eq!(capitalize_words("rust is great"), "Rust Is Great");
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `chars_to_vec`, use `.chars().collect()`
- For `words_to_vec`, use `.split_whitespace()` and `.map(|s| s.to_string())` before `.collect()`
- For `lines_to_vec`, use `.lines()` and convert each `&str` to `String`
- For `count_words`, use `.split_whitespace().count()`
- For `reverse_words`, collect words into a Vec, reverse it, then join with spaces
- For `capitalize_words`, for each word, make the first character uppercase and the rest lowercase, then join with spaces

</details>
