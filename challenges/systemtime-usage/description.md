The `std::time::SystemTime` type represents a point in time according to the system clock. Unlike `Instant` (which is monotonic and only useful for measuring elapsed time), `SystemTime` can be converted to and from calendar time, making it useful for timestamps, file modification times, and any operation that needs to interact with real-world time.

`SystemTime` is anchored to a meaningful point: `UNIX_EPOCH` (January 1, 1970, 00:00:00 UTC). You can compute the duration since the epoch to get a Unix timestamp, or compare two `SystemTime` values to find the elapsed time between them.

## Key Concepts

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

// Get the current time
let now = SystemTime::now();

// Get duration since Unix epoch (Unix timestamp)
let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
let unix_timestamp = since_epoch.as_secs();

// Create a SystemTime from a Unix timestamp
let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
let time = UNIX_EPOCH + Duration::from_secs(timestamp);

// Compare times
let earlier = UNIX_EPOCH + Duration::from_secs(1000);
let later = UNIX_EPOCH + Duration::from_secs(2000);
let elapsed = later.duration_since(earlier).unwrap(); // 1000 seconds
```

## Error Handling

`duration_since` returns a `Result` because the second time might be later than the first (which would require negative duration). This can happen due to clock adjustments or when comparing times incorrectly:

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

let earlier = UNIX_EPOCH + Duration::from_secs(1000);
let later = UNIX_EPOCH + Duration::from_secs(2000);

// This works - later is after earlier
let ok = later.duration_since(earlier); // Ok(Duration::from_secs(1000))

// This fails - earlier is before later
let err = earlier.duration_since(later); // Err(SystemTimeError)
```

## Your Task

Implement the following functions to work with `SystemTime`:

### 1. `current_unix_timestamp() -> u64`

Return the current Unix timestamp (seconds since January 1, 1970 UTC).

### 2. `from_unix_timestamp(timestamp: u64) -> SystemTime`

Create a `SystemTime` from a Unix timestamp.

### 3. `to_unix_timestamp(time: SystemTime) -> Option<u64>`

Convert a `SystemTime` to a Unix timestamp. Return `None` if the time is before the Unix epoch.

### 4. `seconds_between(earlier: SystemTime, later: SystemTime) -> Option<u64>`

Calculate the number of seconds between two times. Return `None` if `earlier` is actually after `later`.

### 5. `is_in_past(time: SystemTime) -> bool`

Check if the given time is in the past (before the current time).

### 6. `is_in_future(time: SystemTime) -> bool`

Check if the given time is in the future (after the current time).

### 7. `add_seconds(time: SystemTime, seconds: u64) -> SystemTime`

Add a number of seconds to a `SystemTime`.

### 8. `time_until(deadline: SystemTime) -> Option<Duration>`

Return the duration until the deadline. Return `None` if the deadline has already passed.

## Examples

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};

// Unix timestamp operations
let timestamp = 1704067200; // Jan 1, 2024 00:00:00 UTC
let time = from_unix_timestamp(timestamp);
assert_eq!(to_unix_timestamp(time), Some(timestamp));

// Seconds between times
let t1 = from_unix_timestamp(1000);
let t2 = from_unix_timestamp(2500);
assert_eq!(seconds_between(t1, t2), Some(1500));
assert_eq!(seconds_between(t2, t1), None); // t2 is after t1

// Past and future checks
let past = UNIX_EPOCH + Duration::from_secs(1);
assert!(is_in_past(past));

let far_future = UNIX_EPOCH + Duration::from_secs(u64::MAX / 2);
assert!(is_in_future(far_future));

// Adding time
let t = from_unix_timestamp(1000);
let later = add_seconds(t, 500);
assert_eq!(to_unix_timestamp(later), Some(1500));

// Time until deadline
let now = SystemTime::now();
let deadline = add_seconds(now, 60);
let remaining = time_until(deadline);
assert!(remaining.is_some());
assert!(remaining.unwrap().as_secs() <= 60);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `SystemTime::now()` to get the current time
- `UNIX_EPOCH` is available from `std::time`
- Use `Duration::from_secs()` to create durations from seconds
- `SystemTime` supports `+` and `-` with `Duration`
- `duration_since()` returns `Result<Duration, SystemTimeError>`
- Use `.ok()` to convert a `Result` to an `Option`, discarding the error
- For `is_in_past` and `is_in_future`, compare with `SystemTime::now()`
- Remember that `duration_since` fails if the argument is after `self`

</details>
