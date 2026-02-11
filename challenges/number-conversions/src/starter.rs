/// Safely converts an i32 to an i16.
///
/// Returns `None` if the value is outside the range of i16 (-32768 to 32767).
pub fn safe_i32_to_i16(value: i32) -> Option<i16> {
    // TODO
    unimplemented!()
}

/// Safely converts a u64 to a u32.
///
/// Returns `None` if the value is larger than u32::MAX.
pub fn safe_u64_to_u32(value: u64) -> Option<u32> {
    // TODO
    unimplemented!()
}

/// Safely converts an i64 to a usize.
///
/// Returns `None` if the value is negative or too large for usize.
pub fn safe_i64_to_usize(value: i64) -> Option<usize> {
    // TODO
    unimplemented!()
}

/// Multiplies two i32 values using checked arithmetic.
///
/// Returns `None` if the multiplication would overflow i32.
pub fn checked_multiply(a: i32, b: i32) -> Option<i32> {
    // TODO
    unimplemented!()
}

/// Calculates base^exp using checked arithmetic.
///
/// Returns `None` if the result would overflow u32.
pub fn checked_power(base: u32, exp: u32) -> Option<u32> {
    // TODO
    unimplemented!()
}

/// Sums all numbers using saturating arithmetic.
///
/// Instead of overflowing, the result clamps at i32::MIN or i32::MAX.
/// Returns 0 for an empty slice.
pub fn saturating_sum(numbers: &[i32]) -> i32 {
    // TODO
    unimplemented!()
}

/// Calculates n! (factorial) using wrapping arithmetic.
///
/// Wrapping arithmetic allows overflow to wrap around.
/// This is useful when you want deterministic behavior despite overflow.
pub fn wrapping_factorial(n: u32) -> u32 {
    // TODO
    unimplemented!()
}

/// Calculates the average of numbers using checked arithmetic.
///
/// Returns `None` if the slice is empty or if the sum would overflow.
pub fn safe_average(numbers: &[i64]) -> Option<i64> {
    // TODO
    unimplemented!()
}

// Example usage
pub fn main() {
    // Safe type conversions
    println!("Converting 1000 from i32 to i16: {:?}", safe_i32_to_i16(1000));
    println!("Converting 100000 from i32 to i16: {:?}", safe_i32_to_i16(100_000));

    // Checked arithmetic
    println!("100 * 200 = {:?}", checked_multiply(100, 200));
    println!("2^10 = {:?}", checked_power(2, 10));

    // Saturating sum
    let nums = vec![i32::MAX, 100];
    println!("Saturating sum of MAX + 100: {}", saturating_sum(&nums));

    // Wrapping factorial
    println!("5! = {}", wrapping_factorial(5));
    println!("13! (wrapped) = {}", wrapping_factorial(13));

    // Safe average
    println!("Average of [10, 20, 30]: {:?}", safe_average(&[10, 20, 30]));
}
