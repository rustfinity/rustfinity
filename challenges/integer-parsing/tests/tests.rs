use integer_parsing::*;

// =============================================================================
// Tests for parse_decimal
// =============================================================================

#[test]
fn parse_decimal_positive() {
    assert_eq!(parse_decimal("42"), Some(42));
}

#[test]
fn parse_decimal_negative() {
    assert_eq!(parse_decimal("-17"), Some(-17));
}

#[test]
fn parse_decimal_zero() {
    assert_eq!(parse_decimal("0"), Some(0));
}

#[test]
fn parse_decimal_with_whitespace() {
    assert_eq!(parse_decimal("  123  "), Some(123));
    assert_eq!(parse_decimal("\t456\n"), Some(456));
}

#[test]
fn parse_decimal_large_number() {
    assert_eq!(parse_decimal("2147483647"), Some(i32::MAX));
    assert_eq!(parse_decimal("-2147483648"), Some(i32::MIN));
}

#[test]
fn parse_decimal_invalid() {
    assert_eq!(parse_decimal("not_a_number"), None);
    assert_eq!(parse_decimal("12.34"), None);
    assert_eq!(parse_decimal("12abc"), None);
    assert_eq!(parse_decimal(""), None);
}

#[test]
fn parse_decimal_overflow() {
    assert_eq!(parse_decimal("2147483648"), None); // i32::MAX + 1
    assert_eq!(parse_decimal("-2147483649"), None); // i32::MIN - 1
}

// =============================================================================
// Tests for parse_binary
// =============================================================================

#[test]
fn parse_binary_basic() {
    assert_eq!(parse_binary("1010"), Some(10));
    assert_eq!(parse_binary("11111111"), Some(255));
}

#[test]
fn parse_binary_zero() {
    assert_eq!(parse_binary("0"), Some(0));
}

#[test]
fn parse_binary_one() {
    assert_eq!(parse_binary("1"), Some(1));
}

#[test]
fn parse_binary_max_u32() {
    assert_eq!(parse_binary("11111111111111111111111111111111"), Some(u32::MAX));
}

#[test]
fn parse_binary_with_whitespace() {
    assert_eq!(parse_binary("  1010  "), Some(10));
}

#[test]
fn parse_binary_invalid_digit() {
    assert_eq!(parse_binary("102"), None);
    assert_eq!(parse_binary("2"), None);
    assert_eq!(parse_binary("abc"), None);
}

#[test]
fn parse_binary_empty() {
    assert_eq!(parse_binary(""), None);
}

// =============================================================================
// Tests for parse_hex
// =============================================================================

#[test]
fn parse_hex_uppercase() {
    assert_eq!(parse_hex("FF"), Some(255));
    assert_eq!(parse_hex("DEADBEEF"), Some(0xDEADBEEF));
}

#[test]
fn parse_hex_lowercase() {
    assert_eq!(parse_hex("ff"), Some(255));
    assert_eq!(parse_hex("deadbeef"), Some(0xDEADBEEF));
}

#[test]
fn parse_hex_mixed_case() {
    assert_eq!(parse_hex("DeAdBeEf"), Some(0xDEADBEEF));
}

#[test]
fn parse_hex_zero() {
    assert_eq!(parse_hex("0"), Some(0));
}

#[test]
fn parse_hex_numbers_only() {
    assert_eq!(parse_hex("1234"), Some(0x1234));
}

#[test]
fn parse_hex_with_whitespace() {
    assert_eq!(parse_hex("  FF  "), Some(255));
}

#[test]
fn parse_hex_invalid() {
    assert_eq!(parse_hex("GG"), None);
    assert_eq!(parse_hex("xyz"), None);
    assert_eq!(parse_hex(""), None);
}

#[test]
fn parse_hex_max_u32() {
    assert_eq!(parse_hex("FFFFFFFF"), Some(u32::MAX));
}

// =============================================================================
// Tests for parse_octal
// =============================================================================

#[test]
fn parse_octal_basic() {
    assert_eq!(parse_octal("77"), Some(63));
    assert_eq!(parse_octal("10"), Some(8));
}

#[test]
fn parse_octal_zero() {
    assert_eq!(parse_octal("0"), Some(0));
}

#[test]
fn parse_octal_one() {
    assert_eq!(parse_octal("1"), Some(1));
}

#[test]
fn parse_octal_permissions() {
    assert_eq!(parse_octal("755"), Some(0o755));
    assert_eq!(parse_octal("644"), Some(0o644));
}

#[test]
fn parse_octal_with_whitespace() {
    assert_eq!(parse_octal("  77  "), Some(63));
}

#[test]
fn parse_octal_invalid_digit() {
    assert_eq!(parse_octal("89"), None);
    assert_eq!(parse_octal("8"), None);
    assert_eq!(parse_octal("abc"), None);
}

#[test]
fn parse_octal_empty() {
    assert_eq!(parse_octal(""), None);
}

// =============================================================================
// Tests for parse_with_radix
// =============================================================================

