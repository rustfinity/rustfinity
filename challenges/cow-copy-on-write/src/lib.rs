use std::borrow::Cow;

// =============================================================================
// Part 1: Basic Cow Usage
// =============================================================================

/// Converts a string to uppercase only if it contains lowercase letters.
///
/// Returns `Cow::Borrowed` if the string is already all uppercase (or empty),
/// `Cow::Owned` if conversion was needed.
///
/// # Arguments
///
/// * `s` - The input string to potentially uppercase
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::maybe_uppercase;
/// use std::borrow::Cow;
///
/// // Already uppercase - returns borrowed
/// let result = maybe_uppercase("HELLO");
/// assert!(matches!(result, Cow::Borrowed(_)));
/// assert_eq!(result, "HELLO");
///
/// // Needs conversion - returns owned
/// let result = maybe_uppercase("Hello");
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result, "HELLO");
///
/// // Empty string - returns borrowed
/// let result = maybe_uppercase("");
/// assert!(matches!(result, Cow::Borrowed(_)));
/// ```
pub fn maybe_uppercase(s: &str) -> Cow<'_, str> {
    if s.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(s.to_uppercase())
    } else {
        Cow::Borrowed(s)
    }
}

/// Ensures a string ends with the given suffix.
///
/// Returns `Cow::Borrowed` if the suffix is already present,
/// `Cow::Owned` with suffix appended otherwise.
///
/// # Arguments
///
/// * `s` - The input string
/// * `suffix` - The suffix to ensure exists
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::ensure_suffix;
/// use std::borrow::Cow;
///
/// // Already has suffix
/// let result = ensure_suffix("file.txt", ".txt");
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Needs suffix
/// let result = ensure_suffix("file", ".txt");
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result, "file.txt");
/// ```
pub fn ensure_suffix<'a>(s: &'a str, suffix: &str) -> Cow<'a, str> {
    if s.ends_with(suffix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{}{}", s, suffix))
    }
}

/// Trims whitespace and converts to lowercase.
///
/// Returns `Cow::Borrowed` only if the string is already trimmed and lowercase,
/// `Cow::Owned` otherwise.
///
/// # Arguments
///
/// * `s` - The input string
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::trim_and_lowercase;
/// use std::borrow::Cow;
///
/// // Already trimmed and lowercase
/// let result = trim_and_lowercase("hello");
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Needs trimming
/// let result = trim_and_lowercase("  hello  ");
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result, "hello");
///
/// // Needs lowercasing
/// let result = trim_and_lowercase("HELLO");
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result, "hello");
/// ```
pub fn trim_and_lowercase(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim();

    // Check if already trimmed (no leading/trailing whitespace)
    let is_trimmed = trimmed.len() == s.len();

    // Check if already lowercase
    let is_lowercase = !trimmed.chars().any(|c| c.is_uppercase());

    if is_trimmed && is_lowercase {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(trimmed.to_lowercase())
    }
}

// =============================================================================
// Part 2: Cow with Collections
// =============================================================================

/// Removes all zeros from a slice.
///
/// Returns `Cow::Borrowed` if no zeros exist,
/// `Cow::Owned` with zeros filtered out.
///
/// # Arguments
///
/// * `values` - The input slice
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::remove_zeros;
/// use std::borrow::Cow;
///
/// // No zeros - returns borrowed
/// let data = [1, 2, 3];
/// let result = remove_zeros(&data);
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Has zeros - returns owned
/// let data = [0, 1, 0, 2, 0];
/// let result = remove_zeros(&data);
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result.as_ref(), &[1, 2]);
/// ```
pub fn remove_zeros(values: &[i32]) -> Cow<'_, [i32]> {
    if values.contains(&0) {
        Cow::Owned(values.iter().copied().filter(|&x| x != 0).collect())
    } else {
        Cow::Borrowed(values)
    }
}

/// Removes consecutive duplicates from a sorted slice.
///
/// Returns `Cow::Borrowed` if no consecutive duplicates exist,
/// `Cow::Owned` with duplicates removed.
///
/// # Arguments
///
/// * `values` - A sorted input slice
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::deduplicate_sorted;
/// use std::borrow::Cow;
///
/// // No duplicates - returns borrowed
/// let data = [1, 2, 3, 4];
/// let result = deduplicate_sorted(&data);
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Has duplicates - returns owned
/// let data = [1, 1, 2, 2, 3];
/// let result = deduplicate_sorted(&data);
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result.as_ref(), &[1, 2, 3]);
/// ```
pub fn deduplicate_sorted<T: Clone + PartialEq>(values: &[T]) -> Cow<'_, [T]> {
    // Check if there are any consecutive duplicates
    let has_duplicates = values.windows(2).any(|w| w[0] == w[1]);

    if has_duplicates {
        let mut result = Vec::with_capacity(values.len());
        for value in values {
            if result.last() != Some(value) {
                result.push(value.clone());
            }
        }
        Cow::Owned(result)
    } else {
        Cow::Borrowed(values)
    }
}

