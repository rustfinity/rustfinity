use std::cmp::Ordering;

/// A simple score wrapper with derived ordering.
///
/// TODO: Add the appropriate derive macros for PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy.
#[derive(Debug, Clone, Copy)]
pub struct Score(pub u32);

/// A semantic version with major, minor, and patch components.
///
/// TODO: Implement PartialOrd and Ord manually to compare versions semantically
/// (major first, then minor, then patch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Creates a new Version with the given components.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }
}

// TODO: Implement PartialOrd for Version

// TODO: Implement Ord for Version

/// A temperature that can be in Celsius or Fahrenheit.
///
/// TODO: Implement PartialEq and PartialOrd to compare temperatures after
/// converting to a common unit. Do NOT implement Eq or Ord (floating-point
/// has no total ordering).
#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Converts the temperature to Celsius.
    pub fn to_celsius(&self) -> f64 {
        // TODO
        unimplemented!()
    }
}

// TODO: Implement PartialEq for Temperature

// TODO: Implement PartialOrd for Temperature

/// A priority level for tasks or events.
///
/// TODO: Add the appropriate derive macros so that ordering follows the
/// declaration order: Low < Medium < High < Critical.
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// A player with a name and score.
///
/// TODO: Implement Ord to sort by score descending (higher scores first),
/// then by name ascending as a tiebreaker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

impl Player {
    /// Creates a new Player with the given name and score.
    pub fn new(name: &str, score: u32) -> Self {
        Player {
            name: name.to_string(),
            score,
        }
    }
}

// TODO: Implement PartialOrd for Player

// TODO: Implement Ord for Player

/// Returns a reference to the minimum element in a slice.
///
/// Returns `None` if the slice is empty.
pub fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    // TODO
    unimplemented!()
}

/// Returns a reference to the maximum element in a slice.
///
/// Returns `None` if the slice is empty.
pub fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    // TODO
    unimplemented!()
}

/// Checks if a slice is sorted in ascending order.
///
/// Returns `true` if the slice is sorted (each element <= the next).
/// Empty slices and single-element slices are considered sorted.
pub fn is_sorted<T: Ord>(items: &[T]) -> bool {
    // TODO
    unimplemented!()
}

/// Clamps a value to be within the given bounds (inclusive).
///
/// If the value is less than min, returns min.
/// If the value is greater than max, returns max.
/// Otherwise, returns the value itself.
pub fn clamp<'a, T: Ord>(value: &'a T, min: &'a T, max: &'a T) -> &'a T {
    // TODO
    unimplemented!()
}

// Example usage
pub fn main() {
    // Score - derived ordering
    let s1 = Score(100);
    let s2 = Score(200);
    println!("{:?} < {:?}: {}", s1, s2, s1 < s2);

    // Version - semantic versioning order
    let v1 = Version::new(1, 9, 0);
    let v2 = Version::new(2, 0, 0);
    println!("{:?} < {:?}: {}", v1, v2, v1 < v2);

    // Temperature - partial ordering across units
    let t1 = Temperature::Celsius(100.0);
    let t2 = Temperature::Fahrenheit(212.0);
    println!("{:?} == {:?}: {}", t1, t2, t1 == t2);

    // Priority - enum ordering
    println!(
        "Priority::Low < Priority::Critical: {}",
        Priority::Low < Priority::Critical
    );

    // Player - custom ordering
    let alice = Player::new("Alice", 100);
    let bob = Player::new("Bob", 200);
    println!("{:?} vs {:?}", alice, bob);

    // Utility functions
    let numbers = [3, 1, 4, 1, 5];
    println!("find_min: {:?}", find_min(&numbers));
    println!("find_max: {:?}", find_max(&numbers));
    println!("is_sorted: {}", is_sorted(&[1, 2, 3, 4, 5]));
    println!("clamp(5, 1, 10): {:?}", clamp(&5, &1, &10));
}
