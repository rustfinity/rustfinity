use std::cell::Cell;
use std::ops::{Deref, DerefMut};

/// A simple smart pointer wrapper demonstrating Deref and DerefMut.
///
/// # Examples
///
/// ```
/// use deref_derefmut::MyBox;
///
/// let mut b = MyBox::new(42);
/// assert_eq!(*b, 42);
/// *b = 100;
/// assert_eq!(*b, 100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    /// Creates a new MyBox containing the given value.
    pub fn new(value: T) -> Self {
        MyBox { value }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// A wrapper that tracks how many times the inner value has been accessed via Deref.
///
/// Uses interior mutability (Cell) to track accesses since Deref takes &self.
///
/// # Examples
///
/// ```
/// use deref_derefmut::CachedValue;
///
/// let cached = CachedValue::new("hello");
/// assert_eq!(cached.access_count(), 0);
///
/// // Accessing via deref increments the counter
/// let len = cached.len();
/// assert_eq!(len, 5);
/// assert_eq!(cached.access_count(), 1);
///
/// // Each deref increments the counter
/// let _ = cached.is_empty();
/// assert_eq!(cached.access_count(), 2);
/// ```
#[derive(Debug)]
pub struct CachedValue<T> {
    value: T,
    access_count: Cell<usize>,
}

impl<T> CachedValue<T> {
    /// Creates a new CachedValue with zero access count.
    pub fn new(value: T) -> Self {
        CachedValue {
            value,
            access_count: Cell::new(0),
        }
    }

    /// Returns the number of times the value has been accessed via Deref.
    pub fn access_count(&self) -> usize {
        self.access_count.get()
    }
}

impl<T> Deref for CachedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &self.value
    }
}

/// A Vec wrapper that guarantees at least one element.
///
/// Derefs to a slice, allowing all slice operations while maintaining the
/// non-empty invariant through the type system.
///
/// # Examples
///
/// ```
/// use deref_derefmut::NonEmptyVec;
///
/// let mut nev = NonEmptyVec::new(1);
/// nev.push(2);
/// nev.push(3);
///
/// // Deref to &[T] for slice operations
/// assert_eq!(nev.len(), 3);
/// assert_eq!(nev.first(), Some(&1));
/// assert_eq!(nev[0], 1);
///
/// // DerefMut for mutable operations
/// nev[0] = 10;
/// assert_eq!(nev[0], 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    /// Creates a new NonEmptyVec with the given first element.
    pub fn new(first: T) -> Self {
        NonEmptyVec { inner: vec![first] }
    }

    /// Adds an element to the end.
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Returns a reference to the first element.
    /// This is guaranteed to succeed since the vec is never empty.
    pub fn first_guaranteed(&self) -> &T {
        // Safe because NonEmptyVec always has at least one element
        &self.inner[0]
    }

    /// Returns a mutable reference to the first element.
    pub fn first_guaranteed_mut(&mut self) -> &mut T {
        &mut self.inner[0]
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A string wrapper that always stores content in uppercase.
///
/// Implements Deref to &str but NOT DerefMut, preventing users from
/// bypassing the uppercase invariant.
///
/// # Examples
///
/// ```
/// use deref_derefmut::UppercaseString;
///
/// let upper = UppercaseString::new("Hello World");
/// assert_eq!(&*upper, "HELLO WORLD");
///
/// // Deref coercion allows using str methods
/// assert!(upper.starts_with("HELLO"));
/// assert_eq!(upper.len(), 11);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UppercaseString {
    inner: String,
}

impl UppercaseString {
    /// Creates a new UppercaseString, converting the input to uppercase.
    pub fn new(s: &str) -> Self {
        UppercaseString {
            inner: s.to_uppercase(),
        }
    }

    /// Returns the inner string, consuming self.
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Deref for UppercaseString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Note: We intentionally do NOT implement DerefMut for UppercaseString
// because that would allow users to bypass the uppercase invariant.

/// Describes the length of any type that derefs to str.
///
/// This function demonstrates deref coercion - it works with &String,
/// &Box<str>, &UppercaseString, or any other type that implements
/// Deref<Target = str>.
///
/// # Examples
///
/// ```
/// use deref_derefmut::{describe_length, UppercaseString};
///
/// let s = String::from("hello");
/// assert_eq!(describe_length(&s), "Length: 5");
///
/// let upper = UppercaseString::new("world");
/// assert_eq!(describe_length(&upper), "Length: 5");
/// ```
pub fn describe_length<T: Deref<Target = str>>(s: &T) -> String {
    format!("Length: {}", s.len())
}
