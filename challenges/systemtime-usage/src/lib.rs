use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp (seconds since January 1, 1970 UTC).
///
/// # Examples
///
/// ```
/// use systemtime_usage::current_unix_timestamp;
///
/// let timestamp = current_unix_timestamp();
/// // Should be a reasonable timestamp (after year 2020)
/// assert!(timestamp > 1577836800); // Jan 1, 2020
/// ```
pub fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before Unix epoch")
        .as_secs()
}

/// Creates a `SystemTime` from a Unix timestamp.
///
/// # Examples
///
/// ```
/// use systemtime_usage::from_unix_timestamp;
/// use std::time::{UNIX_EPOCH, Duration};
///
/// let time = from_unix_timestamp(1704067200);
/// assert_eq!(time, UNIX_EPOCH + Duration::from_secs(1704067200));
/// ```
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(timestamp)
}

/// Converts a `SystemTime` to a Unix timestamp.
///
/// Returns `None` if the time is before the Unix epoch.
///
/// # Examples
///
/// ```
/// use systemtime_usage::{from_unix_timestamp, to_unix_timestamp};
///
/// let time = from_unix_timestamp(1704067200);
/// assert_eq!(to_unix_timestamp(time), Some(1704067200));
/// ```
pub fn to_unix_timestamp(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}

/// Calculates the number of seconds between two times.
///
/// Returns `None` if `earlier` is actually after `later`.
///
/// # Examples
///
/// ```
/// use systemtime_usage::{from_unix_timestamp, seconds_between};
///
/// let t1 = from_unix_timestamp(1000);
/// let t2 = from_unix_timestamp(2500);
/// assert_eq!(seconds_between(t1, t2), Some(1500));
/// assert_eq!(seconds_between(t2, t1), None);
/// ```
pub fn seconds_between(earlier: SystemTime, later: SystemTime) -> Option<u64> {
    later.duration_since(earlier).ok().map(|d| d.as_secs())
}

/// Checks if the given time is in the past (before the current time).
///
/// # Examples
///
/// ```
/// use systemtime_usage::is_in_past;
/// use std::time::{UNIX_EPOCH, Duration};
///
/// let past = UNIX_EPOCH + Duration::from_secs(1);
/// assert!(is_in_past(past));
/// ```
pub fn is_in_past(time: SystemTime) -> bool {
    time < SystemTime::now()
}

/// Checks if the given time is in the future (after the current time).
///
/// # Examples
///
/// ```
/// use systemtime_usage::is_in_future;
/// use std::time::{UNIX_EPOCH, Duration};
///
/// // A time far in the future
/// let future = UNIX_EPOCH + Duration::from_secs(u64::MAX / 2);
/// assert!(is_in_future(future));
/// ```
pub fn is_in_future(time: SystemTime) -> bool {
    time > SystemTime::now()
}

/// Adds a number of seconds to a `SystemTime`.
///
/// # Examples
///
/// ```
/// use systemtime_usage::{from_unix_timestamp, add_seconds, to_unix_timestamp};
///
/// let t = from_unix_timestamp(1000);
/// let later = add_seconds(t, 500);
/// assert_eq!(to_unix_timestamp(later), Some(1500));
/// ```
pub fn add_seconds(time: SystemTime, seconds: u64) -> SystemTime {
    time + Duration::from_secs(seconds)
}

/// Returns the duration until the deadline.
///
/// Returns `None` if the deadline has already passed.
///
/// # Examples
///
/// ```
/// use systemtime_usage::{add_seconds, time_until};
/// use std::time::SystemTime;
///
/// let now = SystemTime::now();
/// let deadline = add_seconds(now, 60);
/// let remaining = time_until(deadline);
/// assert!(remaining.is_some());
/// assert!(remaining.unwrap().as_secs() <= 60);
/// ```
pub fn time_until(deadline: SystemTime) -> Option<Duration> {
    deadline.duration_since(SystemTime::now()).ok()
}
