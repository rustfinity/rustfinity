use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp (seconds since January 1, 1970 UTC).
pub fn current_unix_timestamp() -> u64 {
    // TODO: Get the current time and convert to seconds since UNIX_EPOCH
    todo!()
}

/// Creates a `SystemTime` from a Unix timestamp.
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    // TODO: Add a Duration to UNIX_EPOCH
    todo!()
}

/// Converts a `SystemTime` to a Unix timestamp.
/// Returns `None` if the time is before the Unix epoch.
pub fn to_unix_timestamp(time: SystemTime) -> Option<u64> {
    // TODO: Use duration_since(UNIX_EPOCH) and handle the Result
    todo!()
}

/// Calculates the number of seconds between two times.
/// Returns `None` if `earlier` is actually after `later`.
pub fn seconds_between(earlier: SystemTime, later: SystemTime) -> Option<u64> {
    // TODO: Use duration_since and handle the Result
    todo!()
}

/// Checks if the given time is in the past (before the current time).
pub fn is_in_past(time: SystemTime) -> bool {
    // TODO: Compare with SystemTime::now()
    todo!()
}

/// Checks if the given time is in the future (after the current time).
pub fn is_in_future(time: SystemTime) -> bool {
    // TODO: Compare with SystemTime::now()
    todo!()
}

/// Adds a number of seconds to a `SystemTime`.
pub fn add_seconds(time: SystemTime, seconds: u64) -> SystemTime {
    // TODO: Add a Duration to the SystemTime
    todo!()
}

/// Returns the duration until the deadline.
/// Returns `None` if the deadline has already passed.
pub fn time_until(deadline: SystemTime) -> Option<Duration> {
    // TODO: Calculate duration from now until the deadline
    todo!()
}

fn main() {
    // Example: Get current Unix timestamp
    let now_ts = current_unix_timestamp();
    println!("Current Unix timestamp: {}", now_ts);

    // Example: Create SystemTime from timestamp
    let jan_2024 = from_unix_timestamp(1704067200);
    println!("Jan 1, 2024: {:?}", jan_2024);

    // Example: Convert back to timestamp
    if let Some(ts) = to_unix_timestamp(jan_2024) {
        println!("Converted back: {}", ts);
    }

    // Example: Calculate seconds between times
    let t1 = from_unix_timestamp(1000);
    let t2 = from_unix_timestamp(2500);
    if let Some(secs) = seconds_between(t1, t2) {
        println!("Seconds between: {}", secs);
    }

    // Example: Check past/future
    let past = from_unix_timestamp(1);
    println!("Is in past: {}", is_in_past(past));

    // Example: Add seconds
    let later = add_seconds(t1, 500);
    println!("After adding 500s: {:?}", later);

    // Example: Time until deadline
    let deadline = add_seconds(SystemTime::now(), 60);
    if let Some(remaining) = time_until(deadline) {
        println!("Time until deadline: {:?}", remaining);
    }
}
