use number_formatting::*;

// ==================== format_padded tests ====================

#[test]
fn format_padded_basic() {
    assert_eq!(format_padded(42, 5), "00042");
}

#[test]
fn format_padded_exact_width() {
    assert_eq!(format_padded(123, 3), "123");
}

#[test]
fn format_padded_wider_than_width() {
    assert_eq!(format_padded(12345, 3), "12345");
}

#[test]
fn format_padded_zero() {
    assert_eq!(format_padded(0, 3), "000");
}

#[test]
fn format_padded_negative() {
    assert_eq!(format_padded(-7, 4), "-007");
}

#[test]
fn format_padded_negative_wider() {
    assert_eq!(format_padded(-12345, 4), "-12345");
}

#[test]
fn format_padded_single_digit() {
    assert_eq!(format_padded(5, 1), "5");
}

#[test]
fn format_padded_width_zero() {
    assert_eq!(format_padded(42, 0), "42");
}

#[test]
fn format_padded_large_width() {
    assert_eq!(format_padded(1, 10), "0000000001");
}

// ==================== format_aligned tests ====================

#[test]
fn format_aligned_basic() {
    assert_eq!(format_aligned(42, 5), "   42");
}

#[test]
fn format_aligned_exact_width() {
    assert_eq!(format_aligned(123, 3), "123");
}

#[test]
fn format_aligned_wider_than_width() {
    assert_eq!(format_aligned(12345, 3), "12345");
}

#[test]
fn format_aligned_zero() {
    assert_eq!(format_aligned(0, 3), "  0");
}

#[test]
fn format_aligned_negative() {
    assert_eq!(format_aligned(-7, 5), "   -7");
}

#[test]
fn format_aligned_negative_wider() {
    assert_eq!(format_aligned(-12345, 4), "-12345");
}

#[test]
fn format_aligned_single_digit() {
    assert_eq!(format_aligned(5, 5), "    5");
}

#[test]
fn format_aligned_width_zero() {
    assert_eq!(format_aligned(42, 0), "42");
}

#[test]
fn format_aligned_large_width() {
    assert_eq!(format_aligned(1, 10), "         1");
}

// ==================== format_binary tests ====================

#[test]
fn format_binary_basic() {
    assert_eq!(format_binary(10), "1010");
}

#[test]
fn format_binary_zero() {
    assert_eq!(format_binary(0), "0");
}

#[test]
fn format_binary_one() {
    assert_eq!(format_binary(1), "1");
}

#[test]
fn format_binary_255() {
    assert_eq!(format_binary(255), "11111111");
}

#[test]
fn format_binary_power_of_two() {
    assert_eq!(format_binary(16), "10000");
    assert_eq!(format_binary(32), "100000");
    assert_eq!(format_binary(64), "1000000");
}

#[test]
fn format_binary_large() {
    assert_eq!(format_binary(1000), "1111101000");
}

// ==================== format_binary_prefixed tests ====================

#[test]
fn format_binary_prefixed_basic() {
    assert_eq!(format_binary_prefixed(10), "0b1010");
}

#[test]
fn format_binary_prefixed_zero() {
    assert_eq!(format_binary_prefixed(0), "0b0");
}

#[test]
fn format_binary_prefixed_one() {
    assert_eq!(format_binary_prefixed(1), "0b1");
}

#[test]
fn format_binary_prefixed_255() {
    assert_eq!(format_binary_prefixed(255), "0b11111111");
}

#[test]
fn format_binary_prefixed_large() {
    assert_eq!(format_binary_prefixed(1000), "0b1111101000");
}

// ==================== format_hex_lower tests ====================

#[test]
fn format_hex_lower_basic() {
    assert_eq!(format_hex_lower(255), "ff");
}

#[test]
fn format_hex_lower_zero() {
    assert_eq!(format_hex_lower(0), "0");
}

#[test]
fn format_hex_lower_16() {
    assert_eq!(format_hex_lower(16), "10");
}

#[test]
fn format_hex_lower_171() {
    assert_eq!(format_hex_lower(171), "ab");
}

#[test]
fn format_hex_lower_large() {
    assert_eq!(format_hex_lower(65535), "ffff");
}

