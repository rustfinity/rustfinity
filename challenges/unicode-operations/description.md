When working with strings in Rust, understanding the difference between bytes, characters, and grapheme clusters is essential for writing correct text-processing code. Many programming bugs stem from treating strings as simple arrays of characters, when in reality UTF-8 strings have a much richer structure.

## Bytes vs Characters vs Grapheme Clusters

In Rust, a `String` is a sequence of UTF-8 encoded bytes. There are three ways to view string data:

1. **Bytes** (`.bytes()`) - The raw UTF-8 bytes. ASCII characters are 1 byte, but many Unicode characters take 2-4 bytes.
2. **Characters** (`.chars()`) - Unicode scalar values. Most characters are single `char`s, but some displayed characters (like emojis with modifiers) are multiple `char`s.
3. **Grapheme Clusters** - What humans perceive as a single "character". A family emoji like "👨‍👩‍👧" is multiple Unicode code points combined.

```rust
let text = "Hello";
assert_eq!(text.len(), 5);        // 5 bytes
assert_eq!(text.chars().count(), 5); // 5 characters

let emoji = "👨‍👩‍👧";  // Family emoji (ZWJ sequence)
assert_eq!(emoji.len(), 18);       // 18 bytes!
assert_eq!(emoji.chars().count(), 5); // 5 Unicode scalars
// But visually it's 1 "character" (grapheme cluster)
```

## Safe String Slicing

String slicing in Rust must occur at valid UTF-8 boundaries. Slicing in the middle of a multi-byte character causes a panic:

```rust
let text = "Здравствуйте"; // Russian "Hello"
// text[0..1] would panic! 'З' is 2 bytes
let slice = &text[0..2]; // OK - takes full first character
assert_eq!(slice, "З");
```

To safely extract substrings, use `.chars()` with indices or the `.char_indices()` method:

```rust
let text = "Здравствуйте";
let chars: Vec<char> = text.chars().collect();
let first_two: String = chars[..2].iter().collect();
assert_eq!(first_two, "Зд");
```

## Your Task

Implement the following functions to demonstrate Unicode-aware string handling:

1. `char_count(s: &str) -> usize` - Count the number of Unicode characters (not bytes)
2. `byte_count(s: &str) -> usize` - Count the number of bytes in the UTF-8 encoding
3. `safe_substring(s: &str, start: usize, end: usize) -> Option<String>` - Extract a substring by character indices (not byte indices). Return `None` if indices are out of bounds.
4. `char_at(s: &str, index: usize) -> Option<char>` - Get the character at a specific index (by character position, not byte position)
5. `is_single_char(s: &str) -> bool` - Check if a string contains exactly one Unicode character

## Examples

```rust
// char_count
assert_eq!(char_count("Hello"), 5);
assert_eq!(char_count("Привет"), 6);  // Russian "Hello"
assert_eq!(char_count("你好"), 2);     // Chinese "Hello"
assert_eq!(char_count("🎉"), 1);       // Single emoji

// byte_count
assert_eq!(byte_count("Hello"), 5);    // ASCII: 1 byte each
assert_eq!(byte_count("Привет"), 12);  // Cyrillic: 2 bytes each
assert_eq!(byte_count("你好"), 6);      // Chinese: 3 bytes each
assert_eq!(byte_count("🎉"), 4);        // Emoji: 4 bytes

// safe_substring
assert_eq!(safe_substring("Hello", 0, 3), Some("Hel".to_string()));
assert_eq!(safe_substring("Привет", 0, 2), Some("Пр".to_string()));
assert_eq!(safe_substring("Hello", 0, 10), None);  // Out of bounds
assert_eq!(safe_substring("Hello", 3, 2), None);   // Invalid range

// char_at
assert_eq!(char_at("Hello", 0), Some('H'));
assert_eq!(char_at("Привет", 2), Some('и'));
assert_eq!(char_at("Hello", 10), None);

// is_single_char
assert_eq!(is_single_char("a"), true);
assert_eq!(is_single_char("好"), true);
assert_eq!(is_single_char("🎉"), true);
assert_eq!(is_single_char("ab"), false);
assert_eq!(is_single_char(""), false);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `.chars().count()` to count Unicode characters, not `.len()` which counts bytes
- Use `.len()` or `.bytes().count()` for byte count
- For `safe_substring`, collect characters into a `Vec<char>` first, then validate indices before extracting
- Use `.chars().nth(index)` to get a character at a specific position
- For `is_single_char`, check that the char count equals exactly 1

</details>
