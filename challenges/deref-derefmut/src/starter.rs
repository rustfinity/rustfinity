use std::cell::Cell;
use std::ops::{Deref, DerefMut};

/// A simple smart pointer wrapper demonstrating Deref and DerefMut.
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

// TODO: Implement Deref for MyBox<T>
// - Set Target = T
// - Return a reference to self.value

// TODO: Implement DerefMut for MyBox<T>
// - Return a mutable reference to self.value

/// A wrapper that tracks how many times the inner value has been accessed via Deref.
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

// TODO: Implement Deref for CachedValue<T>
// - Set Target = T
// - Increment the access_count using Cell::set() before returning
// - Return a reference to self.value

/// A Vec wrapper that guarantees at least one element.
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
    pub fn first_guaranteed(&self) -> &T {
        &self.inner[0]
    }

    /// Returns a mutable reference to the first element.
    pub fn first_guaranteed_mut(&mut self) -> &mut T {
        &mut self.inner[0]
    }
}

// TODO: Implement Deref for NonEmptyVec<T>
// - Set Target = [T] (a slice)
// - Return &self.inner (which coerces to &[T])

// TODO: Implement DerefMut for NonEmptyVec<T>
// - Return &mut self.inner (which coerces to &mut [T])

/// A string wrapper that always stores content in uppercase.
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

// TODO: Implement Deref for UppercaseString
// - Set Target = str
// - Return &self.inner (which coerces to &str)
// Note: Do NOT implement DerefMut - it would allow bypassing the uppercase invariant!

/// Describes the length of any type that derefs to str.
// TODO: Implement this function
// - It should accept any type T that implements Deref<Target = str>
// - Return a String like "Length: 5"
pub fn describe_length<T: Deref<Target = str>>(s: &T) -> String {
    // TODO: Use s.len() (via deref coercion) and format the result
    unimplemented!()
}

// Example usage
pub fn main() {
    // MyBox examples
    let mut b = MyBox::new(42);
    println!("MyBox value: {:?}", b);
    // After implementing Deref: println!("Dereferenced: {}", *b);

    // CachedValue examples
    let cached = CachedValue::new("hello");
    println!("Access count: {}", cached.access_count());
    // After implementing Deref: let _ = cached.len();
    // println!("Access count after len(): {}", cached.access_count());

    // NonEmptyVec examples
    let mut nev = NonEmptyVec::new(1);
    nev.push(2);
    nev.push(3);
    println!("First element: {}", nev.first_guaranteed());
    // After implementing Deref: println!("Length: {}", nev.len());

    // UppercaseString examples
    let upper = UppercaseString::new("hello world");
    println!("UppercaseString inner: {}", upper.into_inner());
    // After implementing Deref: println!("Starts with HELLO: {}", upper.starts_with("HELLO"));
}