/// Clamps all values to a range [min, max].
///
/// Returns `Cow::Borrowed` if all values are already in range,
/// `Cow::Owned` with clamped values.
///
/// # Arguments
///
/// * `values` - The input slice
/// * `min` - Minimum allowed value
/// * `max` - Maximum allowed value
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::clamp_values;
/// use std::borrow::Cow;
///
/// // All in range - returns borrowed
/// let data = [5, 6, 7, 8];
/// let result = clamp_values(&data, 0, 10);
/// assert!(matches!(result, Cow::Borrowed(_)));
///
/// // Some out of range - returns owned
/// let data = [-5, 5, 15];
/// let result = clamp_values(&data, 0, 10);
/// assert!(matches!(result, Cow::Owned(_)));
/// assert_eq!(result.as_ref(), &[0, 5, 10]);
/// ```
pub fn clamp_values(values: &[i32], min: i32, max: i32) -> Cow<'_, [i32]> {
    let needs_clamping = values.iter().any(|&v| v < min || v > max);

    if needs_clamping {
        Cow::Owned(values.iter().map(|&v| v.clamp(min, max)).collect())
    } else {
        Cow::Borrowed(values)
    }
}

// =============================================================================
// Part 3: Modifying Cow with to_mut()
// =============================================================================

/// Ensures a string has at least `min_len` characters, padding with `pad_char` if needed.
///
/// Uses `to_mut()` to modify efficiently.
///
/// # Arguments
///
/// * `s` - The input Cow string
/// * `min_len` - Minimum required length
/// * `pad_char` - Character to pad with
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::ensure_capacity;
/// use std::borrow::Cow;
///
/// // Already long enough - no modification
/// let cow: Cow<str> = Cow::Borrowed("hello");
/// let result = ensure_capacity(cow, 3, '!');
/// assert_eq!(result, "hello");
///
/// // Needs padding
/// let cow: Cow<str> = Cow::Borrowed("hi");
/// let result = ensure_capacity(cow, 5, '!');
/// assert_eq!(result, "hi!!!");
///
/// // Works with owned too
/// let cow: Cow<str> = Cow::Owned(String::from("hi"));
/// let result = ensure_capacity(cow, 4, '.');
/// assert_eq!(result, "hi..");
/// ```
pub fn ensure_capacity<'a>(mut s: Cow<'a, str>, min_len: usize, pad_char: char) -> Cow<'a, str> {
    let current_len = s.chars().count();
    if current_len < min_len {
        let needed = min_len - current_len;
        let owned = s.to_mut();
        for _ in 0..needed {
            owned.push(pad_char);
        }
    }
    s
}

/// Applies a transformation function only if the predicate returns true.
///
/// Uses `to_mut()` for efficient in-place modification when already owned.
///
/// # Arguments
///
/// * `s` - The input Cow string
/// * `predicate` - Function to determine if transformation should apply
/// * `transform` - Function to apply to each character
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::modify_if_needed;
/// use std::borrow::Cow;
///
/// // Predicate false - no modification
/// let cow: Cow<str> = Cow::Borrowed("hello");
/// let result = modify_if_needed(cow, |s| s.contains('X'), |c| c.to_ascii_uppercase());
/// assert!(matches!(result, Cow::Borrowed(_)));
/// assert_eq!(result, "hello");
///
/// // Predicate true - modification applied
/// let cow: Cow<str> = Cow::Borrowed("hello");
/// let result = modify_if_needed(cow, |s| s.contains('e'), |c| c.to_ascii_uppercase());
/// assert_eq!(result, "HELLO");
/// ```
pub fn modify_if_needed<'a, P, T>(mut s: Cow<'a, str>, predicate: P, transform: T) -> Cow<'a, str>
where
    P: FnOnce(&str) -> bool,
    T: Fn(char) -> char,
{
    if predicate(&s) {
        let transformed: String = s.chars().map(transform).collect();
        *s.to_mut() = transformed;
    }
    s
}

// =============================================================================
// Part 4: Cow in Data Structures
// =============================================================================

