use number_conversions::*;

// ==================== safe_i32_to_i16 tests ====================

#[test]
fn test_safe_i32_to_i16_zero() {
    assert_eq!(safe_i32_to_i16(0), Some(0));
}

#[test]
fn test_safe_i32_to_i16_positive_in_range() {
    assert_eq!(safe_i32_to_i16(1000), Some(1000));
    assert_eq!(safe_i32_to_i16(32767), Some(32767)); // i16::MAX
}

#[test]
fn test_safe_i32_to_i16_negative_in_range() {
    assert_eq!(safe_i32_to_i16(-1000), Some(-1000));
    assert_eq!(safe_i32_to_i16(-32768), Some(-32768)); // i16::MIN
}

#[test]
fn test_safe_i32_to_i16_positive_overflow() {
    assert_eq!(safe_i32_to_i16(32768), None);
    assert_eq!(safe_i32_to_i16(100_000), None);
    assert_eq!(safe_i32_to_i16(i32::MAX), None);
}

#[test]
fn test_safe_i32_to_i16_negative_overflow() {
    assert_eq!(safe_i32_to_i16(-32769), None);
    assert_eq!(safe_i32_to_i16(-100_000), None);
    assert_eq!(safe_i32_to_i16(i32::MIN), None);
}

#[test]
fn test_safe_i32_to_i16_boundary() {
    assert_eq!(safe_i32_to_i16(i16::MAX as i32), Some(i16::MAX));
    assert_eq!(safe_i32_to_i16(i16::MIN as i32), Some(i16::MIN));
    assert_eq!(safe_i32_to_i16(i16::MAX as i32 + 1), None);
    assert_eq!(safe_i32_to_i16(i16::MIN as i32 - 1), None);
}

// ==================== safe_u64_to_u32 tests ====================

#[test]
fn test_safe_u64_to_u32_zero() {
    assert_eq!(safe_u64_to_u32(0), Some(0));
}

#[test]
fn test_safe_u64_to_u32_in_range() {
    assert_eq!(safe_u64_to_u32(1000), Some(1000));
    assert_eq!(safe_u64_to_u32(1_000_000), Some(1_000_000));
}

#[test]
fn test_safe_u64_to_u32_at_boundary() {
    assert_eq!(safe_u64_to_u32(u32::MAX as u64), Some(u32::MAX));
    assert_eq!(safe_u64_to_u32(u32::MAX as u64 + 1), None);
}

#[test]
fn test_safe_u64_to_u32_overflow() {
    assert_eq!(safe_u64_to_u32(5_000_000_000), None);
    assert_eq!(safe_u64_to_u32(u64::MAX), None);
}

// ==================== safe_i64_to_usize tests ====================

#[test]
fn test_safe_i64_to_usize_zero() {
    assert_eq!(safe_i64_to_usize(0), Some(0));
}

#[test]
fn test_safe_i64_to_usize_positive() {
    assert_eq!(safe_i64_to_usize(1000), Some(1000));
    assert_eq!(safe_i64_to_usize(1_000_000), Some(1_000_000));
}

#[test]
fn test_safe_i64_to_usize_negative() {
    assert_eq!(safe_i64_to_usize(-1), None);
    assert_eq!(safe_i64_to_usize(-1000), None);
    assert_eq!(safe_i64_to_usize(i64::MIN), None);
}

#[test]
fn test_safe_i64_to_usize_large_positive() {
    // On 64-bit systems, this should work
    // On 32-bit systems, large values would fail
    let value = 1_000_000_000i64;
    assert_eq!(safe_i64_to_usize(value), Some(1_000_000_000));
}

// ==================== checked_multiply tests ====================

#[test]
fn test_checked_multiply_basic() {
    assert_eq!(checked_multiply(5, 6), Some(30));
    assert_eq!(checked_multiply(100, 200), Some(20_000));
}

#[test]
fn test_checked_multiply_zero() {
    assert_eq!(checked_multiply(0, 1000), Some(0));
    assert_eq!(checked_multiply(1000, 0), Some(0));
    assert_eq!(checked_multiply(0, 0), Some(0));
}

#[test]
fn test_checked_multiply_negative() {
    assert_eq!(checked_multiply(-5, 6), Some(-30));
    assert_eq!(checked_multiply(5, -6), Some(-30));
    assert_eq!(checked_multiply(-5, -6), Some(30));
}

#[test]
fn test_checked_multiply_overflow_positive() {
    assert_eq!(checked_multiply(i32::MAX, 2), None);
    assert_eq!(checked_multiply(100_000, 100_000), None);
    assert_eq!(checked_multiply(50_000, 50_000), None);
}

