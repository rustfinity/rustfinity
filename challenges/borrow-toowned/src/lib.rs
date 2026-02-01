use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Looks up a value in a HashMap using any type that can be borrowed as the key type.
///
/// This demonstrates the power of `Borrow` - you can look up a `HashMap<String, V>`
/// using a `&str` key without needing to allocate a new `String`.
///
/// # Arguments
///
/// * `map` - A reference to the HashMap to search
/// * `key` - Any type that the key type can borrow as
///
/// # Examples
///
/// ```
/// use borrow_toowned::lookup;
/// use std::collections::HashMap;
///
/// let mut map: HashMap<String, i32> = HashMap::new();
/// map.insert("hello".to_string(), 42);
///
/// // Can look up with &str even though keys are String
/// assert_eq!(lookup(&map, "hello"), Some(&42));
/// assert_eq!(lookup(&map, "world"), None);
/// ```
pub fn lookup<'a, K, V, Q>(map: &'a HashMap<K, V>, key: &Q) -> Option<&'a V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
{
    map.get(key)
}

/// Converts a borrowed string slice to an owned String using ToOwned.
///
/// # Arguments
///
/// * `s` - A string slice to convert
///
/// # Examples
///
/// ```
/// use borrow_toowned::make_owned;
///
/// let borrowed = "hello";
/// let owned = make_owned(borrowed);
/// assert_eq!(owned, String::from("hello"));
/// ```
pub fn make_owned(s: &str) -> String {
    s.to_owned()
}

/// Normalizes multiple consecutive whitespace characters to single spaces.
///
/// Uses `Cow` to avoid allocation when no changes are needed.
///
/// # Arguments
///
/// * `input` - The string to normalize
///
/// # Examples
///
/// ```
/// use borrow_toowned::normalize_whitespace;
/// use std::borrow::Cow;
///
/// // No change needed - returns borrowed
/// let result = normalize_whitespace("hello world");
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Change needed - returns owned
/// let result = normalize_whitespace("hello   world");
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result, "hello world");
/// ```
pub fn normalize_whitespace(input: &str) -> Cow<'_, str> {
    // Check if normalization is needed
    let needs_normalization = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(a, b)| a.is_whitespace() && b.is_whitespace());

    if !needs_normalization {
        Cow::Borrowed(input)
    } else {
        let mut result = String::with_capacity(input.len());
        let mut prev_was_whitespace = false;

        for c in input.chars() {
            if c.is_whitespace() {
                if !prev_was_whitespace {
                    result.push(' ');
                }
                prev_was_whitespace = true;
            } else {
                result.push(c);
                prev_was_whitespace = false;
            }
        }

        Cow::Owned(result)
    }
}

/// Ensures a string starts with the given prefix.
///
/// Uses `Cow` to return borrowed data when the prefix already exists,
/// avoiding unnecessary allocation.
///
/// # Arguments
///
/// * `s` - The string to check/modify
/// * `prefix` - The prefix to ensure exists
///
/// # Examples
///
/// ```
/// use borrow_toowned::ensure_prefix;
/// use std::borrow::Cow;
///
/// // Already has prefix - returns borrowed
/// let result = ensure_prefix("hello_world", "hello_");
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Needs prefix - returns owned
/// let result = ensure_prefix("world", "hello_");
/// assert_eq!(result, "hello_world");
/// ```
pub fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    if s.starts_with(prefix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}{}", prefix, s))
    }
}

/// A case-insensitive string type for use in collections.
///
/// This type implements `Borrow<str>`, `Hash`, and `Eq` such that
/// strings differing only in case are considered equal and hash the same.
///
/// # Examples
///
/// ```
/// use borrow_toowned::CaseInsensitiveString;
/// use std::collections::HashMap;
///
/// let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
/// map.insert(CaseInsensitiveString::new("Hello"), 1);
///
/// // Can look up with any case
/// assert_eq!(map.get(&CaseInsensitiveString::new("HELLO")), Some(&1));
/// assert_eq!(map.get(&CaseInsensitiveString::new("hello")), Some(&1));
/// ```
#[derive(Debug, Clone)]
pub struct CaseInsensitiveString {
    original: String,
    lowercase: String,
}

