use std::time::{Duration, SystemTime, UNIX_EPOCH};
use systemtime_usage::*;

// ============================================================================
// Tests for current_unix_timestamp
// ============================================================================

#[test]
fn current_unix_timestamp_is_reasonable() {
    let timestamp = current_unix_timestamp();
    // Should be after Jan 1, 2020
    assert!(timestamp > 1577836800, "Timestamp should be after year 2020");
    // Should be before year 2100
    assert!(timestamp < 4102444800, "Timestamp should be before year 2100");
}

#[test]
fn current_unix_timestamp_is_consistent() {
    let t1 = current_unix_timestamp();
    let t2 = current_unix_timestamp();
    // Second call should be >= first call
    assert!(t2 >= t1, "Timestamps should be monotonically increasing");
}

// ============================================================================
// Tests for from_unix_timestamp
// ============================================================================

#[test]
fn from_unix_timestamp_zero() {
    let time = from_unix_timestamp(0);
    assert_eq!(time, UNIX_EPOCH);
}

#[test]
fn from_unix_timestamp_one() {
    let time = from_unix_timestamp(1);
    assert_eq!(time, UNIX_EPOCH + Duration::from_secs(1));
}

#[test]
fn from_unix_timestamp_typical() {
    let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
    let time = from_unix_timestamp(timestamp);
    assert_eq!(time, UNIX_EPOCH + Duration::from_secs(timestamp));
}

#[test]
fn from_unix_timestamp_large() {
    let timestamp = 4102444800; // Year 2100
    let time = from_unix_timestamp(timestamp);
    assert_eq!(time, UNIX_EPOCH + Duration::from_secs(timestamp));
}

// ============================================================================
// Tests for to_unix_timestamp
// ============================================================================

#[test]
fn to_unix_timestamp_epoch() {
    assert_eq!(to_unix_timestamp(UNIX_EPOCH), Some(0));
}

#[test]
fn to_unix_timestamp_one_second() {
    let time = UNIX_EPOCH + Duration::from_secs(1);
    assert_eq!(to_unix_timestamp(time), Some(1));
}

#[test]
fn to_unix_timestamp_typical() {
    let timestamp = 1704067200u64;
    let time = UNIX_EPOCH + Duration::from_secs(timestamp);
    assert_eq!(to_unix_timestamp(time), Some(timestamp));
}

#[test]
fn to_unix_timestamp_round_trip() {
    let original = 1234567890u64;
    let time = from_unix_timestamp(original);
    assert_eq!(to_unix_timestamp(time), Some(original));
}

#[test]
fn to_unix_timestamp_truncates_subseconds() {
    let time = UNIX_EPOCH + Duration::from_millis(1500); // 1.5 seconds
    assert_eq!(to_unix_timestamp(time), Some(1));
}

// ============================================================================
// Tests for seconds_between
// ============================================================================

#[test]
fn seconds_between_same_time() {
    let t = from_unix_timestamp(1000);
    assert_eq!(seconds_between(t, t), Some(0));
}

#[test]
fn seconds_between_one_second() {
    let t1 = from_unix_timestamp(1000);
    let t2 = from_unix_timestamp(1001);
    assert_eq!(seconds_between(t1, t2), Some(1));
}

#[test]
fn seconds_between_many_seconds() {
    let t1 = from_unix_timestamp(1000);
    let t2 = from_unix_timestamp(2500);
    assert_eq!(seconds_between(t1, t2), Some(1500));
}

#[test]
fn seconds_between_wrong_order() {
    let t1 = from_unix_timestamp(1000);
    let t2 = from_unix_timestamp(2500);
    // t2 is after t1, so this should return None
    assert_eq!(seconds_between(t2, t1), None);
}

#[test]
fn seconds_between_from_epoch() {
    let t1 = UNIX_EPOCH;
    let t2 = from_unix_timestamp(3600);
    assert_eq!(seconds_between(t1, t2), Some(3600));
}

#[test]
fn seconds_between_large_gap() {
    let t1 = from_unix_timestamp(0);
    let t2 = from_unix_timestamp(1000000000);
    assert_eq!(seconds_between(t1, t2), Some(1000000000));
}

#[test]
fn seconds_between_truncates_subseconds() {
    let t1 = UNIX_EPOCH + Duration::from_millis(500);
    let t2 = UNIX_EPOCH + Duration::from_millis(2000);
    // Difference is 1.5 seconds, should truncate to 1
    assert_eq!(seconds_between(t1, t2), Some(1));
}

// ============================================================================
// Tests for is_in_past
// ============================================================================

#[test]
fn is_in_past_epoch() {
    assert!(is_in_past(UNIX_EPOCH));
}

#[test]
fn is_in_past_old_date() {
    let old = from_unix_timestamp(1000);
    assert!(is_in_past(old));
}

#[test]
fn is_in_past_year_2020() {
    let y2020 = from_unix_timestamp(1577836800);
    assert!(is_in_past(y2020));
}

#[test]
fn is_in_past_far_future() {
    // Year 3000 - definitely in the future
    let far_future = from_unix_timestamp(32503680000);
    assert!(!is_in_past(far_future));
}

// ============================================================================
// Tests for is_in_future
// ============================================================================

#[test]
fn is_in_future_epoch() {
    assert!(!is_in_future(UNIX_EPOCH));
}

