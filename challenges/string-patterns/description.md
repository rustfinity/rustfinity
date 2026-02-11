Searching for patterns within strings is a fundamental text processing task. Rust's standard library provides several powerful methods for finding substrings, checking prefixes and suffixes, and locating specific patterns within text.

## Pattern Matching Methods

Rust's `str` type offers these key pattern-matching methods:

1. **`.starts_with(pattern)`** - Check if a string begins with a pattern
2. **`.ends_with(pattern)`** - Check if a string ends with a pattern
3. **`.contains(pattern)`** - Check if a pattern exists anywhere in the string
4. **`.find(pattern)`** - Find the first occurrence, returns `Option<usize>` (byte index)
5. **`.rfind(pattern)`** - Find the last occurrence, returns `Option<usize>` (byte index)
6. **`.matches(pattern)`** - Iterate over all matches of a pattern
7. **`.match_indices(pattern)`** - Iterate over matches with their byte positions

These methods accept various pattern types: `&str`, `char`, or closures that take a `char` and return `bool`.

```rust
let text = "hello world, hello universe";

// Prefix/suffix checks
assert!(text.starts_with("hello"));
assert!(text.ends_with("universe"));

// Finding patterns
assert!(text.contains("world"));
assert_eq!(text.find("hello"), Some(0));
assert_eq!(text.rfind("hello"), Some(13));

// Counting matches
assert_eq!(text.matches("hello").count(), 2);
```

## Working with Indices

The `.find()` and `.rfind()` methods return byte indices, not character indices. For ASCII strings these are the same, but for Unicode strings you need to be careful:

```rust
let text = "café";
// byte index, not char index
assert_eq!(
    text.find('é'),
    Some(3)
);
```

## Your Task

Implement the following functions for string pattern matching:

1. `has_prefix(s: &str, prefix: &str) -> bool` - Check if string starts with the given prefix
2. `has_suffix(s: &str, suffix: &str) -> bool` - Check if string ends with the given suffix
3. `find_first(s: &str, pattern: &str) -> Option<usize>` - Find first occurrence of pattern (byte index)
4. `find_last(s: &str, pattern: &str) -> Option<usize>` - Find last occurrence of pattern (byte index)
5. `count_occurrences(s: &str, pattern: &str) -> usize` - Count how many times pattern appears
6. `find_all_indices(s: &str, pattern: &str) -> Vec<usize>` - Find all byte indices where pattern occurs
7. `extract_between(s: &str, start: &str, end: &str) -> Option<String>` - Extract text between first occurrence of start and end markers

## Examples

```rust
// has_prefix
assert!(has_prefix("hello world", "hello"));
assert!(!has_prefix("hello world", "world"));
// empty prefix matches empty string
assert!(has_prefix("", ""));

// has_suffix
assert!(has_suffix("hello world", "world"));
assert!(!has_suffix("hello world", "hello"));
assert!(has_suffix("test.rs", ".rs"));

// find_first
assert_eq!(find_first("hello hello", "hello"), Some(0));
assert_eq!(find_first("hello world", "xyz"), None);
assert_eq!(find_first("abcabc", "bc"), Some(1));

// find_last
assert_eq!(find_last("hello hello", "hello"), Some(6));
assert_eq!(find_last("hello world", "o"), Some(7));
assert_eq!(find_last("test", "xyz"), None);

// count_occurrences
assert_eq!(count_occurrences("abababab", "ab"), 4);
assert_eq!(count_occurrences("hello world", "o"), 2);
assert_eq!(count_occurrences("test", "xyz"), 0);

// find_all_indices
assert_eq!(
    find_all_indices("abcabc", "abc"),
    vec![0, 3]
);
assert_eq!(
    find_all_indices("hello", "l"),
    vec![2, 3]
);
assert_eq!(
    find_all_indices("test", "xyz"),
    vec![]
);

// extract_between
assert_eq!(
    extract_between(
        "<tag>content</tag>",
        "<tag>",
        "</tag>"
    ),
    Some("content".to_string())
);
assert_eq!(
    extract_between("hello [world] test", "[", "]"),
    Some("world".to_string())
);
assert_eq!(
    extract_between("no markers here", "[", "]"),
    None
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `has_prefix` and `has_suffix`, use `.starts_with()` and `.ends_with()` directly
- For `find_first` and `find_last`, use `.find()` and `.rfind()`
- For `count_occurrences`, use `.matches(pattern).count()`
- For `find_all_indices`, use `.match_indices(pattern)`
  and collect the indices
- For `extract_between`:
  1. Use `.find()` to locate the start marker
  2. Calculate where the content begins (after the start marker)
  3. Use slicing and `.find()` again to locate the end marker
  4. Extract the substring between them

</details>