impl CaseInsensitiveString {
    /// Creates a new CaseInsensitiveString from any type convertible to String.
    pub fn new(s: impl Into<String>) -> Self {
        let original = s.into();
        let lowercase = original.to_lowercase();
        CaseInsensitiveString {
            original,
            lowercase,
        }
    }

    /// Returns a reference to the original string.
    pub fn as_str(&self) -> &str {
        &self.original
    }
}

impl Borrow<str> for CaseInsensitiveString {
    fn borrow(&self) -> &str {
        // We return the lowercase version for consistent hashing and comparison
        // when used as a key in collections
        &self.lowercase
    }
}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lowercase.hash(state);
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.lowercase == other.lowercase
    }
}

impl Eq for CaseInsensitiveString {}

/// Appends an element to a slice only if it's not already present.
///
/// Uses `Cow` to avoid cloning when the element already exists.
///
/// # Arguments
///
/// * `items` - The slice to potentially append to
/// * `value` - The value to append if missing
///
/// # Examples
///
/// ```
/// use borrow_toowned::append_if_missing;
/// use std::borrow::Cow;
///
/// let items = [1, 2, 3];
///
/// // Element exists - returns borrowed
/// let result = append_if_missing(&items, 2);
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Element missing - returns owned with new element
/// let result = append_if_missing(&items, 4);
/// assert_eq!(result.as_ref(), &[1, 2, 3, 4]);
/// ```
pub fn append_if_missing<'a, T>(items: &'a [T], value: T) -> Cow<'a, [T]>
where
    T: Clone + PartialEq,
{
    if items.contains(&value) {
        Cow::Borrowed(items)
    } else {
        let mut vec = items.to_vec();
        vec.push(value);
        Cow::Owned(vec)
    }
}

/// Normalizes a path by removing redundant "./" components.
///
/// Uses `Cow` for efficiency - returns borrowed data when no changes needed.
///
/// # Arguments
///
/// * `path` - The path string to normalize
///
/// # Examples
///
/// ```
/// use borrow_toowned::normalize_path;
/// use std::borrow::Cow;
///
/// // Needs normalization
/// assert_eq!(normalize_path("./foo/./bar"), "foo/bar");
///
/// // No normalization needed - returns borrowed
/// let result = normalize_path("foo/bar");
/// assert!(matches!(result, Cow::Borrowed(_)));
/// ```
pub fn normalize_path(path: &str) -> Cow<'_, str> {
    if !path.contains("./") {
        return Cow::Borrowed(path);
    }

    let mut result = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    let mut last_was_slash = false;
    let mut at_start = true;

    while let Some(c) = chars.next() {
        if c == '.' {
            // Check if this is "./"
            if chars.peek() == Some(&'/') {
                // Skip the dot, the slash will be handled next iteration
                // But if we're at start or after a slash, skip both . and /
                if at_start || last_was_slash {
                    chars.next(); // consume the /
                    continue;
                }
            }
            result.push(c);
            last_was_slash = false;
            at_start = false;
        } else if c == '/' {
            // Avoid double slashes
            if !last_was_slash && !result.is_empty() {
                result.push(c);
            } else if result.is_empty() && !at_start {
                result.push(c);
            }
            last_was_slash = true;
            at_start = false;
        } else {
            result.push(c);
            last_was_slash = false;
            at_start = false;
        }
    }

    // Remove trailing slash if present (unless it's the only character)
    if result.ends_with('/') && result.len() > 1 {
        result.pop();
    }

    Cow::Owned(result)
}

/// Converts a borrowed slice to an owned vector using ToOwned.
///
/// # Arguments
///
/// * `slice` - A slice to convert to an owned vector
///
/// # Examples
///
/// ```
/// use borrow_toowned::to_owned_vec;
///
/// let slice = &[1, 2, 3];
/// let vec = to_owned_vec(slice);
/// assert_eq!(vec, vec![1, 2, 3]);
/// ```
pub fn to_owned_vec<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.to_owned()
}
