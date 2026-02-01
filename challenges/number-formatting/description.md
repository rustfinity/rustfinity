# Number Formatting

Formatting numbers for display is a common task in Rust
programming. Whether you're building command-line tools,
generating reports, or creating user interfaces, you'll need
to convert numeric values into human-readable strings with
specific formatting requirements.

Rust's `format!` macro and the `std::fmt` module provide powerful formatting capabilities. You can control padding, alignment, precision for floating-point numbers, and display integers in different bases (binary, octal, hexadecimal). Understanding format specifiers like `{:05}`, `{:.2}`, `{:#x}`, and `{:>10}` is essential for producing well-formatted output.

The formatting traits (`Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex`) work together with format specifiers to give you complete control over how numbers appear in strings.

## Your Task

Implement the following functions that demonstrate various number formatting techniques:

1. **`format_padded`** - Format an integer with leading zeros
   - Takes an `i32` and a width `usize`
   - Returns a `String` with the number padded to the specified width with leading zeros
   - Negative numbers should include the minus sign before the zeros

2. **`format_aligned`** - Format a number with right alignment
   - Takes an `i32` and a width `usize`
   - Returns a `String` with the number right-aligned in a field of the given width
   - Pad with spaces on the left

3. **`format_binary`** - Format an unsigned integer as binary
   - Takes a `u32` and returns a `String` in binary format
   - Should NOT include "0b" prefix

4. **`format_binary_prefixed`** - Format an unsigned integer as binary with prefix
   - Takes a `u32` and returns a `String` in binary format
   - Should include "0b" prefix

5. **`format_hex_lower`** - Format an unsigned integer as lowercase hexadecimal
   - Takes a `u32` and returns a `String` in lowercase hex format
   - Should NOT include "0x" prefix

6. **`format_hex_upper_prefixed`** - Format an unsigned integer as uppercase hex with prefix
   - Takes a `u32` and returns a `String` in uppercase hex format
   - Should include "0x" prefix

7. **`format_octal`** - Format an unsigned integer as octal
   - Takes a `u32` and returns a `String` in octal format
   - Should NOT include "0o" prefix

8. **`format_float_precision`** - Format a floating-point number with fixed decimal places
   - Takes an `f64` and a precision `usize`
   - Returns a `String` with exactly the specified number of decimal places

9. **`format_scientific`** - Format a floating-point number in scientific notation
   - Takes an `f64` and returns a `String` in lowercase scientific notation (e.g., "1.23e4")

10. **`format_currency`** - Format a floating-point number as currency
    - Takes an `f64` representing dollars
    - Returns a `String` in the format "$X.XX" with exactly 2 decimal places
    - Negative amounts should display as "-$X.XX"

## Examples

```rust
use number_formatting::*;

// Padding with zeros
assert_eq!(format_padded(42, 5), "00042");
assert_eq!(format_padded(-7, 4), "-007");
// Wider than width
assert_eq!(format_padded(123, 2), "123");

// Right alignment
assert_eq!(format_aligned(42, 5), "   42");
assert_eq!(format_aligned(-7, 5), "   -7");

// Binary formatting
assert_eq!(format_binary(10), "1010");
assert_eq!(format_binary(255), "11111111");
assert_eq!(format_binary_prefixed(10), "0b1010");

// Hexadecimal formatting
assert_eq!(format_hex_lower(255), "ff");
assert_eq!(format_hex_upper_prefixed(255), "0xFF");

// Octal formatting
assert_eq!(format_octal(8), "10");
assert_eq!(format_octal(63), "77");

// Float precision
assert_eq!(format_float_precision(3.14159, 2), "3.14");
assert_eq!(format_float_precision(2.5, 4), "2.5000");

// Scientific notation
assert_eq!(format_scientific(1234.5), "1.2345e3");
assert_eq!(format_scientific(0.00123), "1.23e-3");

// Currency formatting
assert_eq!(format_currency(19.99), "$19.99");
assert_eq!(format_currency(-5.5), "-$5.50");
assert_eq!(format_currency(1000.0), "$1000.00");
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `format!("{:0width$}", num, width = w)`
  for zero-padded formatting
- Use `format!("{:>width$}", num, width = w)`
  for right-aligned formatting
- Use `format!("{:b}", num)` for binary, `format!("{:x}", num)` for hex, `format!("{:o}", num)` for octal
- The `#` flag adds prefixes: `{:#b}` gives "0b...", `{:#x}` gives "0x..."
- Use `{:X}` for uppercase hex
- Use `format!("{:.prec$}", num, prec = p)` for decimal precision
- Use `format!("{:e}", num)` for scientific notation
- For currency, you'll need to handle negative numbers specially with `abs()` and conditional formatting

</details>