#[test]
fn test_checked_multiply_overflow_negative() {
    assert_eq!(checked_multiply(i32::MIN, 2), None);
    assert_eq!(checked_multiply(i32::MIN, -1), None);
    assert_eq!(checked_multiply(-100_000, 100_000), None);
}

#[test]
fn test_checked_multiply_boundary() {
    assert_eq!(checked_multiply(46340, 46340), Some(2_147_395_600)); // Just under i32::MAX
    assert_eq!(checked_multiply(46341, 46341), None); // Just over
}

// ==================== checked_power tests ====================

#[test]
fn test_checked_power_zero_exponent() {
    assert_eq!(checked_power(5, 0), Some(1));
    assert_eq!(checked_power(0, 0), Some(1)); // Mathematical convention
    assert_eq!(checked_power(100, 0), Some(1));
}

#[test]
fn test_checked_power_one_exponent() {
    assert_eq!(checked_power(5, 1), Some(5));
    assert_eq!(checked_power(100, 1), Some(100));
}

#[test]
fn test_checked_power_basic() {
    assert_eq!(checked_power(2, 10), Some(1024));
    assert_eq!(checked_power(3, 5), Some(243));
    assert_eq!(checked_power(10, 5), Some(100_000));
}

#[test]
fn test_checked_power_large_valid() {
    assert_eq!(checked_power(10, 9), Some(1_000_000_000));
    assert_eq!(checked_power(2, 31), Some(2_147_483_648)); // 2^31, fits in u32
}

#[test]
fn test_checked_power_overflow() {
    assert_eq!(checked_power(10, 10), None);
    assert_eq!(checked_power(2, 32), None);
    assert_eq!(checked_power(100, 5), None);
}

#[test]
fn test_checked_power_zero_base() {
    assert_eq!(checked_power(0, 1), Some(0));
    assert_eq!(checked_power(0, 10), Some(0));
}

// ==================== saturating_sum tests ====================

#[test]
fn test_saturating_sum_empty() {
    assert_eq!(saturating_sum(&[]), 0);
}

#[test]
fn test_saturating_sum_single() {
    assert_eq!(saturating_sum(&[42]), 42);
    assert_eq!(saturating_sum(&[-42]), -42);
}