#[test]
fn parse_with_radix_binary() {
    assert_eq!(parse_with_radix("10", 2), Some(2));
    assert_eq!(parse_with_radix("1111", 2), Some(15));
}

#[test]
fn parse_with_radix_decimal() {
    assert_eq!(parse_with_radix("10", 10), Some(10));
    assert_eq!(parse_with_radix("255", 10), Some(255));
}

#[test]
fn parse_with_radix_hex() {
    assert_eq!(parse_with_radix("10", 16), Some(16));
    assert_eq!(parse_with_radix("FF", 16), Some(255));
}

#[test]
fn parse_with_radix_base36() {
    assert_eq!(parse_with_radix("Z", 36), Some(35));
    assert_eq!(parse_with_radix("10", 36), Some(36));
    assert_eq!(parse_with_radix("zz", 36), Some(1295));
}

#[test]
fn parse_with_radix_base3() {
    assert_eq!(parse_with_radix("10", 3), Some(3));
    assert_eq!(parse_with_radix("22", 3), Some(8));
}

#[test]
fn parse_with_radix_zero() {
    assert_eq!(parse_with_radix("0", 2), Some(0));
    assert_eq!(parse_with_radix("0", 16), Some(0));
    assert_eq!(parse_with_radix("0", 36), Some(0));
}

#[test]
fn parse_with_radix_invalid_radix() {
    assert_eq!(parse_with_radix("10", 1), None);
    assert_eq!(parse_with_radix("10", 37), None);
    assert_eq!(parse_with_radix("10", 0), None);
}

#[test]
fn parse_with_radix_invalid_digit() {
    assert_eq!(parse_with_radix("2", 2), None);
    assert_eq!(parse_with_radix("A", 10), None);
}

#[test]
fn parse_with_radix_with_whitespace() {
    assert_eq!(parse_with_radix("  10  ", 16), Some(16));
}

// =============================================================================
// Tests for parse_multiple
// =============================================================================

#[test]
fn parse_multiple_basic() {
    assert_eq!(parse_multiple("1, 2, 3"), vec![1, 2, 3]);
}

#[test]
fn parse_multiple_with_invalid() {
    assert_eq!(parse_multiple("1, bad, 3"), vec![1, 3]);
    assert_eq!(parse_multiple("a, b, c"), Vec::<i32>::new());
}

#[test]
fn parse_multiple_empty() {
    assert_eq!(parse_multiple(""), Vec::<i32>::new());
}

#[test]
fn parse_multiple_single() {
    assert_eq!(parse_multiple("42"), vec![42]);
}

#[test]
fn parse_multiple_with_whitespace() {
    assert_eq!(parse_multiple("  10 , -20 , 30  "), vec![10, -20, 30]);
}

#[test]
fn parse_multiple_negative_numbers() {
    assert_eq!(parse_multiple("-1, -2, -3"), vec![-1, -2, -3]);
}

#[test]
fn parse_multiple_mixed() {
    assert_eq!(parse_multiple("1, -2, bad, 3, , 4"), vec![1, -2, 3, 4]);
}

#[test]
fn parse_multiple_trailing_comma() {
    assert_eq!(parse_multiple("1, 2, 3,"), vec![1, 2, 3]);
}

#[test]
fn parse_multiple_leading_comma() {
    assert_eq!(parse_multiple(",1, 2, 3"), vec![1, 2, 3]);
}

// =============================================================================
// Tests for try_parse_u8
// =============================================================================

#[test]
fn try_parse_u8_valid() {
    assert_eq!(try_parse_u8("100"), Ok(100));
    assert_eq!(try_parse_u8("0"), Ok(0));
    assert_eq!(try_parse_u8("255"), Ok(255));
}

#[test]
fn try_parse_u8_boundary() {
    assert_eq!(try_parse_u8("0"), Ok(0));
    assert_eq!(try_parse_u8("255"), Ok(255));
}

#[test]
fn try_parse_u8_with_whitespace() {
    assert_eq!(try_parse_u8("  100  "), Ok(100));
}

#[test]
fn try_parse_u8_overflow() {
    let result = try_parse_u8("256");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("256"));
}

#[test]
fn try_parse_u8_negative() {
    let result = try_parse_u8("-1");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("-1"));
}

#[test]
fn try_parse_u8_invalid() {
    let result = try_parse_u8("abc");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("abc"));
}

#[test]
fn try_parse_u8_empty() {
    assert!(try_parse_u8("").is_err());
}

#[test]
fn try_parse_u8_large_overflow() {
    let result = try_parse_u8("1000");
    assert!(result.is_err());
}

// =============================================================================
// Tests for detect_and_parse
// =============================================================================

#[test]
fn detect_and_parse_decimal() {
    assert_eq!(detect_and_parse("42"), Some(42));
    assert_eq!(detect_and_parse("-42"), Some(-42));
    assert_eq!(detect_and_parse("0"), Some(0));
}

#[test]
fn detect_and_parse_hex_lowercase() {
    assert_eq!(detect_and_parse("0xff"), Some(255));
    assert_eq!(detect_and_parse("0xdeadbeef"), Some(0xDEADBEEF));
}

