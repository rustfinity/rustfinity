use std::cmp::Ordering;

/// A simple score wrapper with derived ordering.
///
/// # Examples
///
/// ```
/// use partialord_ord::Score;
///
/// let s1 = Score(100);
/// let s2 = Score(200);
/// assert!(s1 < s2);
/// assert!(s2 > s1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub u32);

/// A semantic version with major, minor, and patch components.
///
/// Ordering follows semantic versioning rules: major version takes precedence,
/// then minor, then patch.
///
/// # Examples
///
/// ```
/// use partialord_ord::Version;
///
/// let v1 = Version::new(1, 9, 0);
/// let v2 = Version::new(2, 0, 0);
/// assert!(v1 < v2);
///
/// let v3 = Version::new(1, 0, 0);
/// let v4 = Version::new(1, 0, 1);
/// assert!(v3 < v4);
/// ```
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

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

/// A temperature that can be in Celsius or Fahrenheit.
///
/// Comparisons are done after converting to Celsius, enabling comparison
/// across units. This only implements `PartialOrd` (not `Ord`) because
/// floating-point values have no total ordering (NaN issues).
///
/// # Examples
///
/// ```
/// use partialord_ord::Temperature;
///
/// let c = Temperature::Celsius(100.0);
/// let f = Temperature::Fahrenheit(212.0);  // 100°C
/// assert!(c == f);
///
/// let cold = Temperature::Celsius(0.0);
/// let freezing = Temperature::Fahrenheit(32.0);  // 0°C
/// assert!(cold == freezing);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Converts the temperature to Celsius.
    pub fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (*f - 32.0) * 5.0 / 9.0,
        }
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.0001;
        (self.to_celsius() - other.to_celsius()).abs() < epsilon
    }
}

// Temperature intentionally does NOT implement Eq because floating-point
// comparison is not reflexive for NaN.

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_celsius().partial_cmp(&other.to_celsius())
    }
}

// Temperature intentionally does NOT implement Ord because floating-point
// has no total ordering.

/// A priority level for tasks or events.
///
/// Ordering follows the declaration order: Low < Medium < High < Critical.
///
/// # Examples
///
/// ```
/// use partialord_ord::Priority;
///
/// assert!(Priority::Low < Priority::Medium);
/// assert!(Priority::Medium < Priority::High);
/// assert!(Priority::High < Priority::Critical);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// A player with a name and score.
///
/// Players are ordered by score descending (higher scores first), then
/// by name ascending as a tiebreaker.
///
/// # Examples
///
/// ```
/// use partialord_ord::Player;
///
/// let alice = Player::new("Alice", 100);
/// let bob = Player::new("Bob", 200);
/// assert!(bob < alice);  // Bob has higher score, comes first
///
/// let carol = Player::new("Carol", 100);
/// assert!(alice < carol);  // Same score, alphabetical order
/// ```
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

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by score descending (reverse comparison), then name ascending
        other
            .score
            .cmp(&self.score)
            .then(self.name.cmp(&other.name))
    }
}

/// Returns a reference to the minimum element in a slice.
///
/// Returns `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use partialord_ord::find_min;
///
/// assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
/// assert_eq!(find_min(&[42]), Some(&42));
/// assert_eq!(find_min::<i32>(&[]), None);
/// ```
pub fn find_min<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().min()
}

/// Returns a reference to the maximum element in a slice.
///
/// Returns `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use partialord_ord::find_max;
///
/// assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(&5));
/// assert_eq!(find_max(&[42]), Some(&42));
/// assert_eq!(find_max::<i32>(&[]), None);
/// ```
pub fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    items.iter().max()
}

/// Checks if a slice is sorted in ascending order.
///
/// Returns `true` if the slice is sorted (each element <= the next).
/// Empty slices and single-element slices are considered sorted.
///
/// # Examples
///
/// ```
/// use partialord_ord::is_sorted;
///
/// assert!(is_sorted(&[1, 2, 3, 4, 5]));
/// assert!(is_sorted(&[1, 1, 2, 2, 3]));  // Equal elements are ok
/// assert!(!is_sorted(&[1, 3, 2]));
/// assert!(is_sorted::<i32>(&[]));
/// assert!(is_sorted(&[42]));
/// ```
pub fn is_sorted<T: Ord>(items: &[T]) -> bool {
    items.windows(2).all(|w| w[0] <= w[1])
}

/// Clamps a value to be within the given bounds (inclusive).
///
/// If the value is less than min, returns min.
/// If the value is greater than max, returns max.
/// Otherwise, returns the value itself.
///
/// # Examples
///
/// ```
/// use partialord_ord::clamp;
///
/// assert_eq!(clamp(&5, &1, &10), &5);   // Within bounds
/// assert_eq!(clamp(&0, &1, &10), &1);   // Below min
/// assert_eq!(clamp(&15, &1, &10), &10); // Above max
/// ```
pub fn clamp<'a, T: Ord>(value: &'a T, min: &'a T, max: &'a T) -> &'a T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