#[test]
fn is_in_future_old_date() {
    let old = from_unix_timestamp(1000);
    assert!(!is_in_future(old));
}

#[test]
fn is_in_future_far_future() {
    // Year 3000 - definitely in the future
    let far_future = from_unix_timestamp(32503680000);
    assert!(is_in_future(far_future));
}

#[test]
fn is_in_future_year_2200() {
    let y2200 = from_unix_timestamp(7258118400);
    assert!(is_in_future(y2200));
}

// ============================================================================
// Tests for add_seconds
// ============================================================================

#[test]
fn add_seconds_zero() {
    let t = from_unix_timestamp(1000);
    let result = add_seconds(t, 0);
    assert_eq!(to_unix_timestamp(result), Some(1000));
}

#[test]
fn add_seconds_one() {
    let t = from_unix_timestamp(1000);
    let result = add_seconds(t, 1);
    assert_eq!(to_unix_timestamp(result), Some(1001));
}

#[test]
fn add_seconds_many() {
    let t = from_unix_timestamp(1000);
    let result = add_seconds(t, 500);
    assert_eq!(to_unix_timestamp(result), Some(1500));
}

#[test]
fn add_seconds_to_epoch() {
    let result = add_seconds(UNIX_EPOCH, 3600);
    assert_eq!(to_unix_timestamp(result), Some(3600));
}

#[test]
fn add_seconds_large_value() {
    let t = from_unix_timestamp(1000);
    let result = add_seconds(t, 1000000);
    assert_eq!(to_unix_timestamp(result), Some(1001000));
}

#[test]
fn add_seconds_preserves_subseconds() {
    let t = UNIX_EPOCH + Duration::from_millis(500);
    let result = add_seconds(t, 1);
    let expected = UNIX_EPOCH + Duration::from_millis(1500);
    assert_eq!(result, expected);
}

// ============================================================================
// Tests for time_until
// ============================================================================

#[test]
fn time_until_past_deadline() {
    let past = from_unix_timestamp(1000);
    assert_eq!(time_until(past), None);
}

#[test]
fn time_until_epoch() {
    assert_eq!(time_until(UNIX_EPOCH), None);
}

#[test]
fn time_until_future_deadline() {
    // Set a deadline 1 hour in the future
    let now = SystemTime::now();
    let deadline = add_seconds(now, 3600);
    let remaining = time_until(deadline);
    assert!(remaining.is_some());
    // Should be close to 3600 seconds (allowing for some execution time)
    let secs = remaining.unwrap().as_secs();
    assert!(secs <= 3600, "Remaining time should be <= 3600 seconds");
    assert!(secs >= 3598, "Remaining time should be close to 3600 seconds");
}

#[test]
fn time_until_far_future() {
    // Year 3000
    let far_future = from_unix_timestamp(32503680000);
    let remaining = time_until(far_future);
    assert!(remaining.is_some());
    // Should be many years worth of seconds
    assert!(remaining.unwrap().as_secs() > 1000000000);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn integration_round_trip_timestamp() {
    let original = 1704067200u64;
    let time = from_unix_timestamp(original);
    let back = to_unix_timestamp(time);
    assert_eq!(back, Some(original));
}

#[test]
fn integration_add_and_compare() {
    let t1 = from_unix_timestamp(1000);
    let t2 = add_seconds(t1, 500);
    assert_eq!(seconds_between(t1, t2), Some(500));
}

#[test]
fn integration_past_future_consistency() {
    let now = SystemTime::now();
    let past = from_unix_timestamp(1000);
    let future = add_seconds(now, 3600);

    assert!(is_in_past(past));
    assert!(!is_in_future(past));
    assert!(is_in_future(future));
    assert!(!is_in_past(future));
}

#[test]
fn integration_time_until_and_add() {
    let now = SystemTime::now();
    let deadline = add_seconds(now, 120);
    let remaining = time_until(deadline);
    assert!(remaining.is_some());
    let secs = remaining.unwrap().as_secs();
    assert!(secs <= 120);
    assert!(secs >= 118);
}

#[test]
fn integration_seconds_between_and_to_timestamp() {
    let t1 = from_unix_timestamp(5000);
    let t2 = from_unix_timestamp(8000);
    let diff = seconds_between(t1, t2);
    assert_eq!(diff, Some(3000));

    let ts1 = to_unix_timestamp(t1).unwrap();
    let ts2 = to_unix_timestamp(t2).unwrap();
    assert_eq!(ts2 - ts1, 3000);
}

#[test]
fn integration_chain_add_seconds() {
    let t = from_unix_timestamp(1000);
    let t = add_seconds(t, 100);
    let t = add_seconds(t, 200);
    let t = add_seconds(t, 300);
    assert_eq!(to_unix_timestamp(t), Some(1600));
}

#[test]
fn integration_current_timestamp_to_systemtime() {
    let ts = current_unix_timestamp();
    let time = from_unix_timestamp(ts);
    let back = to_unix_timestamp(time);
    assert_eq!(back, Some(ts));
}

#[test]
fn integration_seconds_between_epoch_and_now() {
    let now = SystemTime::now();
    let secs = seconds_between(UNIX_EPOCH, now);
    assert!(secs.is_some());
    // Should be after year 2020
    assert!(secs.unwrap() > 1577836800);
}
