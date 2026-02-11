/// Parses a string as a decimal (base 10) i32.
///
/// Returns `Some(value)` on success, `None` on failure.
/// Handles positive and negative numbers and leading/trailing whitespace.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_decimal;
///
/// assert_eq!(parse_decimal("42"), Some(42));
/// assert_eq!(parse_decimal("-17"), Some(-17));
/// assert_eq!(parse_decimal("  123  "), Some(123));
/// assert_eq!(parse_decimal("not_a_number"), None);
/// ```
pub fn parse_decimal(s: &str) -> Option<i32> {
    s.trim().parse().ok()
}

/// Parses a binary string (e.g., "1010") as a u32.
///
/// The input should NOT include the "0b" prefix.
/// Returns `Some(value)` on success, `None` on failure.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_binary;
///
/// assert_eq!(parse_binary("1010"), Some(10));
/// assert_eq!(parse_binary("11111111"), Some(255));
/// assert_eq!(parse_binary("0"), Some(0));
/// assert_eq!(parse_binary("102"), None);  // Invalid binary digit
/// ```
pub fn parse_binary(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim(), 2).ok()
}

/// Parses a hexadecimal string (e.g., "FF" or "ff") as a u32.
///
/// The input should NOT include the "0x" prefix.
/// Case-insensitive: both "FF" and "ff" work.
/// Returns `Some(value)` on success, `None` on failure.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_hex;
///
/// assert_eq!(parse_hex("FF"), Some(255));
/// assert_eq!(parse_hex("ff"), Some(255));
/// assert_eq!(parse_hex("1a2b"), Some(6699));
/// assert_eq!(parse_hex("0"), Some(0));
/// assert_eq!(parse_hex("GG"), None);  // Invalid hex digit
/// ```
pub fn parse_hex(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim(), 16).ok()
}

/// Parses an octal string (e.g., "77") as a u32.
///
/// The input should NOT include the "0o" prefix.
/// Returns `Some(value)` on success, `None` on failure.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_octal;
///
/// assert_eq!(parse_octal("77"), Some(63));
/// assert_eq!(parse_octal("10"), Some(8));
/// assert_eq!(parse_octal("0"), Some(0));
/// assert_eq!(parse_octal("89"), None);  // Invalid octal digit
/// ```
pub fn parse_octal(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim(), 8).ok()
}

/// Parses a string with any radix from 2 to 36.
///
/// Returns `Some(value)` on success, `None` on failure.
/// Valid radix values are 2 through 36.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_with_radix;
///
/// assert_eq!(parse_with_radix("Z", 36), Some(35));
/// assert_eq!(parse_with_radix("10", 2), Some(2));
/// assert_eq!(parse_with_radix("10", 10), Some(10));
/// assert_eq!(parse_with_radix("10", 16), Some(16));
/// ```
pub fn parse_with_radix(s: &str, radix: u32) -> Option<u32> {
    if !(2..=36).contains(&radix) {
        return None;
    }
    u32::from_str_radix(s.trim(), radix).ok()
}

/// Parses a comma-separated string of integers.
///
/// Returns a `Vec<i32>` containing all successfully parsed numbers.
/// Values that fail to parse are silently skipped.
///
/// # Examples
///
/// ```
/// use integer_parsing::parse_multiple;
///
/// assert_eq!(parse_multiple("1, 2, 3"), vec![1, 2, 3]);
/// assert_eq!(parse_multiple("1, bad, 3"), vec![1, 3]);
/// assert_eq!(parse_multiple(""), Vec::<i32>::new());
/// assert_eq!(parse_multiple("  10 , -20 , 30  "), vec![10, -20, 30]);
/// ```
pub fn parse_multiple(s: &str) -> Vec<i32> {
    s.split(',')
        .filter_map(|part| part.trim().parse().ok())
        .collect()
}

/// Parses a string as a u8 (0-255).
///
/// Returns `Ok(value)` on success.
/// Returns `Err(String)` with a descriptive error message on failure.
///
/// # Examples
///
/// ```
/// use integer_parsing::try_parse_u8;
///
/// assert_eq!(try_parse_u8("100"), Ok(100));
/// assert_eq!(try_parse_u8("0"), Ok(0));
/// assert_eq!(try_parse_u8("255"), Ok(255));
/// assert!(try_parse_u8("256").is_err());
/// assert!(try_parse_u8("-1").is_err());
/// assert!(try_parse_u8("abc").is_err());
/// ```
pub fn try_parse_u8(s: &str) -> Result<u8, String> {
    s.trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("Failed to parse '{}': {}", s.trim(), e))
}

/// Automatically detects the base and parses the integer.
///
/// - Strings starting with "0x" or "0X" are hexadecimal
/// - Strings starting with "0b" or "0B" are binary
/// - Strings starting with "0o" or "0O" are octal
/// - All other strings are decimal
///
/// Returns `Some(i64)` on success, `None` on failure.
///
/// # Examples
///
/// ```
/// use integer_parsing::detect_and_parse;
///
/// assert_eq!(detect_and_parse("0xFF"), Some(255));
/// assert_eq!(detect_and_parse("0XFF"), Some(255));
/// assert_eq!(detect_and_parse("0b1010"), Some(10));
/// assert_eq!(detect_and_parse("0B1010"), Some(10));
/// assert_eq!(detect_and_parse("0o77"), Some(63));
/// assert_eq!(detect_and_parse("0O77"), Some(63));
/// assert_eq!(detect_and_parse("42"), Some(42));
/// assert_eq!(detect_and_parse("-42"), Some(-42));
/// ```
pub fn detect_and_parse(s: &str) -> Option<i64> {
    let s = s.trim();

    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        i64::from_str_radix(bin, 2).ok()
    } else if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        i64::from_str_radix(oct, 8).ok()
    } else {
        s.parse().ok()
    }
}
