/// Checks if a number is a valid, finite number (not NaN or infinite).
///
/// Returns `true` if `x` is a number you can safely do math with.
pub fn is_valid_number(x: f64) -> bool {
    // TODO: Check if x is finite (not NaN and not infinite)
    // Hint: Look at the is_finite() method on f64
    unimplemented!()
}

/// Classifies a floating-point number into categories.
///
/// Returns one of: `"nan"`, `"infinite"`, `"zero"`, `"normal"`, or `"subnormal"`.
pub fn classify_float(x: f64) -> &'static str {
    // TODO: Use the classify() method and match on FpCategory
    // You'll need: use std::num::FpCategory;
    unimplemented!()
}

/// Safely divides two numbers, handling edge cases.
///
/// Returns `None` if:
/// - `b` is zero
/// - Either `a` or `b` is NaN
/// - Either `a` or `b` is infinite
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // TODO: Check for invalid inputs before dividing
    // Consider: What makes division unsafe?
    unimplemented!()
}

/// Rounds a number to the specified number of decimal places.
pub fn round_to_places(x: f64, places: u32) -> f64 {
    // TODO: Multiply, round, then divide to achieve decimal place rounding
    // Hint: 10_f64.powi(places as i32) gives you the multiplier
    unimplemented!()
}

/// Checks if two numbers are approximately equal within a tolerance.
///
/// Returns `false` if either number is NaN (since NaN comparisons are always false).
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    // TODO: Handle NaN case first, then check if |a - b| <= epsilon
    unimplemented!()
}

/// Clamps a number to a range, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - Any of the inputs is NaN
/// - `min > max`
pub fn clamp_to_range(x: f64, min: f64, max: f64) -> Option<f64> {
    // TODO: Validate inputs, then clamp x to [min, max]
    // Note: Don't use the built-in clamp() as it doesn't handle NaN correctly
    unimplemented!()
}

/// Safely computes the square root, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - `x` is negative
/// - `x` is NaN
pub fn safe_sqrt(x: f64) -> Option<f64> {
    // TODO: Check for invalid inputs before computing sqrt
    unimplemented!()
}

/// Sums all finite numbers in a slice, skipping NaN and infinite values.
///
/// Returns 0.0 for an empty slice.
pub fn sum_finite(numbers: &[f64]) -> f64 {
    // TODO: Filter out non-finite values, then sum the rest
    // Hint: Use filter() with is_finite()
    unimplemented!()
}

// Example usage
pub fn main() {
    // Test is_valid_number
    println!("42.0 is valid: {}", is_valid_number(42.0));
    println!("NaN is valid: {}", is_valid_number(f64::NAN));

    // Test classify_float
    println!("Classify 1.0: {}", classify_float(1.0));
    println!("Classify NaN: {}", classify_float(f64::NAN));

    // Test safe_divide
    println!("10 / 2 = {:?}", safe_divide(10.0, 2.0));
    println!("10 / 0 = {:?}", safe_divide(10.0, 0.0));

    // Test round_to_places
    println!("3.14159 rounded to 2 places: {}", round_to_places(3.14159, 2));

    // Test approx_equal
    println!("0.1 + 0.2 ≈ 0.3: {}", approx_equal(0.1 + 0.2, 0.3, 1e-10));

    // Test clamp_to_range
    println!("15.0 clamped to [0, 10]: {:?}", clamp_to_range(15.0, 0.0, 10.0));

    // Test safe_sqrt
    println!("sqrt(4) = {:?}", safe_sqrt(4.0));
    println!("sqrt(-1) = {:?}", safe_sqrt(-1.0));

    // Test sum_finite
    let numbers = vec![1.0, f64::NAN, 2.0, f64::INFINITY, 3.0];
    println!("sum_finite({:?}) = {}", numbers, sum_finite(&numbers));
}