#[test]
fn detect_and_parse_hex_uppercase() {
    assert_eq!(detect_and_parse("0XFF"), Some(255));
    assert_eq!(detect_and_parse("0XDEADBEEF"), Some(0xDEADBEEF));
}

#[test]
fn detect_and_parse_binary_lowercase() {
    assert_eq!(detect_and_parse("0b1010"), Some(10));
    assert_eq!(detect_and_parse("0b11111111"), Some(255));
}

#[test]
fn detect_and_parse_binary_uppercase() {
    assert_eq!(detect_and_parse("0B1010"), Some(10));
    assert_eq!(detect_and_parse("0B11111111"), Some(255));
}

#[test]
fn detect_and_parse_octal_lowercase() {
    assert_eq!(detect_and_parse("0o77"), Some(63));
    assert_eq!(detect_and_parse("0o10"), Some(8));
}

#[test]
fn detect_and_parse_octal_uppercase() {
    assert_eq!(detect_and_parse("0O77"), Some(63));
    assert_eq!(detect_and_parse("0O10"), Some(8));
}

#[test]
fn detect_and_parse_with_whitespace() {
    assert_eq!(detect_and_parse("  0xFF  "), Some(255));
    assert_eq!(detect_and_parse("  42  "), Some(42));
}

#[test]
fn detect_and_parse_invalid() {
    assert_eq!(detect_and_parse("0xGG"), None);
    assert_eq!(detect_and_parse("0b123"), None);
    assert_eq!(detect_and_parse("abc"), None);
    assert_eq!(detect_and_parse(""), None);
}

#[test]
fn detect_and_parse_large_numbers() {
    assert_eq!(detect_and_parse("9223372036854775807"), Some(i64::MAX));
    assert_eq!(detect_and_parse("-9223372036854775808"), Some(i64::MIN));
}

#[test]
fn detect_and_parse_zero_variants() {
    assert_eq!(detect_and_parse("0x0"), Some(0));
    assert_eq!(detect_and_parse("0b0"), Some(0));
    assert_eq!(detect_and_parse("0o0"), Some(0));
}

// =============================================================================
// Integration tests
// =============================================================================

#[test]
fn integration_same_value_different_bases() {
    // 255 in different bases
    assert_eq!(parse_decimal("255"), Some(255));
    assert_eq!(parse_binary("11111111"), Some(255));
    assert_eq!(parse_hex("FF"), Some(255));
    assert_eq!(parse_octal("377"), Some(255));
    assert_eq!(parse_with_radix("73", 36), Some(255)); // 7*36 + 3 = 255
}

#[test]
fn integration_detect_and_parse_equivalence() {
    // Verify detect_and_parse gives same results as specific parsers
    assert_eq!(detect_and_parse("0xFF"), Some(parse_hex("FF").unwrap() as i64));
    assert_eq!(detect_and_parse("0b1010"), Some(parse_binary("1010").unwrap() as i64));
    assert_eq!(detect_and_parse("0o77"), Some(parse_octal("77").unwrap() as i64));
}

#[test]
fn integration_parse_config_values() {
    // Simulating parsing a config with mixed formats
    let values = vec!["0xFF", "256", "0b1000", "0o777"];
    let parsed: Vec<i64> = values
        .iter()
        .filter_map(|s| detect_and_parse(s))
        .collect();
    assert_eq!(parsed, vec![255, 256, 8, 511]);
}

#[test]
fn integration_parse_csv_data() {
    // Parsing CSV-like data with some invalid entries
    let csv = "100, 200, invalid, 300, -50, , 400";
    let parsed = parse_multiple(csv);
    assert_eq!(parsed, vec![100, 200, 300, -50, 400]);
}

#[test]
fn integration_try_parse_rgb_values() {
    // Parsing RGB color values (0-255)
    let r = try_parse_u8("255");
    let g = try_parse_u8("128");
    let b = try_parse_u8("0");

    assert!(r.is_ok() && g.is_ok() && b.is_ok());
    assert_eq!((r.unwrap(), g.unwrap(), b.unwrap()), (255, 128, 0));

    // Invalid RGB
    assert!(try_parse_u8("256").is_err());
}

#[test]
fn integration_radix_conversion() {
    // Convert between bases using parse and format
    let decimal = 42u32;
    let binary_str = format!("{:b}", decimal);
    let hex_str = format!("{:x}", decimal);
    let octal_str = format!("{:o}", decimal);

    assert_eq!(parse_binary(&binary_str), Some(42));
    assert_eq!(parse_hex(&hex_str), Some(42));
    assert_eq!(parse_octal(&octal_str), Some(42));
}

#[test]
fn integration_error_messages_are_descriptive() {
    let err = try_parse_u8("not_a_number").unwrap_err();
    assert!(err.contains("not_a_number"));
    assert!(err.contains("Failed to parse"));

    let err = try_parse_u8("999").unwrap_err();
    assert!(err.contains("999"));
}
