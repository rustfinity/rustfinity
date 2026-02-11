# Integer Parsing

Parsing integers from strings is a fundamental skill in Rust programming. Whether you're reading configuration files, processing user input, or parsing network protocols, you'll frequently need to convert text into numeric values.

Rust provides several methods for parsing integers. The most common is the `.parse()` method, which uses the `FromStr` trait to convert strings into typed values. For parsing numbers in different bases (binary, octal, hexadecimal), Rust provides the `from_str_radix()` function on integer types.

Understanding how to handle parsing errors is equally important. When parsing fails, Rust returns a `ParseIntError` that you can handle gracefully using `Result` combinators like `ok()`, `map_err()`, or pattern matching.

## Your Task

Implement the following functions that demonstrate various integer parsing techniques:

1. **`parse_decimal`** - Parse a string as a decimal (base 10) i32
   - Returns `Some(value)` on success, `None` on failure
   - Should handle positive and negative numbers

2. **`parse_binary`** - Parse a binary string (e.g., "1010") as a u32
   - Returns `Some(value)` on success, `None` on failure
   - The input should NOT include "0b" prefix

3. **`parse_hex`** - Parse a hexadecimal string (e.g., "FF" or "ff") as a u32
   - Returns `Some(value)` on success, `None` on failure
   - The input should NOT include "0x" prefix
   - Should be case-insensitive

4. **`parse_octal`** - Parse an octal string (e.g., "77") as a u32
   - Returns `Some(value)` on success, `None` on failure
   - The input should NOT include "0o" prefix

5. **`parse_with_radix`** - Parse a string with any radix from 2 to 36
   - Returns `Some(value)` on success, `None` on failure
   - Should handle radix values 2-36

6. **`parse_multiple`** - Parse a comma-separated string of integers
   - Returns a `Vec<i32>` containing all successfully parsed numbers
   - Skip any values that fail to parse (don't return an error)

7. **`try_parse_u8`** - Parse a string as a u8 (0-255)
   - Returns `Ok(value)` on success
   - Returns `Err(String)` with a descriptive error message on failure

8. **`detect_and_parse`** - Automatically detect the base and parse
   - Strings starting with "0x" or "0X" are hexadecimal
   - Strings starting with "0b" or "0B" are binary
   - Strings starting with "0o" or "0O" are octal
   - All other strings are decimal
   - Returns `Some(i64)` on success, `None` on failure

## Examples

```rust
use integer_parsing::*;

// Basic decimal parsing
assert_eq!(parse_decimal("42"), Some(42));
assert_eq!(parse_decimal("-17"), Some(-17));
assert_eq!(parse_decimal("not_a_number"), None);

// Binary parsing
assert_eq!(parse_binary("1010"), Some(10));
assert_eq!(parse_binary("11111111"), Some(255));

// Hexadecimal parsing
assert_eq!(parse_hex("FF"), Some(255));
assert_eq!(parse_hex("ff"), Some(255));
assert_eq!(parse_hex("1a2b"), Some(6699));

// Octal parsing
assert_eq!(parse_octal("77"), Some(63));
assert_eq!(parse_octal("10"), Some(8));

// Custom radix parsing
assert_eq!(parse_with_radix("Z", 36), Some(35));
assert_eq!(parse_with_radix("10", 2), Some(2));

// Multiple values
assert_eq!(parse_multiple("1, 2, 3"), vec![1, 2, 3]);
assert_eq!(parse_multiple("1, bad, 3"), vec![1, 3]);

// Safe u8 parsing with error messages
assert_eq!(try_parse_u8("100"), Ok(100));
assert!(try_parse_u8("256").is_err());  // Overflow

// Auto-detect base
assert_eq!(detect_and_parse("0xFF"), Some(255));
assert_eq!(detect_and_parse("0b1010"), Some(10));
assert_eq!(detect_and_parse("0o77"), Some(63));
assert_eq!(detect_and_parse("42"), Some(42));
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `.parse::<Type>()` for standard decimal parsing
- Use `Type::from_str_radix(s, radix)` for parsing with specific bases
- The `ok()` method converts `Result<T, E>` to `Option<T>`
- For `parse_multiple`, use `.split(',')` and `.filter_map()` to handle failures
- Remember that `from_str_radix` returns a `Result`, so you need to handle the error case
- For `detect_and_parse`, use `strip_prefix()` to remove and check prefixes
- Leading/trailing whitespace can be handled with `.trim()`

</details>
