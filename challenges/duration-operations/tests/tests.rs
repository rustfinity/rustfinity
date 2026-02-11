use duration_operations::*;
use std::time::Duration;

// =============================================================================
// from_minutes tests
// =============================================================================

#[test]
fn test_from_minutes_zero() {
    assert_eq!(from_minutes(0), Duration::ZERO);
}

#[test]
fn test_from_minutes_one() {
    assert_eq!(from_minutes(1), Duration::from_secs(60));
}

#[test]
fn test_from_minutes_thirty() {
    assert_eq!(from_minutes(30), Duration::from_secs(1800));
}

#[test]
fn test_from_minutes_sixty() {
    assert_eq!(from_minutes(60), Duration::from_secs(3600));
}

#[test]
fn test_from_minutes_large() {
    assert_eq!(from_minutes(1440), Duration::from_secs(86400)); // 24 hours
}

// =============================================================================
// from_hours tests
// =============================================================================

#[test]
fn test_from_hours_zero() {
    assert_eq!(from_hours(0), Duration::ZERO);
}

#[test]
fn test_from_hours_one() {
    assert_eq!(from_hours(1), Duration::from_secs(3600));
}

#[test]
fn test_from_hours_two() {
    assert_eq!(from_hours(2), Duration::from_secs(7200));
}

#[test]
fn test_from_hours_twenty_four() {
    assert_eq!(from_hours(24), Duration::from_secs(86400));
}

#[test]
fn test_from_hours_large() {
    assert_eq!(from_hours(168), Duration::from_secs(604800)); // 1 week
}

// =============================================================================
// to_minutes tests
// =============================================================================

#[test]
fn test_to_minutes_zero() {
    assert_eq!(to_minutes(Duration::ZERO), 0);
}

#[test]
fn test_to_minutes_one_minute() {
    assert_eq!(to_minutes(Duration::from_secs(60)), 1);
}

#[test]
fn test_to_minutes_partial_minute() {
    assert_eq!(to_minutes(Duration::from_secs(90)), 1); // truncates
}

#[test]
fn test_to_minutes_exact() {
    assert_eq!(to_minutes(Duration::from_secs(1800)), 30);
}

#[test]
fn test_to_minutes_large() {
    assert_eq!(to_minutes(Duration::from_secs(7500)), 125);
}

#[test]
fn test_to_minutes_with_millis() {
    assert_eq!(to_minutes(Duration::from_millis(90500)), 1); // sub-second ignored
}

// =============================================================================
// to_hours tests
// =============================================================================

#[test]
fn test_to_hours_zero() {
    assert_eq!(to_hours(Duration::ZERO), 0);
}

#[test]
fn test_to_hours_one_hour() {
    assert_eq!(to_hours(Duration::from_secs(3600)), 1);
}

#[test]
fn test_to_hours_partial_hour() {
    assert_eq!(to_hours(Duration::from_secs(5400)), 1); // 1.5 hours, truncates
}

#[test]
fn test_to_hours_exact() {
    assert_eq!(to_hours(Duration::from_secs(7200)), 2);
}

#[test]
fn test_to_hours_large() {
    assert_eq!(to_hours(Duration::from_secs(86400)), 24);
}

#[test]
fn test_to_hours_less_than_hour() {
    assert_eq!(to_hours(Duration::from_secs(1800)), 0);
}

// =============================================================================
// format_duration tests
// =============================================================================

#[test]
fn test_format_duration_zero() {
    assert_eq!(format_duration(Duration::ZERO), "0s");
}

#[test]
fn test_format_duration_seconds_only() {
    assert_eq!(format_duration(Duration::from_secs(45)), "45s");
}

#[test]
fn test_format_duration_one_second() {
    assert_eq!(format_duration(Duration::from_secs(1)), "1s");
}

#[test]
fn test_format_duration_minutes_and_seconds() {
    assert_eq!(format_duration(Duration::from_secs(1845)), "30m 45s");
}

