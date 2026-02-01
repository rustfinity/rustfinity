use std::convert::TryFrom;

/// Safely converts an i32 to an i16.
///
/// Returns `None` if the value is outside the range of i16.
///
/// # Examples
///
/// ```
/// use number_conversions::safe_i32_to_i16;
///
/// assert_eq!(safe_i32_to_i16(1000), Some(1000));
/// assert_eq!(safe_i32_to_i16(-1000), Some(-1000));
/// assert_eq!(safe_i32_to_i16(100_000), None);
/// assert_eq!(safe_i32_to_i16(-100_000), None);
/// ```
pub fn safe_i32_to_i16(value: i32) -> Option<i16> {
    i16::try_from(value).ok()
}

/// Safely converts a u64 to a u32.
///
/// Returns `None` if the value is outside the range of u32.
///
/// # Examples
///
/// ```
/// use number_conversions::safe_u64_to_u32;
///
/// assert_eq!(safe_u64_to_u32(1000), Some(1000));
/// assert_eq!(safe_u64_to_u32(u32::MAX as u64), Some(u32::MAX));
/// assert_eq!(safe_u64_to_u32(u64::MAX), None);
/// ```
pub fn safe_u64_to_u32(value: u64) -> Option<u32> {
    u32::try_from(value).ok()
}

/// Safely converts an i64 to a usize.
///
/// Returns `None` if the value is negative or too large for usize.
///
/// # Examples
///
/// ```
/// use number_conversions::safe_i64_to_usize;
///
/// assert_eq!(safe_i64_to_usize(1000), Some(1000));
/// assert_eq!(safe_i64_to_usize(0), Some(0));
/// assert_eq!(safe_i64_to_usize(-1), None);
/// ```
pub fn safe_i64_to_usize(value: i64) -> Option<usize> {
    usize::try_from(value).ok()
}

/// Multiplies two i32 values using checked arithmetic.
///
/// Returns `None` if the multiplication would overflow.
///
/// # Examples
///
/// ```
/// use number_conversions::checked_multiply;
///
/// assert_eq!(checked_multiply(100, 200), Some(20_000));
/// assert_eq!(checked_multiply(i32::MAX, 2), None);
/// assert_eq!(checked_multiply(-1000, 1000), Some(-1_000_000));
/// ```
pub fn checked_multiply(a: i32, b: i32) -> Option<i32> {
    a.checked_mul(b)
}

/// Calculates base^exp using checked arithmetic.
///
/// Returns `None` if the result would overflow.
///
/// # Examples
///
/// ```
/// use number_conversions::checked_power;
///
/// assert_eq!(checked_power(2, 10), Some(1024));
/// assert_eq!(checked_power(10, 9), Some(1_000_000_000));
/// assert_eq!(checked_power(10, 10), None);  // Would overflow u32
/// assert_eq!(checked_power(2, 0), Some(1));
/// ```
pub fn checked_power(base: u32, exp: u32) -> Option<u32> {
    base.checked_pow(exp)
}

/// Sums all numbers using saturating arithmetic.
///
/// The result is clamped at i32::MIN or i32::MAX instead of overflowing.
///
/// # Examples
///
/// ```
/// use number_conversions::saturating_sum;
///
/// assert_eq!(saturating_sum(&[1, 2, 3, 4, 5]), 15);
/// assert_eq!(saturating_sum(&[i32::MAX, 1]), i32::MAX);
/// assert_eq!(saturating_sum(&[i32::MIN, -1]), i32::MIN);
/// assert_eq!(saturating_sum(&[]), 0);
/// ```
pub fn saturating_sum(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0i32, |acc, &x| acc.saturating_add(x))
}

/// Calculates n! (factorial) using wrapping arithmetic.
///
/// Wrapping arithmetic allows overflow to wrap around, which is useful
/// when you want to continue computation despite overflow (like in
/// hash functions or certain algorithms).
///
/// # Examples
///
/// ```
/// use number_conversions::wrapping_factorial;
///
/// assert_eq!(wrapping_factorial(0), 1);
/// assert_eq!(wrapping_factorial(5), 120);
/// assert_eq!(wrapping_factorial(10), 3_628_800);
/// // 13! overflows u32, but wrapping continues
/// assert_eq!(wrapping_factorial(13), 1_932_053_504);
/// ```
pub fn wrapping_factorial(n: u32) -> u32 {
    (1..=n).fold(1u32, |acc, x| acc.wrapping_mul(x))
}

/// Calculates the average of numbers using checked arithmetic.
///
/// Returns `None` if the slice is empty or if the sum would overflow.
///
/// # Examples
///
/// ```
/// use number_conversions::safe_average;
///
/// assert_eq!(safe_average(&[10, 20, 30]), Some(20));
/// assert_eq!(safe_average(&[i64::MAX, i64::MAX]), None);  // Sum overflows
/// assert_eq!(safe_average(&[]), None);  // Empty slice
/// ```
pub fn safe_average(numbers: &[i64]) -> Option<i64> {
    if numbers.is_empty() {
        return None;
    }

    let mut sum: i64 = 0;
    for &n in numbers {
        sum = sum.checked_add(n)?;
    }

    Some(sum / numbers.len() as i64)
}