#[test]
fn format_hex_lower_mixed_digits() {
    assert_eq!(format_hex_lower(3735928559), "deadbeef");
}

// ==================== format_hex_upper_prefixed tests ====================

#[test]
fn format_hex_upper_prefixed_basic() {
    assert_eq!(format_hex_upper_prefixed(255), "0xFF");
}

#[test]
fn format_hex_upper_prefixed_zero() {
    assert_eq!(format_hex_upper_prefixed(0), "0x0");
}

#[test]
fn format_hex_upper_prefixed_16() {
    assert_eq!(format_hex_upper_prefixed(16), "0x10");
}

#[test]
fn format_hex_upper_prefixed_171() {
    assert_eq!(format_hex_upper_prefixed(171), "0xAB");
}

#[test]
fn format_hex_upper_prefixed_large() {
    assert_eq!(format_hex_upper_prefixed(65535), "0xFFFF");
}

#[test]
fn format_hex_upper_prefixed_mixed_digits() {
    assert_eq!(format_hex_upper_prefixed(3735928559), "0xDEADBEEF");
}

// ==================== format_octal tests ====================

#[test]
fn format_octal_basic() {
    assert_eq!(format_octal(8), "10");
}

#[test]
fn format_octal_zero() {
    assert_eq!(format_octal(0), "0");
}

#[test]
fn format_octal_seven() {
    assert_eq!(format_octal(7), "7");
}

#[test]
fn format_octal_63() {
    assert_eq!(format_octal(63), "77");
}

#[test]
fn format_octal_permissions() {
    assert_eq!(format_octal(493), "755");  // Unix permissions
    assert_eq!(format_octal(420), "644");
}

#[test]
fn format_octal_large() {
    assert_eq!(format_octal(511), "777");
}

// ==================== format_float_precision tests ====================

#[test]
fn format_float_precision_two_places() {
    assert_eq!(format_float_precision(3.14159, 2), "3.14");
}

#[test]
fn format_float_precision_zero_places() {
    assert_eq!(format_float_precision(3.7, 0), "4");  // Rounds up
    assert_eq!(format_float_precision(3.2, 0), "3");  // Rounds down
}

#[test]
fn format_float_precision_many_places() {
    assert_eq!(format_float_precision(2.5, 4), "2.5000");
}

#[test]
fn format_float_precision_negative() {
    assert_eq!(format_float_precision(-1.5, 1), "-1.5");
}

#[test]
fn format_float_precision_zero() {
    assert_eq!(format_float_precision(0.0, 3), "0.000");
}

#[test]
fn format_float_precision_rounds_up() {
    assert_eq!(format_float_precision(1.999, 2), "2.00");
}

#[test]
fn format_float_precision_rounds_down() {
    assert_eq!(format_float_precision(1.991, 2), "1.99");
}

#[test]
fn format_float_precision_large() {
    assert_eq!(format_float_precision(123456.789, 2), "123456.79");
}

#[test]
fn format_float_precision_small() {
    assert_eq!(format_float_precision(0.00123, 5), "0.00123");
}

#[test]
fn format_float_precision_one_place() {
    assert_eq!(format_float_precision(9.99, 1), "10.0");
}

// ==================== format_scientific tests ====================

#[test]
fn format_scientific_basic() {
    assert_eq!(format_scientific(1234.5), "1.2345e3");
}

#[test]
fn format_scientific_small() {
    assert_eq!(format_scientific(0.00123), "1.23e-3");
}

#[test]
fn format_scientific_one() {
    assert_eq!(format_scientific(1.0), "1e0");
}

#[test]
fn format_scientific_negative() {
    assert_eq!(format_scientific(-5000.0), "-5e3");
}

#[test]
fn format_scientific_zero() {
    assert_eq!(format_scientific(0.0), "0e0");
}

#[test]
fn format_scientific_large() {
    assert_eq!(format_scientific(1e10), "1e10");
}

#[test]
fn format_scientific_very_small() {
    assert_eq!(format_scientific(1e-10), "1e-10");
}

#[test]
fn format_scientific_with_decimal() {
    assert_eq!(format_scientific(12.34), "1.234e1");
}

// ==================== format_currency tests ====================

#[test]
fn format_currency_basic() {
    assert_eq!(format_currency(19.99), "$19.99");
}

