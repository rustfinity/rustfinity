The `std::time::Duration` type represents a span of time, typically used for system timeouts, sleep operations, and measuring elapsed time. Unlike calendar time (which can have leap seconds and timezone issues), a `Duration` is a simple, monotonic count of seconds and nanoseconds.

`Duration` provides methods to create durations from various units (seconds, milliseconds, microseconds, nanoseconds), extract values in different units, and perform arithmetic operations. It's designed to be precise and overflow-safe, making it ideal for timing operations in systems programming.

## Creating Durations

```rust
use std::time::Duration;

// From various units
let d1 = Duration::from_secs(5);           // 5 seconds
let d2 = Duration::from_millis(1500);      // 1.5 seconds
let d3 = Duration::from_micros(1_000_000); // 1 second
let d4 = Duration::from_nanos(1_000_000_000); // 1 second
let d5 = Duration::from_secs_f64(2.5);     // 2.5 seconds

// Zero and maximum
let zero = Duration::ZERO;
let max = Duration::MAX;
```

## Extracting Values

```rust
use std::time::Duration;

let d = Duration::from_millis(2500);

// Total values (truncates)
d.as_secs();    // 2
d.as_millis();  // 2500
d.as_micros();  // 2_500_000
d.as_nanos();   // 2_500_000_000

// Floating point total
d.as_secs_f64(); // 2.5
d.as_secs_f32(); // 2.5

// Sub-second component (nanoseconds portion)
d.subsec_nanos();  // 500_000_000
d.subsec_micros(); // 500_000
d.subsec_millis(); // 500
```

## Arithmetic Operations

```rust
use std::time::Duration;

let d1 = Duration::from_secs(5);
let d2 = Duration::from_secs(3);

// Addition and subtraction
let sum = d1 + d2;           // 8 seconds
let diff = d1 - d2;          // 2 seconds

// Checked operations (return Option)
let checked = d1.checked_add(d2);  // Some(8 seconds)
let overflow = Duration::MAX.checked_add(d1); // None

// Saturating operations (clamp at bounds)
let saturated = Duration::MAX.saturating_add(d1); // Duration::MAX

// Multiplication and division
let doubled = d1 * 2;        // 10 seconds
let halved = d1 / 2;         // 2.5 seconds
```

## Your Task

Implement the following functions to work with `Duration`:

**1. `from_minutes(minutes: u64) -> Duration`**

Create a Duration from a number of minutes.

**2. `from_hours(hours: u64) -> Duration`**

Create a Duration from a number of hours.

**3. `to_minutes(duration: Duration) -> u64`**

Convert a Duration to total minutes (truncated).

**4. `to_hours(duration: Duration) -> u64`**

Convert a Duration to total hours (truncated).

**5. `format_duration(duration: Duration) -> String`**

Format a duration as "Xh Ym Zs" (e.g., "2h 30m 45s").
Omit zero components at the start (e.g., "30m 45s" not
"0h 30m 45s"), but always show seconds.

**6. `add_durations(durations: &[Duration]) -> Duration`**

Sum all durations in a slice. Return `Duration::ZERO` for
an empty slice.

**7. `average_duration(durations: &[Duration]) -> Option<Duration>`**

Calculate the average duration. Return `None` for an
empty slice.

**8. `is_longer_than(d1: Duration, d2: Duration) -> bool`**

Check if the first duration is strictly longer than the
second.

## Examples

```rust
use std::time::Duration;

// Creating from minutes and hours
let thirty_min = from_minutes(30);
assert_eq!(thirty_min, Duration::from_secs(1800));

let two_hours = from_hours(2);
assert_eq!(two_hours, Duration::from_secs(7200));

// Converting to minutes and hours
let d = Duration::from_secs(7500); // 2h 5m
assert_eq!(to_minutes(d), 125);
assert_eq!(to_hours(d), 2);

// Formatting
let d = Duration::from_secs(9045); // 2h 30m 45s
assert_eq!(format_duration(d), "2h 30m 45s");

let d = Duration::from_secs(45);
assert_eq!(format_duration(d), "45s");

let d = Duration::from_secs(1845); // 30m 45s
assert_eq!(format_duration(d), "30m 45s");

// Adding durations
let durations = vec![
    Duration::from_secs(60),
    Duration::from_secs(120),
    Duration::from_secs(180),
];
assert_eq!(add_durations(&durations), Duration::from_secs(360));

// Average duration
let durations = vec![
    Duration::from_secs(10),
    Duration::from_secs(20),
    Duration::from_secs(30),
];
assert_eq!(
    average_duration(&durations),
    Some(Duration::from_secs(20))
);
assert_eq!(average_duration(&[]), None);

// Comparison
assert!(is_longer_than(Duration::from_secs(10), Duration::from_secs(5)));
assert!(!is_longer_than(Duration::from_secs(5), Duration::from_secs(5)));
```

## Hints

<details>
  <summary>Click here for hints</summary>

- One minute is 60 seconds, one hour is 3600 seconds
- Use `as_secs()` to get the total seconds, then divide to get minutes/hours
- For `format_duration`, calculate hours, minutes, and seconds separately using division and modulo
- Use `Duration::ZERO` for the identity element when summing
- For average, sum all durations then divide by the count
- `Duration` implements `Ord`, so you can use comparison operators directly
- Remember that `Duration` division by a number uses `duration / n` where n is `u32`

</details>
