use std::num::FpCategory;

/// Checks if a number is a valid, finite number (not NaN or infinite).
///
/// Returns `true` if `x` is a number you can safely do math with.
///
/// # Examples
///
/// ```
/// use floating_point::is_valid_number;
///
/// assert!(is_valid_number(42.0));
/// assert!(is_valid_number(-3.14));
/// assert!(is_valid_number(0.0));
/// assert!(!is_valid_number(f64::NAN));
/// assert!(!is_valid_number(f64::INFINITY));
/// assert!(!is_valid_number(f64::NEG_INFINITY));
/// ```
pub fn is_valid_number(x: f64) -> bool {
    x.is_finite()
}

/// Classifies a floating-point number into categories.
///
/// Returns one of: `"nan"`, `"infinite"`, `"zero"`, `"normal"`, or `"subnormal"`.
///
/// # Examples
///
/// ```
/// use floating_point::classify_float;
///
/// assert_eq!(classify_float(f64::NAN), "nan");
/// assert_eq!(classify_float(f64::INFINITY), "infinite");
/// assert_eq!(classify_float(0.0), "zero");
/// assert_eq!(classify_float(1.5), "normal");
/// ```
pub fn classify_float(x: f64) -> &'static str {
    match x.classify() {
        FpCategory::Nan => "nan",
        FpCategory::Infinite => "infinite",
        FpCategory::Zero => "zero",
        FpCategory::Subnormal => "subnormal",
        FpCategory::Normal => "normal",
    }
}

/// Safely divides two numbers, handling edge cases.
///
/// Returns `None` if:
/// - `b` is zero
/// - Either `a` or `b` is NaN
/// - Either `a` or `b` is infinite
///
/// # Examples
///
/// ```
/// use floating_point::safe_divide;
///
/// assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
/// assert_eq!(safe_divide(10.0, 0.0), None);
/// assert_eq!(safe_divide(f64::NAN, 2.0), None);
/// assert_eq!(safe_divide(1.0, f64::INFINITY), None);
/// ```
pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if !a.is_finite() || !b.is_finite() || b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Rounds a number to the specified number of decimal places.
///
/// # Examples
///
/// ```
/// use floating_point::round_to_places;
///
/// assert_eq!(round_to_places(3.14159, 2), 3.14);
/// assert_eq!(round_to_places(3.14159, 4), 3.1416);
/// assert_eq!(round_to_places(2.5, 0), 3.0);
/// assert_eq!(round_to_places(-2.5, 0), -3.0);
/// ```
pub fn round_to_places(x: f64, places: u32) -> f64 {
    let multiplier = 10_f64.powi(places as i32);
    (x * multiplier).round() / multiplier
}

/// Checks if two numbers are approximately equal within a tolerance.
///
/// Returns `false` if either number is NaN (since NaN comparisons are always false).
///
/// # Examples
///
/// ```
/// use floating_point::approx_equal;
///
/// assert!(approx_equal(0.1 + 0.2, 0.3, 1e-10));
/// assert!(approx_equal(1.0, 1.0001, 0.001));
/// assert!(!approx_equal(1.0, 2.0, 0.5));
/// assert!(!approx_equal(f64::NAN, f64::NAN, 1.0));
/// ```
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    (a - b).abs() <= epsilon
}

/// Clamps a number to a range, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - Any of the inputs is NaN
/// - `min > max`
///
/// # Examples
///
/// ```
/// use floating_point::clamp_to_range;
///
/// assert_eq!(clamp_to_range(5.0, 0.0, 10.0), Some(5.0));
/// assert_eq!(clamp_to_range(-5.0, 0.0, 10.0), Some(0.0));
/// assert_eq!(clamp_to_range(15.0, 0.0, 10.0), Some(10.0));
/// assert_eq!(clamp_to_range(f64::NAN, 0.0, 10.0), None);
/// assert_eq!(clamp_to_range(5.0, 10.0, 0.0), None);  // min > max
/// ```
pub fn clamp_to_range(x: f64, min: f64, max: f64) -> Option<f64> {
    if x.is_nan() || min.is_nan() || max.is_nan() || min > max {
        None
    } else if x < min {
        Some(min)
    } else if x > max {
        Some(max)
    } else {
        Some(x)
    }
}

/// Safely computes the square root, returning `None` for invalid inputs.
///
/// Returns `None` if:
/// - `x` is negative
/// - `x` is NaN
///
/// # Examples
///
/// ```
/// use floating_point::safe_sqrt;
///
/// assert_eq!(safe_sqrt(4.0), Some(2.0));
/// assert_eq!(safe_sqrt(0.0), Some(0.0));
/// assert_eq!(safe_sqrt(-1.0), None);
/// assert_eq!(safe_sqrt(f64::NAN), None);
/// ```
pub fn safe_sqrt(x: f64) -> Option<f64> {
    if x.is_nan() || x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

/// Sums all finite numbers in a slice, skipping NaN and infinite values.
///
/// Returns 0.0 for an empty slice.
///
/// # Examples
///
/// ```
/// use floating_point::sum_finite;
///
/// assert_eq!(sum_finite(&[1.0, 2.0, 3.0]), 6.0);
/// assert_eq!(sum_finite(&[1.0, f64::NAN, 2.0, f64::INFINITY, 3.0]), 6.0);
/// assert_eq!(sum_finite(&[f64::NAN, f64::INFINITY]), 0.0);
/// assert_eq!(sum_finite(&[]), 0.0);
/// ```
pub fn sum_finite(numbers: &[f64]) -> f64 {
    numbers.iter().filter(|x| x.is_finite()).sum()
}
