use std::time::Duration;

/// Creates a Duration from a number of minutes.
///
/// # Examples
///
/// ```
/// use duration_operations::from_minutes;
/// use std::time::Duration;
///
/// let d = from_minutes(30);
/// assert_eq!(d, Duration::from_secs(1800));
/// ```
pub fn from_minutes(minutes: u64) -> Duration {
    Duration::from_secs(minutes * 60)
}

/// Creates a Duration from a number of hours.
///
/// # Examples
///
/// ```
/// use duration_operations::from_hours;
/// use std::time::Duration;
///
/// let d = from_hours(2);
/// assert_eq!(d, Duration::from_secs(7200));
/// ```
pub fn from_hours(hours: u64) -> Duration {
    Duration::from_secs(hours * 3600)
}

/// Converts a Duration to total minutes (truncated).
///
/// # Examples
///
/// ```
/// use duration_operations::to_minutes;
/// use std::time::Duration;
///
/// let d = Duration::from_secs(7500);
/// assert_eq!(to_minutes(d), 125);
/// ```
pub fn to_minutes(duration: Duration) -> u64 {
    duration.as_secs() / 60
}

/// Converts a Duration to total hours (truncated).
///
/// # Examples
///
/// ```
/// use duration_operations::to_hours;
/// use std::time::Duration;
///
/// let d = Duration::from_secs(7500);
/// assert_eq!(to_hours(d), 2);
/// ```
pub fn to_hours(duration: Duration) -> u64 {
    duration.as_secs() / 3600
}

/// Formats a duration as "Xh Ym Zs".
///
/// Omits zero components at the start (e.g., "30m 45s" not "0h 30m 45s"),
/// but always shows seconds.
///
/// # Examples
///
/// ```
/// use duration_operations::format_duration;
/// use std::time::Duration;
///
/// let d = Duration::from_secs(9045);
/// assert_eq!(format_duration(d), "2h 30m 45s");
///
/// let d = Duration::from_secs(45);
/// assert_eq!(format_duration(d), "45s");
///
/// let d = Duration::from_secs(1845);
/// assert_eq!(format_duration(d), "30m 45s");
/// ```
pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Sums all durations in a slice.
///
/// Returns `Duration::ZERO` for an empty slice.
///
/// # Examples
///
/// ```
/// use duration_operations::add_durations;
/// use std::time::Duration;
///
/// let durations = vec![
///     Duration::from_secs(60),
///     Duration::from_secs(120),
///     Duration::from_secs(180),
/// ];
/// assert_eq!(add_durations(&durations), Duration::from_secs(360));
///
/// assert_eq!(add_durations(&[]), Duration::ZERO);
/// ```
pub fn add_durations(durations: &[Duration]) -> Duration {
    durations.iter().fold(Duration::ZERO, |acc, &d| acc + d)
}

/// Calculates the average duration.
///
/// Returns `None` for an empty slice.
///
/// # Examples
///
/// ```
/// use duration_operations::average_duration;
/// use std::time::Duration;
///
/// let durations = vec![
///     Duration::from_secs(10),
///     Duration::from_secs(20),
///     Duration::from_secs(30),
/// ];
/// assert_eq!(average_duration(&durations), Some(Duration::from_secs(20)));
///
/// assert_eq!(average_duration(&[]), None);
/// ```
pub fn average_duration(durations: &[Duration]) -> Option<Duration> {
    if durations.is_empty() {
        return None;
    }
    let total = add_durations(durations);
    Some(total / durations.len() as u32)
}

/// Checks if the first duration is strictly longer than the second.
///
/// # Examples
///
/// ```
/// use duration_operations::is_longer_than;
/// use std::time::Duration;
///
/// assert!(is_longer_than(Duration::from_secs(10), Duration::from_secs(5)));
/// assert!(!is_longer_than(Duration::from_secs(5), Duration::from_secs(5)));
/// assert!(!is_longer_than(Duration::from_secs(3), Duration::from_secs(5)));
/// ```
pub fn is_longer_than(d1: Duration, d2: Duration) -> bool {
    d1 > d2
}