#[test]
fn test_format_duration_minutes_zero_seconds() {
    assert_eq!(format_duration(Duration::from_secs(1800)), "30m 0s");
}

#[test]
fn test_format_duration_one_minute() {
    assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
}

#[test]
fn test_format_duration_hours_minutes_seconds() {
    assert_eq!(format_duration(Duration::from_secs(9045)), "2h 30m 45s");
}

#[test]
fn test_format_duration_hours_zero_minutes_zero_seconds() {
    assert_eq!(format_duration(Duration::from_secs(7200)), "2h 0m 0s");
}

#[test]
fn test_format_duration_hours_only_zero_minutes() {
    assert_eq!(format_duration(Duration::from_secs(7245)), "2h 0m 45s");
}

#[test]
fn test_format_duration_large() {
    assert_eq!(format_duration(Duration::from_secs(90061)), "25h 1m 1s");
}

#[test]
fn test_format_duration_fifty_nine_seconds() {
    assert_eq!(format_duration(Duration::from_secs(59)), "59s");
}

#[test]
fn test_format_duration_fifty_nine_minutes() {
    assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
}

// =============================================================================
// add_durations tests
// =============================================================================

#[test]
fn test_add_durations_empty() {
    assert_eq!(add_durations(&[]), Duration::ZERO);
}

#[test]
fn test_add_durations_single() {
    let durations = [Duration::from_secs(60)];
    assert_eq!(add_durations(&durations), Duration::from_secs(60));
}

#[test]
fn test_add_durations_multiple() {
    let durations = vec![
        Duration::from_secs(60),
        Duration::from_secs(120),
        Duration::from_secs(180),
    ];
    assert_eq!(add_durations(&durations), Duration::from_secs(360));
}

#[test]
fn test_add_durations_with_zero() {
    let durations = vec![
        Duration::from_secs(100),
        Duration::ZERO,
        Duration::from_secs(50),
    ];
    assert_eq!(add_durations(&durations), Duration::from_secs(150));
}

#[test]
fn test_add_durations_all_same() {
    let durations = vec![Duration::from_secs(10); 5];
    assert_eq!(add_durations(&durations), Duration::from_secs(50));
}

#[test]
fn test_add_durations_with_millis() {
    let durations = vec![
        Duration::from_millis(500),
        Duration::from_millis(750),
        Duration::from_millis(250),
    ];
    assert_eq!(add_durations(&durations), Duration::from_millis(1500));
}

// =============================================================================
// average_duration tests
// =============================================================================

#[test]
fn test_average_duration_empty() {
    assert_eq!(average_duration(&[]), None);
}

#[test]
fn test_average_duration_single() {
    let durations = [Duration::from_secs(60)];
    assert_eq!(average_duration(&durations), Some(Duration::from_secs(60)));
}

#[test]
fn test_average_duration_multiple() {
    let durations = vec![
        Duration::from_secs(10),
        Duration::from_secs(20),
        Duration::from_secs(30),
    ];
    assert_eq!(average_duration(&durations), Some(Duration::from_secs(20)));
}

#[test]
fn test_average_duration_even() {
    let durations = vec![
        Duration::from_secs(100),
        Duration::from_secs(200),
    ];
    assert_eq!(average_duration(&durations), Some(Duration::from_secs(150)));
}

#[test]
fn test_average_duration_preserves_precision() {
    let durations = vec![
        Duration::from_secs(10),
        Duration::from_secs(11),
    ];
    // Average is 10.5 seconds - Duration division preserves subsecond precision
    assert_eq!(average_duration(&durations), Some(Duration::from_millis(10500)));
}

#[test]
fn test_average_duration_all_zeros() {
    let durations = vec![Duration::ZERO; 3];
    assert_eq!(average_duration(&durations), Some(Duration::ZERO));
}

#[test]
fn test_average_duration_with_millis() {
    let durations = vec![
        Duration::from_millis(1000),
        Duration::from_millis(2000),
        Duration::from_millis(3000),
    ];
    assert_eq!(average_duration(&durations), Some(Duration::from_millis(2000)));
}