#[test]
fn test_saturating_sum_basic() {
    assert_eq!(saturating_sum(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(saturating_sum(&[-1, -2, -3, -4, -5]), -15);
    assert_eq!(saturating_sum(&[10, -5, 3, -8, 20]), 20);
}

#[test]
fn test_saturating_sum_positive_overflow() {
    assert_eq!(saturating_sum(&[i32::MAX, 1]), i32::MAX);
    assert_eq!(saturating_sum(&[i32::MAX, i32::MAX]), i32::MAX);
    assert_eq!(saturating_sum(&[i32::MAX - 10, 20]), i32::MAX);
}

#[test]
fn test_saturating_sum_negative_overflow() {
    assert_eq!(saturating_sum(&[i32::MIN, -1]), i32::MIN);
    assert_eq!(saturating_sum(&[i32::MIN, i32::MIN]), i32::MIN);
    assert_eq!(saturating_sum(&[i32::MIN + 10, -20]), i32::MIN);
}

#[test]
fn test_saturating_sum_mixed() {
    // Positive overflow then negative brings it back down
    assert_eq!(saturating_sum(&[i32::MAX, -1000]), i32::MAX - 1000);
    // Starts small, never overflows
    assert_eq!(saturating_sum(&[1000, -500, 2000, -1500]), 1000);
}

// ==================== wrapping_factorial tests ====================

#[test]
fn test_wrapping_factorial_zero() {
    assert_eq!(wrapping_factorial(0), 1);
}

#[test]
fn test_wrapping_factorial_one() {
    assert_eq!(wrapping_factorial(1), 1);
}

#[test]
fn test_wrapping_factorial_small() {
    assert_eq!(wrapping_factorial(2), 2);
    assert_eq!(wrapping_factorial(3), 6);
    assert_eq!(wrapping_factorial(4), 24);
    assert_eq!(wrapping_factorial(5), 120);
}

#[test]
fn test_wrapping_factorial_medium() {
    assert_eq!(wrapping_factorial(10), 3_628_800);
    assert_eq!(wrapping_factorial(12), 479_001_600);
}

#[test]
fn test_wrapping_factorial_overflow() {
    // 13! = 6,227,020,800 which overflows u32 (max: 4,294,967,295)
    // 6,227,020,800 mod 2^32 = 1,932,053,504
    assert_eq!(wrapping_factorial(13), 1_932_053_504);
    // Larger factorials continue wrapping
    assert_eq!(wrapping_factorial(20), 2_192_834_560);
}

// ==================== safe_average tests ====================

#[test]
fn test_safe_average_empty() {
    assert_eq!(safe_average(&[]), None);
}

#[test]
fn test_safe_average_single() {
    assert_eq!(safe_average(&[42]), Some(42));
    assert_eq!(safe_average(&[-100]), Some(-100));
}

#[test]
fn test_safe_average_basic() {
    assert_eq!(safe_average(&[10, 20, 30]), Some(20));
    assert_eq!(safe_average(&[1, 2, 3, 4, 5]), Some(3));
    assert_eq!(safe_average(&[-10, -20, -30]), Some(-20));
}

#[test]
fn test_safe_average_mixed() {
    assert_eq!(safe_average(&[-10, 10]), Some(0));
    assert_eq!(safe_average(&[-100, 50, 50]), Some(0));
}

#[test]
fn test_safe_average_truncation() {
    // 10 / 3 = 3 (integer division truncates)
    assert_eq!(safe_average(&[3, 3, 4]), Some(3));
    // (7 + 8) / 2 = 7 (truncated)
    assert_eq!(safe_average(&[7, 8]), Some(7));
}

#[test]
fn test_safe_average_overflow() {
    assert_eq!(safe_average(&[i64::MAX, i64::MAX]), None);
    assert_eq!(safe_average(&[i64::MAX, 1]), None);
    assert_eq!(safe_average(&[i64::MIN, i64::MIN]), None);
}

#[test]
fn test_safe_average_large_valid() {
    // Large but no overflow
    let half_max = i64::MAX / 2;
    assert_eq!(safe_average(&[half_max, half_max]), Some(half_max));
}

// ==================== Integration tests ====================

#[test]
fn test_conversion_chain() {
    // Start with i64, convert through chain
    let original: i64 = 1000;
    let as_usize = safe_i64_to_usize(original).unwrap();
    assert_eq!(as_usize, 1000usize);

    let u64_value = as_usize as u64;
    let as_u32 = safe_u64_to_u32(u64_value).unwrap();
    assert_eq!(as_u32, 1000u32);
}

#[test]
fn test_checked_vs_saturating() {
    // Same operation, different handling
    let nums = &[i32::MAX, 100];

    // Checked: fails on overflow
    let checked_result = nums[0].checked_add(nums[1]);
    assert_eq!(checked_result, None);

    // Saturating: clamps to max
    let sat_result = saturating_sum(nums);
    assert_eq!(sat_result, i32::MAX);
}

#[test]
fn test_power_and_multiply_relationship() {
    // 2^10 should equal 2*2*2*2*2*2*2*2*2*2
    let power_result = checked_power(2, 10);
    let mut mul_result = Some(1i32);
    for _ in 0..10 {
        mul_result = mul_result.and_then(|x| checked_multiply(x, 2));
    }
    assert_eq!(power_result, Some(1024));
    assert_eq!(mul_result, Some(1024));
}

#[test]
fn test_factorial_and_average() {
    // Calculate average of first few factorials
    let factorials: Vec<i64> = (0..=5).map(|n| wrapping_factorial(n) as i64).collect();
    // [1, 1, 2, 6, 24, 120] sum = 154, avg = 25
    assert_eq!(safe_average(&factorials), Some(25));
}

#[test]
fn test_all_boundary_values() {
    // Test with various type boundaries
    assert_eq!(safe_i32_to_i16(i16::MAX as i32), Some(i16::MAX));
    assert_eq!(safe_i32_to_i16(i16::MIN as i32), Some(i16::MIN));
    assert_eq!(safe_u64_to_u32(u32::MAX as u64), Some(u32::MAX));
    assert_eq!(safe_i64_to_usize(0), Some(0));
    assert_eq!(checked_multiply(1, i32::MAX), Some(i32::MAX));
    assert_eq!(checked_power(1, u32::MAX), Some(1));
}

#[test]
fn test_practical_use_case() {
    // Simulate reading sensor data with potential overflow
    let sensor_readings: Vec<i64> = vec![100, 200, 150, 175, 225];

    // Calculate average safely
    let avg = safe_average(&sensor_readings);
    assert_eq!(avg, Some(170));

    // Convert to smaller type for storage
    let avg_i32: Option<i32> = avg.and_then(|v| i32::try_from(v).ok());
    assert_eq!(avg_i32, Some(170));
}
