use std::hash::Hash;

/// A 2D point with x and y coordinates.
///
/// Uses derived `PartialEq` and `Eq` for standard field-by-field comparison.
///
/// # Examples
///
/// ```
/// use partialeq_eq::Point;
///
/// let p1 = Point { x: 1, y: 2 };
/// let p2 = Point { x: 1, y: 2 };
/// let p3 = Point { x: 3, y: 4 };
/// assert_eq!(p1, p2);
/// assert_ne!(p1, p3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A string wrapper that compares equality case-insensitively.
///
/// # Examples
///
/// ```
/// use partialeq_eq::CaseInsensitiveString;
///
/// let s1 = CaseInsensitiveString::new("Hello");
/// let s2 = CaseInsensitiveString::new("HELLO");
/// let s3 = CaseInsensitiveString::new("World");
/// assert_eq!(s1, s2);
/// assert_ne!(s1, s3);
/// ```
#[derive(Debug, Clone)]
pub struct CaseInsensitiveString(pub String);

impl CaseInsensitiveString {
    /// Creates a new CaseInsensitiveString from any string-like type.
    pub fn new<S: Into<String>>(s: S) -> Self {
        CaseInsensitiveString(s.into())
    }

    /// Returns the inner string value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

/// A floating-point wrapper that compares values within an epsilon.
///
/// This type implements `PartialEq` but NOT `Eq` because floating-point
/// equality is not total (e.g., NaN != NaN).
///
/// # Examples
///
/// ```
/// use partialeq_eq::ApproximateFloat;
///
/// let f1 = ApproximateFloat(1.0);
/// let f2 = ApproximateFloat(1.00005);  // Within epsilon
/// let f3 = ApproximateFloat(1.001);    // Outside epsilon
/// assert_eq!(f1, f2);
/// assert_ne!(f1, f3);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ApproximateFloat(pub f64);

/// The epsilon value used for approximate float comparisons.
pub const EPSILON: f64 = 0.0001;

impl PartialEq for ApproximateFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
    }
}

// Note: ApproximateFloat does NOT implement Eq because floating-point
// comparison is not total (NaN != NaN).

/// A unique user identifier that can be used as a HashMap key.
///
/// # Examples
///
/// ```
/// use partialeq_eq::UserId;
/// use std::collections::HashSet;
///
/// let mut ids: HashSet<UserId> = HashSet::new();
/// ids.insert(UserId(1));
/// ids.insert(UserId(2));
/// assert!(ids.contains(&UserId(1)));
/// assert!(!ids.contains(&UserId(99)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

/// A person with a name and unique ID.
///
/// Equality is based ONLY on the `id` field, not the name.
///
/// # Examples
///
/// ```
/// use partialeq_eq::Person;
///
/// let alice1 = Person { name: String::from("Alice"), id: 1 };
/// let alice2 = Person { name: String::from("Alice Smith"), id: 1 };
/// let bob = Person { name: String::from("Bob"), id: 2 };
///
/// assert_eq!(alice1, alice2);  // Same id
/// assert_ne!(alice1, bob);     // Different id
/// ```
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub id: u64,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}

/// A status indicating the current state of an entity.
///
/// # Examples
///
/// ```
/// use partialeq_eq::Status;
///
/// assert_eq!(Status::Active, Status::Active);
/// assert_ne!(Status::Active, Status::Inactive);
/// assert_ne!(Status::Pending, Status::Active);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

/// Returns `true` if all elements in the slice are equal to each other.
///
/// An empty slice returns `true` (vacuously true - no unequal elements exist).
/// A single-element slice also returns `true`.
///
/// # Examples
///
/// ```
/// use partialeq_eq::are_all_equal;
///
/// assert!(are_all_equal(&[1, 1, 1, 1]));
/// assert!(are_all_equal(&[5]));
/// assert!(are_all_equal::<i32>(&[]));
/// assert!(!are_all_equal(&[1, 2, 1, 1]));
/// ```
pub fn are_all_equal<T: Eq>(items: &[T]) -> bool {
    match items.first() {
        None => true,
        Some(first) => items.iter().all(|item| item == first),
    }
}

/// Counts how many elements in the slice equal the target value.
///
/// # Examples
///
/// ```
/// use partialeq_eq::count_matches;
///
/// assert_eq!(count_matches(&[1, 2, 1, 3, 1], &1), 3);
/// assert_eq!(count_matches(&[1, 2, 3], &5), 0);
/// assert_eq!(count_matches::<i32>(&[], &1), 0);
/// ```
pub fn count_matches<T: PartialEq>(items: &[T], target: &T) -> usize {
    items.iter().filter(|item| *item == target).count()
}

/// Returns the index of the first element that equals the target, or `None`.
///
/// # Examples
///
/// ```
/// use partialeq_eq::find_first_match;
///
/// assert_eq!(find_first_match(&[10, 20, 30, 40], &30), Some(2));
/// assert_eq!(find_first_match(&[10, 20, 30, 40], &50), None);
/// assert_eq!(find_first_match::<i32>(&[], &1), None);
/// ```
pub fn find_first_match<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    items.iter().position(|item| item == target)
}
