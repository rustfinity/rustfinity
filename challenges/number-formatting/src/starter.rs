/// Formats an integer with leading zeros to the specified width.
///
/// The number is padded with leading zeros to reach the specified width.
/// If the number is wider than the specified width, it is not truncated.
/// Negative numbers include the minus sign before the zeros.
pub fn format_padded(num: i32, width: usize) -> String {
    // TODO
    todo!()
}

/// Formats a number with right alignment in a field of the given width.
///
/// The number is right-aligned and padded with spaces on the left.
/// If the number is wider than the specified width, it is not truncated.
pub fn format_aligned(num: i32, width: usize) -> String {
    // TODO
    todo!()
}

/// Formats an unsigned integer as binary without prefix.
///
/// Returns the binary representation as a string without the "0b" prefix.
pub fn format_binary(num: u32) -> String {
    // TODO
    todo!()
}

/// Formats an unsigned integer as binary with "0b" prefix.
///
/// Returns the binary representation as a string with the "0b" prefix.
pub fn format_binary_prefixed(num: u32) -> String {
    // TODO
    todo!()
}

/// Formats an unsigned integer as lowercase hexadecimal without prefix.
///
/// Returns the hexadecimal representation as a string without the "0x" prefix.
/// Letters a-f are lowercase.
pub fn format_hex_lower(num: u32) -> String {
    // TODO
    todo!()
}

/// Formats an unsigned integer as uppercase hexadecimal with "0x" prefix.
///
/// Returns the hexadecimal representation as a string with the "0x" prefix.
/// Letters A-F are uppercase.
pub fn format_hex_upper_prefixed(num: u32) -> String {
    // TODO
    todo!()
}

/// Formats an unsigned integer as octal without prefix.
///
/// Returns the octal representation as a string without the "0o" prefix.
pub fn format_octal(num: u32) -> String {
    // TODO
    todo!()
}

/// Formats a floating-point number with the specified number of decimal places.
///
/// Returns a string representation with exactly the specified precision.
/// Rounds the number if necessary.
pub fn format_float_precision(num: f64, precision: usize) -> String {
    // TODO
    todo!()
}

/// Formats a floating-point number in scientific notation.
///
/// Returns a string representation in lowercase scientific notation (e.g., "1.23e4").
pub fn format_scientific(num: f64) -> String {
    // TODO
    todo!()
}

/// Formats a floating-point number as currency.
///
/// Returns a string in the format "$X.XX" with exactly 2 decimal places.
/// Negative amounts are displayed as "-$X.XX".
pub fn format_currency(amount: f64) -> String {
    // TODO
    todo!()
}

// Example usage
pub fn main() {
    // Padding with zeros
    println!("format_padded(42, 5) = {}", format_padded(42, 5));

    // Right alignment
    println!("format_aligned(42, 5) = '{}'", format_aligned(42, 5));

    // Binary formatting
    println!("format_binary(10) = {}", format_binary(10));
    println!("format_binary_prefixed(10) = {}", format_binary_prefixed(10));

    // Hexadecimal formatting
    println!("format_hex_lower(255) = {}", format_hex_lower(255));
    println!("format_hex_upper_prefixed(255) = {}", format_hex_upper_prefixed(255));

    // Octal formatting
    println!("format_octal(8) = {}", format_octal(8));

    // Float precision
    println!("format_float_precision(3.14159, 2) = {}", format_float_precision(3.14159, 2));

    // Scientific notation
    println!("format_scientific(1234.5) = {}", format_scientific(1234.5));

    // Currency
    println!("format_currency(19.99) = {}", format_currency(19.99));
    println!("format_currency(-5.5) = {}", format_currency(-5.5));
}