#[test]
fn format_currency_negative() {
    assert_eq!(format_currency(-5.5), "-$5.50");
}

#[test]
fn format_currency_round_number() {
    assert_eq!(format_currency(1000.0), "$1000.00");
}

#[test]
fn format_currency_zero() {
    assert_eq!(format_currency(0.0), "$0.00");
}

#[test]
fn format_currency_small() {
    assert_eq!(format_currency(0.01), "$0.01");
}

#[test]
fn format_currency_one_decimal() {
    assert_eq!(format_currency(5.5), "$5.50");
}

#[test]
fn format_currency_rounds_up() {
    assert_eq!(format_currency(9.999), "$10.00");
}

#[test]
fn format_currency_rounds_down() {
    assert_eq!(format_currency(9.994), "$9.99");
}

#[test]
fn format_currency_large() {
    assert_eq!(format_currency(1000000.00), "$1000000.00");
}

#[test]
fn format_currency_negative_cents() {
    assert_eq!(format_currency(-0.99), "-$0.99");
}

#[test]
fn format_currency_negative_zero() {
    // -0.0 should display as $0.00
    assert_eq!(format_currency(-0.0), "$0.00");
}

// ==================== Integration tests ====================

#[test]
fn integration_format_for_table() {
    // Simulating table column formatting
    let values = vec![1, 23, 456, 7890];
    let formatted: Vec<String> = values.iter().map(|&v| format_aligned(v, 6)).collect();
    assert_eq!(formatted, vec!["     1", "    23", "   456", "  7890"]);
}

#[test]
fn integration_format_hex_and_binary() {
    // Format same number in different bases
    let num = 42u32;
    assert_eq!(format_binary(num), "101010");
    assert_eq!(format_hex_lower(num), "2a");
    assert_eq!(format_octal(num), "52");
}

#[test]
fn integration_format_currency_calculations() {
    // Price calculation with formatting
    let price = 29.99;
    let tax = 0.08;
    let total = price * (1.0 + tax);
    assert_eq!(format_currency(total), "$32.39");
}

#[test]
fn integration_format_padded_ids() {
    // Format IDs with leading zeros
    let ids = vec![1, 10, 100, 1000];
    let formatted: Vec<String> = ids.iter().map(|&id| format_padded(id, 4)).collect();
    assert_eq!(formatted, vec!["0001", "0010", "0100", "1000"]);
}

#[test]
fn integration_format_mixed_precision() {
    // Different precision for different use cases
    let pi = std::f64::consts::PI;
    assert_eq!(format_float_precision(pi, 0), "3");
    assert_eq!(format_float_precision(pi, 2), "3.14");
    assert_eq!(format_float_precision(pi, 5), "3.14159");
}

#[test]
fn integration_format_byte_values() {
    // Common byte values
    assert_eq!(format_hex_lower(0), "0");
    assert_eq!(format_hex_lower(127), "7f");
    assert_eq!(format_hex_lower(255), "ff");
}

#[test]
fn integration_format_permissions() {
    // Unix-style permissions
    assert_eq!(format_octal(493), "755");
    assert_eq!(format_octal(420), "644");
    assert_eq!(format_octal(511), "777");
    assert_eq!(format_octal(256), "400");
}

#[test]
fn integration_format_memory_addresses() {
    // Memory address formatting
    let addr = 0xDEADBEEFu32;
    assert_eq!(format_hex_upper_prefixed(addr), "0xDEADBEEF");
    assert_eq!(format_hex_lower(addr), "deadbeef");
}

#[test]
fn integration_scientific_vs_regular() {
    // Compare scientific and regular for same value
    let num = 1234567.89;
    let regular = format_float_precision(num, 2);
    let scientific = format_scientific(num);
    assert_eq!(regular, "1234567.89");
    assert_eq!(scientific, "1.23456789e6");
}

#[test]
fn integration_binary_powers_of_two() {
    // Powers of 2 in binary should be 1 followed by zeros
    assert_eq!(format_binary(1), "1");
    assert_eq!(format_binary(2), "10");
    assert_eq!(format_binary(4), "100");
    assert_eq!(format_binary(8), "1000");
    assert_eq!(format_binary(16), "10000");
    assert_eq!(format_binary(256), "100000000");
}
