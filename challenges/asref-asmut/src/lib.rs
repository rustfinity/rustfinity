use std::path::Path;

/// Returns the length of any string-like type.
///
/// # Arguments
///
/// * `s` - Any type that implements `AsRef<str>`
///
/// # Examples
///
/// ```
/// use asref_asmut::string_length;
///
/// assert_eq!(string_length("hello"), 5);
/// assert_eq!(string_length(String::from("world")), 5);
/// ```
pub fn string_length<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

/// Sums all elements in any slice-like collection of i32.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsRef<[i32]>`
///
/// # Examples
///
/// ```
/// use asref_asmut::slice_sum;
///
/// assert_eq!(slice_sum(&[1, 2, 3, 4]), 10);
/// assert_eq!(slice_sum(vec![10, 20, 30]), 60);
/// ```
pub fn slice_sum<V: AsRef<[i32]>>(values: V) -> i32 {
    values.as_ref().iter().sum()
}

/// Checks if a slice-like collection contains a specific element.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsRef<[i32]>`
/// * `target` - The element to search for
///
/// # Examples
///
/// ```
/// use asref_asmut::contains_element;
///
/// assert!(contains_element(&[1, 2, 3], 2));
/// assert!(!contains_element(vec![1, 2, 3], 5));
/// ```
pub fn contains_element<V: AsRef<[i32]>>(values: V, target: i32) -> bool {
    values.as_ref().contains(&target)
}

/// Doubles every element in a mutable slice-like collection.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsMut<[i32]>`
///
/// # Examples
///
/// ```
/// use asref_asmut::double_all;
///
/// let mut arr = [1, 2, 3];
/// double_all(&mut arr);
/// assert_eq!(arr, [2, 4, 6]);
/// ```
pub fn double_all<V: AsMut<[i32]>>(values: &mut V) {
    for elem in values.as_mut().iter_mut() {
        *elem *= 2;
    }
}

/// A custom text type that wraps a String and provides multiple AsRef implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    content: String,
}

impl Text {
    /// Creates a new Text from any type that can be converted into a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use asref_asmut::Text;
    ///
    /// let text = Text::new("hello");
    /// let text2 = Text::new(String::from("world"));
    /// ```
    pub fn new(s: impl Into<String>) -> Self {
        Text {
            content: s.into(),
        }
    }

    /// Returns the inner string content.
    pub fn into_inner(self) -> String {
        self.content
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.content
    }
}

impl AsRef<[u8]> for Text {
    fn as_ref(&self) -> &[u8] {
        self.content.as_bytes()
    }
}

impl AsMut<String> for Text {
    fn as_mut(&mut self) -> &mut String {
        &mut self.content
    }
}

/// Converts any string-like type to uppercase and returns it.
///
/// # Arguments
///
/// * `s` - Any type that implements `AsRef<str>`
///
/// # Examples
///
/// ```
/// use asref_asmut::print_as_uppercase;
///
/// assert_eq!(print_as_uppercase("hello"), "HELLO");
/// assert_eq!(print_as_uppercase(String::from("World")), "WORLD");
/// ```
pub fn print_as_uppercase<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_uppercase()
}

/// Appends a value to any mutable Vec-like collection.
///
/// # Arguments
///
/// * `buffer` - Any type that implements `AsMut<Vec<i32>>`
/// * `value` - The value to append
///
/// # Examples
///
/// ```
/// use asref_asmut::append_value;
///
/// let mut vec = vec![1, 2];
/// append_value(&mut vec, 3);
/// assert_eq!(vec, vec![1, 2, 3]);
/// ```
pub fn append_value<V: AsMut<Vec<i32>>>(buffer: &mut V, value: i32) {
    buffer.as_mut().push(value);
}

/// Extracts the file extension from any path-like type.
///
/// # Arguments
///
/// * `path` - Any type that implements `AsRef<Path>`
///
/// # Examples
///
/// ```
/// use asref_asmut::get_extension;
///
/// assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
/// assert_eq!(get_extension("no_extension"), None);
/// ```
pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}