// =============================================================================
// is_longer_than tests
// =============================================================================

#[test]
fn test_is_longer_than_true() {
    assert!(is_longer_than(Duration::from_secs(10), Duration::from_secs(5)));
}

#[test]
fn test_is_longer_than_false_equal() {
    assert!(!is_longer_than(Duration::from_secs(5), Duration::from_secs(5)));
}

#[test]
fn test_is_longer_than_false_shorter() {
    assert!(!is_longer_than(Duration::from_secs(3), Duration::from_secs(5)));
}

#[test]
fn test_is_longer_than_zero() {
    assert!(is_longer_than(Duration::from_secs(1), Duration::ZERO));
    assert!(!is_longer_than(Duration::ZERO, Duration::ZERO));
    assert!(!is_longer_than(Duration::ZERO, Duration::from_secs(1)));
}

#[test]
fn test_is_longer_than_millis() {
    assert!(is_longer_than(Duration::from_millis(1001), Duration::from_secs(1)));
    assert!(!is_longer_than(Duration::from_millis(999), Duration::from_secs(1)));
}

#[test]
fn test_is_longer_than_nanos() {
    let d1 = Duration::from_nanos(1_000_000_001);
    let d2 = Duration::from_secs(1);
    assert!(is_longer_than(d1, d2));
}

// =============================================================================
// Integration tests
// =============================================================================

#[test]
fn test_round_trip_minutes() {
    let original = 45u64;
    let duration = from_minutes(original);
    assert_eq!(to_minutes(duration), original);
}

#[test]
fn test_round_trip_hours() {
    let original = 12u64;
    let duration = from_hours(original);
    assert_eq!(to_hours(duration), original);
}

#[test]
fn test_format_and_parse_consistency() {
    // 1 hour, 30 minutes, 15 seconds = 5415 seconds
    let duration = Duration::from_secs(5415);
    let formatted = format_duration(duration);
    assert_eq!(formatted, "1h 30m 15s");
}

#[test]
fn test_add_and_average_relationship() {
    let durations = vec![
        Duration::from_secs(100),
        Duration::from_secs(200),
        Duration::from_secs(300),
    ];
    let sum = add_durations(&durations);
    let avg = average_duration(&durations).unwrap();
    assert_eq!(sum, Duration::from_secs(600));
    assert_eq!(avg, Duration::from_secs(200));
    assert_eq!(sum, avg * 3);
}

#[test]
fn test_comparison_with_converted_values() {
    let one_hour = from_hours(1);
    let sixty_minutes = from_minutes(60);
    assert!(!is_longer_than(one_hour, sixty_minutes));
    assert!(!is_longer_than(sixty_minutes, one_hour));
}

#[test]
fn test_mixed_units_addition() {
    let durations = vec![
        from_hours(1),           // 3600s
        from_minutes(30),        // 1800s
        Duration::from_secs(45), // 45s
    ];
    let total = add_durations(&durations);
    assert_eq!(total, Duration::from_secs(5445));
    assert_eq!(format_duration(total), "1h 30m 45s");
}

#[test]
fn test_edge_case_just_under_minute() {
    let d = Duration::from_secs(59);
    assert_eq!(to_minutes(d), 0);
    assert_eq!(format_duration(d), "59s");
}

#[test]
fn test_edge_case_just_under_hour() {
    let d = Duration::from_secs(3599);
    assert_eq!(to_hours(d), 0);
    assert_eq!(to_minutes(d), 59);
    assert_eq!(format_duration(d), "59m 59s");
}

#[test]
fn test_subsecond_precision_ignored_in_format() {
    let d = Duration::from_millis(5500); // 5.5 seconds
    assert_eq!(format_duration(d), "5s");
}

#[test]
fn test_many_small_durations() {
    let durations: Vec<Duration> = (0..100).map(|_| Duration::from_millis(10)).collect();
    assert_eq!(add_durations(&durations), Duration::from_secs(1));
}