/// A text processor that demonstrates Cow usage in a struct.
///
/// Stores text as `Cow<'a, str>`, allowing efficient borrowing when
/// no modifications are needed.
///
/// # Examples
///
/// ```
/// use cow_copy_on_write::TextProcessor;
///
/// // Create from borrowed text
/// let mut processor = TextProcessor::new("  hello   world  ");
/// assert!(processor.is_borrowed());
///
/// // Processing causes conversion to owned
/// processor.process();
/// assert!(!processor.is_borrowed());
/// assert_eq!(processor.into_string(), "hello world");
/// ```
#[derive(Debug, Clone)]
pub struct TextProcessor<'a> {
    text: Cow<'a, str>,
}

impl<'a> TextProcessor<'a> {
    /// Creates a new TextProcessor with borrowed text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to process
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::new("hello");
    /// assert!(processor.is_borrowed());
    /// ```
    pub fn new(text: &'a str) -> Self {
        TextProcessor {
            text: Cow::Borrowed(text),
        }
    }

    /// Creates a new TextProcessor with owned text.
    ///
    /// # Arguments
    ///
    /// * `text` - The owned text to process
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::from_owned(String::from("hello"));
    /// assert!(!processor.is_borrowed());
    /// ```
    pub fn from_owned(text: String) -> Self {
        TextProcessor {
            text: Cow::Owned(text),
        }
    }

    /// Processes the text by trimming and normalizing whitespace.
    ///
    /// Multiple consecutive whitespace characters are collapsed to single spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let mut processor = TextProcessor::new("  hello   world  ");
    /// processor.process();
    /// assert_eq!(processor.as_str(), "hello world");
    /// ```
    pub fn process(&mut self) {
        let trimmed = self.text.trim();

        // Check if processing is actually needed
        let needs_processing = self.text.len() != trimmed.len()
            || trimmed
                .chars()
                .zip(trimmed.chars().skip(1))
                .any(|(a, b)| a.is_whitespace() && b.is_whitespace());

        if needs_processing {
            let mut result = String::with_capacity(trimmed.len());
            let mut prev_was_whitespace = false;

            for c in trimmed.chars() {
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

            self.text = Cow::Owned(result);
        }
    }

    /// Returns a reference to the current text.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::new("hello");
    /// assert_eq!(processor.as_str(), "hello");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Converts the processor into a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::new("hello");
    /// let s = processor.into_string();
    /// assert_eq!(s, "hello");
    /// ```
    pub fn into_string(self) -> String {
        self.text.into_owned()
    }

    /// Checks if the text is currently borrowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let mut processor = TextProcessor::new("hello");
    /// assert!(processor.is_borrowed());
    ///
    /// let processor = TextProcessor::from_owned(String::from("hello"));
    /// assert!(!processor.is_borrowed());
    /// ```
    pub fn is_borrowed(&self) -> bool {
        matches!(self.text, Cow::Borrowed(_))
    }

    /// Appends text to the processor.
    ///
    /// This will convert to owned if currently borrowed.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to append
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let mut processor = TextProcessor::new("hello");
    /// assert!(processor.is_borrowed());
    /// processor.append(" world");
    /// assert!(!processor.is_borrowed());
    /// assert_eq!(processor.as_str(), "hello world");
    /// ```
    pub fn append(&mut self, text: &str) {
        self.text.to_mut().push_str(text);
    }

    /// Gets the length of the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::new("hello");
    /// assert_eq!(processor.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Checks if the text is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cow_copy_on_write::TextProcessor;
    ///
    /// let processor = TextProcessor::new("");
    /// assert!(processor.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl Default for TextProcessor<'_> {
    fn default() -> Self {
        TextProcessor {
            text: Cow::Owned(String::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_uppercase_basic() {
        let result = maybe_uppercase("HELLO");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = maybe_uppercase("Hello");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_ensure_suffix_basic() {
        let result = ensure_suffix("file.txt", ".txt");
        assert!(matches!(result, Cow::Borrowed(_)));

        let result = ensure_suffix("file", ".txt");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "file.txt");
    }

    #[test]
    fn test_remove_zeros_basic() {
        let data = [1, 2, 3];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Borrowed(_)));

        let data = [0, 1, 0, 2];
        let result = remove_zeros(&data);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result.as_ref(), &[1, 2]);
    }

    #[test]
    fn test_text_processor_basic() {
        let mut processor = TextProcessor::new("  hello   world  ");
        assert!(processor.is_borrowed());
        processor.process();
        assert!(!processor.is_borrowed());
        assert_eq!(processor.into_string(), "hello world");
    }
}
