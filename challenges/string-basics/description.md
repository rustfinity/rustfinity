In Rust, there are two main string types: `String` and `&str`. Understanding the difference between them is fundamental to working with text in Rust.

- **`String`** is an owned, heap-allocated, growable string type. You have full control over it and can modify its contents.
- **`&str`** (string slice) is a borrowed reference to a string. It's typically used for string literals and when you want to view string data without owning it.

## Converting Between String Types

You can create a `String` from a `&str` in several ways:

```rust
let s1 = "hello".to_string();  // Using .to_string()
let s2 = String::from("hello"); // Using String::from()
```

## Iterating Over Strings

Rust strings are UTF-8 encoded, which means characters can be 1-4 bytes. You can iterate over:

- **Characters** using `.chars()` - gives you each Unicode scalar value
- **Bytes** using `.bytes()` - gives you each raw byte

```rust
let s = "hello";
for c in s.chars() {
    println!("{}", c); // prints: h, e, l, l, o
}
for b in s.bytes() {
    println!("{}", b); // prints: 104, 101, 108, 108, 111
}
```

## Checking ASCII

You can check if a string contains only ASCII characters using `.is_ascii()`:

```rust
let ascii_str = "Hello";
let unicode_str = "Hello, world!";

assert!(ascii_str.is_ascii());     // true
assert!(unicode_str.is_ascii());   // true - common punctuation is ASCII
```

## Your Task

Implement the following functions:

1. `to_owned_string(s: &str) -> String` - Convert a string slice to an owned `String`
2. `count_chars(s: &str) -> usize` - Count the number of characters in a string
3. `count_bytes(s: &str) -> usize` - Count the number of bytes in a string
4. `is_ascii_only(s: &str) -> bool` - Check if a string contains only ASCII characters
5. `first_char(s: &str) -> Option<char>` - Return the first character of a string, or `None` if empty

## Examples

```rust
assert_eq!(to_owned_string("hello"), String::from("hello"));
assert_eq!(count_chars("hello"), 5);
// 5 Unicode characters (cafe + combining accent)
assert_eq!(count_chars("cafe\u{0301}"), 5);
assert_eq!(count_bytes("hello"), 5);
// 5 bytes ('c', 'a', 'f' are 1 byte, 'e with accent' is 2 bytes)
assert_eq!(count_bytes("caf\u{00E9}"), 5);
assert_eq!(is_ascii_only("hello"), true);
// accented 'e' is not ASCII
assert_eq!(is_ascii_only("caf\u{00E9}"), false);
assert_eq!(first_char("hello"), Some('h'));
assert_eq!(first_char(""), None);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `.to_string()` or `String::from()` to convert `&str` to `String`
- Use `.chars().count()` to count Unicode characters
- Use `.bytes().count()` or `.len()` to count bytes
- Use `.is_ascii()` method on `&str` to check ASCII
- Use `.chars().next()` to get the first character as an `Option<char>`

</details>
