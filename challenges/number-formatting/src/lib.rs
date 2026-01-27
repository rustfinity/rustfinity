/// Formats an integer with leading zeros to the specified width.
///
/// The number is padded with leading zeros to reach the specified width.
/// If the number is wider than the specified width, it is not truncated.
/// Negative numbers include the minus sign before the zeros.
///
/// # Examples
///
/// ```
/// use number_formatting::format_padded;
///
/// assert_eq!(format_padded(42, 5), "00042");
/// assert_eq!(format_padded(-7, 4), "-007");
/// assert_eq!(format_padded(123, 2), "123");
/// assert_eq!(format_padded(0, 3), "000");
/// ```
pub fn format_padded(num: i32, width: usize) -> String {
    format!("{:0width$}", num, width = width)
}

/// Formats a number with right alignment in a field of the given width.
///
/// The number is right-aligned and padded with spaces on the left.
/// If the number is wider than the specified width, it is not truncated.
///
/// # Examples
///
/// ```
/// use number_formatting::format_aligned;
///
/// assert_eq!(format_aligned(42, 5), "   42");
/// assert_eq!(format_aligned(-7, 5), "   -7");
/// assert_eq!(format_aligned(12345, 3), "12345");
/// ```
pub fn format_aligned(num: i32, width: usize) -> String {
    format!("{:>width$}", num, width = width)
}

/// Formats an unsigned integer as binary without prefix.
///
/// Returns the binary representation as a string without the "0b" prefix.
///
/// # Examples
///
/// ```
/// use number_formatting::format_binary;
///
/// assert_eq!(format_binary(10), "1010");
/// assert_eq!(format_binary(255), "11111111");
/// assert_eq!(format_binary(0), "0");
/// assert_eq!(format_binary(1), "1");
/// ```
pub fn format_binary(num: u32) -> String {
    format!("{:b}", num)
}

/// Formats an unsigned integer as binary with "0b" prefix.
///
/// Returns the binary representation as a string with the "0b" prefix.
///
/// # Examples
///
/// ```
/// use number_formatting::format_binary_prefixed;
///
/// assert_eq!(format_binary_prefixed(10), "0b1010");
/// assert_eq!(format_binary_prefixed(255), "0b11111111");
/// assert_eq!(format_binary_prefixed(0), "0b0");
/// ```
pub fn format_binary_prefixed(num: u32) -> String {
    format!("{:#b}", num)
}

/// Formats an unsigned integer as lowercase hexadecimal without prefix.
///
/// Returns the hexadecimal representation as a string without the "0x" prefix.
/// Letters a-f are lowercase.
///
/// # Examples
///
/// ```
/// use number_formatting::format_hex_lower;
///
/// assert_eq!(format_hex_lower(255), "ff");
/// assert_eq!(format_hex_lower(16), "10");
/// assert_eq!(format_hex_lower(0), "0");
/// assert_eq!(format_hex_lower(171), "ab");
/// ```
pub fn format_hex_lower(num: u32) -> String {
    format!("{:x}", num)
}

/// Formats an unsigned integer as uppercase hexadecimal with "0x" prefix.
///
/// Returns the hexadecimal representation as a string with the "0x" prefix.
/// Letters A-F are uppercase.
///
/// # Examples
///
/// ```
/// use number_formatting::format_hex_upper_prefixed;
///
/// assert_eq!(format_hex_upper_prefixed(255), "0xFF");
/// assert_eq!(format_hex_upper_prefixed(16), "0x10");
/// assert_eq!(format_hex_upper_prefixed(0), "0x0");
/// assert_eq!(format_hex_upper_prefixed(171), "0xAB");
/// ```
pub fn format_hex_upper_prefixed(num: u32) -> String {
    format!("{:#X}", num)
}

/// Formats an unsigned integer as octal without prefix.
///
/// Returns the octal representation as a string without the "0o" prefix.
///
/// # Examples
///
/// ```
/// use number_formatting::format_octal;
///
/// assert_eq!(format_octal(8), "10");
/// assert_eq!(format_octal(63), "77");
/// assert_eq!(format_octal(0), "0");
/// assert_eq!(format_octal(7), "7");
/// ```
pub fn format_octal(num: u32) -> String {
    format!("{:o}", num)
}

/// Formats a floating-point number with the specified number of decimal places.
///
/// Returns a string representation with exactly the specified precision.
/// Rounds the number if necessary.
///
/// # Examples
///
/// ```
/// use number_formatting::format_float_precision;
///
/// assert_eq!(format_float_precision(3.14159, 2), "3.14");
/// assert_eq!(format_float_precision(2.5, 4), "2.5000");
/// assert_eq!(format_float_precision(-1.5, 1), "-1.5");
/// assert_eq!(format_float_precision(0.0, 3), "0.000");
/// ```
pub fn format_float_precision(num: f64, precision: usize) -> String {
    format!("{:.prec$}", num, prec = precision)
}

/// Formats a floating-point number in scientific notation.
///
/// Returns a string representation in lowercase scientific notation (e.g., "1.23e4").
///
/// # Examples
///
/// ```
/// use number_formatting::format_scientific;
///
/// assert_eq!(format_scientific(1234.5), "1.2345e3");
/// assert_eq!(format_scientific(0.00123), "1.23e-3");
/// assert_eq!(format_scientific(1.0), "1e0");
/// assert_eq!(format_scientific(-5000.0), "-5e3");
/// ```
pub fn format_scientific(num: f64) -> String {
    format!("{:e}", num)
}

/// Formats a floating-point number as currency.
///
/// Returns a string in the format "$X.XX" with exactly 2 decimal places.
/// Negative amounts are displayed as "-$X.XX".
///
/// # Examples
///
/// ```
/// use number_formatting::format_currency;
///
/// assert_eq!(format_currency(19.99), "$19.99");
/// assert_eq!(format_currency(-5.5), "-$5.50");
/// assert_eq!(format_currency(1000.0), "$1000.00");
/// assert_eq!(format_currency(0.0), "$0.00");
/// ```
pub fn format_currency(amount: f64) -> String {
    if amount < 0.0 {
        format!("-${:.2}", amount.abs())
    } else {
        // Use abs() to normalize -0.0 to 0.0
        format!("${:.2}", amount.abs())
    }
}
