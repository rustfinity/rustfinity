use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// A 2D point with x and y coordinates.
///
/// TODO: Add the necessary derive macros for Hash, PartialEq, Eq, Debug, Clone, Copy
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A unique user identifier that can be used as a HashMap key.
///
/// TODO: Add the necessary derive macros for Hash, PartialEq, Eq, Debug, Clone, Copy
pub struct UserId(pub u64);

/// A string wrapper that compares and hashes case-insensitively.
///
/// Two `CaseInsensitiveString` values are equal if their lowercase forms
/// are equal. The hash is computed from the lowercase form to maintain
/// the invariant that equal values must hash the same.
#[derive(Debug, Clone)]
pub struct CaseInsensitiveString(pub String);

impl CaseInsensitiveString {
    /// Creates a new CaseInsensitiveString from any string-like type.
    pub fn new<S: Into<String>>(s: S) -> Self {
        CaseInsensitiveString(s.into())
    }

    /// Returns the original string value (preserving case).
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compare lowercase forms
        todo!()
    }
}

impl Eq for CaseInsensitiveString {}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: Hash the lowercase form to ensure equal values hash the same
        todo!()
    }
}

/// A document with an id, title, and content.
///
/// Two documents are equal if they have the same `id`, regardless of
/// title or content. The hash is computed from only the `id` field
/// to maintain consistency with equality.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u64,
    pub title: String,
    pub content: String,
}

impl Document {
    /// Creates a new Document with the given id, title, and content.
    pub fn new<S: Into<String>>(id: u64, title: S, content: S) -> Self {
        Document {
            id,
            title: title.into(),
            content: content.into(),
        }
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Compare only the id field
        todo!()
    }
}

impl Eq for Document {}

impl Hash for Document {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: Hash only the id field to match equality semantics
        todo!()
    }
}

/// An RGB color with red, green, and blue components.
///
/// TODO: Add the necessary derive macros for Hash, PartialEq, Eq, Debug, Clone, Copy
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Creates a new Rgb color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

/// Returns the count of unique elements in a slice.
///
/// Uses a `HashSet` internally to efficiently count distinct values.
pub fn count_unique<T: Hash + Eq>(items: &[T]) -> usize {
    // TODO: Collect items into a HashSet and return its length
    todo!()
}

/// Returns a vector of elements that appear more than once.
///
/// Each duplicate element appears exactly once in the result, even if
/// it appears multiple times in the input.
pub fn find_duplicates<T: Hash + Eq + Clone>(items: &[T]) -> Vec<T> {
    // TODO: Count occurrences with a HashMap, then filter for count > 1
    todo!()
}

/// Groups items by a key function.
///
/// Returns a `HashMap` where each key maps to a vector of items that
/// produced that key when passed to the key function.
pub fn group_by_hash<T: Clone, K: Hash + Eq, F: Fn(&T) -> K>(
    items: &[T],
    key_fn: F,
) -> HashMap<K, Vec<T>> {
    // TODO: Use the Entry API to group items by their key
    todo!()
}

// Example usage
pub fn main() {
    // Point with derived Hash
    let p1 = Point { x: 1, y: 2 };
    let mut point_set: HashSet<Point> = HashSet::new();
    point_set.insert(p1);
    println!("Points in set: {}", point_set.len());

    // UserId in HashMap
    let mut scores: HashMap<UserId, u32> = HashMap::new();
    scores.insert(UserId(1), 100);
    println!("Score for user 1: {:?}", scores.get(&UserId(1)));

    // CaseInsensitiveString
    let s1 = CaseInsensitiveString::new("Hello");
    let s2 = CaseInsensitiveString::new("HELLO");
    println!("Are they equal? {}", s1 == s2);

    // Document with id-only equality
    let doc1 = Document::new(1, "Draft", "Content v1");
    let doc2 = Document::new(1, "Final", "Content v2");
    println!("Same document? {}", doc1 == doc2);

    // Utility functions
    let numbers = vec![1, 2, 2, 3, 3, 3];
    println!("Unique count: {}", count_unique(&numbers));
    println!("Duplicates: {:?}", find_duplicates(&numbers));

    let grouped = group_by_hash(&numbers, |n| n % 2);
    println!("Grouped by parity: {:?}", grouped);
}
