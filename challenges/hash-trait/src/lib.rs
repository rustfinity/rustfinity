use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// A 2D point with x and y coordinates.
///
/// Uses derived `Hash`, `PartialEq`, and `Eq` for standard field-by-field
/// comparison and hashing.
///
/// # Examples
///
/// ```
/// use hash_trait::Point;
/// use std::collections::HashSet;
///
/// let p1 = Point { x: 1, y: 2 };
/// let p2 = Point { x: 1, y: 2 };
/// let mut set: HashSet<Point> = HashSet::new();
/// set.insert(p1);
/// assert!(!set.insert(p2)); // Returns false, p2 is duplicate
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A unique user identifier that can be used as a HashMap key.
///
/// # Examples
///
/// ```
/// use hash_trait::UserId;
/// use std::collections::HashMap;
///
/// let mut scores: HashMap<UserId, u32> = HashMap::new();
/// scores.insert(UserId(1), 100);
/// scores.insert(UserId(2), 200);
/// assert_eq!(scores.get(&UserId(1)), Some(&100));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

/// A string wrapper that compares and hashes case-insensitively.
///
/// Two `CaseInsensitiveString` values are equal if their lowercase forms
/// are equal. The hash is computed from the lowercase form to maintain
/// the invariant that equal values must hash the same.
///
/// # Examples
///
/// ```
/// use hash_trait::CaseInsensitiveString;
/// use std::collections::HashSet;
///
/// let s1 = CaseInsensitiveString::new("Hello");
/// let s2 = CaseInsensitiveString::new("HELLO");
/// let mut set: HashSet<CaseInsensitiveString> = HashSet::new();
/// set.insert(s1);
/// assert!(!set.insert(s2)); // Same hash due to case-insensitivity
/// ```
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
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the lowercase form to ensure equal values hash the same
        self.0.to_lowercase().hash(state);
    }
}

/// A document with an id, title, and content.
///
/// Two documents are equal if they have the same `id`, regardless of
/// title or content. The hash is computed from only the `id` field
/// to maintain consistency with equality.
///
/// # Examples
///
/// ```
/// use hash_trait::Document;
/// use std::collections::HashSet;
///
/// let doc1 = Document::new(1, "Draft", "Content v1");
/// let doc2 = Document::new(1, "Final", "Content v2");
/// let mut set: HashSet<Document> = HashSet::new();
/// set.insert(doc1);
/// assert!(!set.insert(doc2)); // Same id, same hash
/// ```
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
        self.id == other.id
    }
}

impl Eq for Document {}

impl Hash for Document {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only hash the id to match the equality semantics
        self.id.hash(state);
    }
}

/// An RGB color with red, green, and blue components.
///
/// Uses derived `Hash`, `PartialEq`, and `Eq` for standard behavior.
///
/// # Examples
///
/// ```
/// use hash_trait::Rgb;
/// use std::collections::HashSet;
///
/// let red = Rgb { r: 255, g: 0, b: 0 };
/// let also_red = Rgb { r: 255, g: 0, b: 0 };
/// let mut set: HashSet<Rgb> = HashSet::new();
/// set.insert(red);
/// assert!(!set.insert(also_red)); // Duplicate
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
///
/// # Examples
///
/// ```
/// use hash_trait::count_unique;
///
/// assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
/// assert_eq!(count_unique(&[1, 1, 1, 1]), 1);
/// assert_eq!(count_unique::<i32>(&[]), 0);
/// ```
pub fn count_unique<T: Hash + Eq>(items: &[T]) -> usize {
    items.iter().collect::<HashSet<_>>().len()
}

/// Returns a vector of elements that appear more than once.
///
/// Each duplicate element appears exactly once in the result, even if
/// it appears multiple times in the input. The order of elements in
/// the result is not guaranteed.
///
/// # Examples
///
/// ```
/// use hash_trait::find_duplicates;
///
/// let dups = find_duplicates(&[1, 2, 2, 3, 3, 3]);
/// assert_eq!(dups.len(), 2);
/// assert!(dups.contains(&2));
/// assert!(dups.contains(&3));
///
/// assert_eq!(find_duplicates(&[1, 2, 3]), Vec::<i32>::new());
/// ```
pub fn find_duplicates<T: Hash + Eq + Clone>(items: &[T]) -> Vec<T> {
    let mut counts: HashMap<&T, usize> = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(item, _)| item.clone())
        .collect()
}

/// Groups items by a key function.
///
/// Returns a `HashMap` where each key maps to a vector of items that
/// produced that key when passed to the key function.
///
/// # Examples
///
/// ```
/// use hash_trait::group_by_hash;
///
/// let words = vec!["apple", "banana", "apricot", "blueberry"];
/// let grouped = group_by_hash(&words, |w| w.chars().next().unwrap());
/// assert_eq!(grouped.get(&'a').unwrap().len(), 2);  // apple, apricot
/// assert_eq!(grouped.get(&'b').unwrap().len(), 2);  // banana, blueberry
///
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let by_parity = group_by_hash(&numbers, |n| n % 2);
/// assert_eq!(by_parity.get(&0).unwrap(), &vec![2, 4, 6]);  // even
/// assert_eq!(by_parity.get(&1).unwrap(), &vec![1, 3, 5]);  // odd
/// ```
pub fn group_by_hash<T: Clone, K: Hash + Eq, F: Fn(&T) -> K>(
    items: &[T],
    key_fn: F,
) -> HashMap<K, Vec<T>> {
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        let key = key_fn(item);
        groups.entry(key).or_default().push(item.clone());
    }
    groups
}
