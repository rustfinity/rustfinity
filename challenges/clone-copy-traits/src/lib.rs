/// A color represented by RGB components.
///
/// This type implements `Copy` because all its fields are primitive `u8` values.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::Color;
///
/// let color1 = Color { r: 255, g: 128, b: 0 };
/// let color2 = color1;  // Copy - color1 is still valid
/// assert_eq!(color1.r, 255);
/// assert_eq!(color2.r, 255);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Dimensions represented by width and height.
///
/// This type implements `Copy` because all its fields are primitive `f64` values.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::Dimensions;
///
/// let dims = Dimensions { width: 100.0, height: 50.0 };
/// let dims2 = dims;  // Copy - dims is still valid
/// assert_eq!(dims.width, 100.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

/// A text label.
///
/// This type implements `Clone` but NOT `Copy` because `String` is not `Copy`.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::Label;
///
/// let label1 = Label { text: String::from("Hello") };
/// let label2 = label1.clone();  // Must explicitly clone
/// assert_eq!(label1.text, "Hello");
/// assert_eq!(label2.text, "Hello");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub text: String,
}

/// A document with a title and pages.
///
/// This type implements `Clone` but NOT `Copy` because it contains
/// heap-allocated data (`String` and `Vec<String>`).
///
/// # Examples
///
/// ```
/// use clone_copy_traits::Document;
///
/// let doc = Document {
///     title: String::from("My Book"),
///     pages: vec![String::from("Chapter 1"), String::from("Chapter 2")],
/// };
/// let doc_copy = doc.clone();
/// assert_eq!(doc.title, doc_copy.title);
/// assert_eq!(doc.pages.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub title: String,
    pub pages: Vec<String>,
}

/// A generic tagged value.
///
/// This type implements `Clone` when `T` implements `Clone`, but cannot
/// implement `Copy` because it contains a `String`.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::TaggedValue;
///
/// let tagged = TaggedValue {
///     tag: String::from("count"),
///     value: 42,
/// };
/// let cloned = tagged.clone();
/// assert_eq!(cloned.tag, "count");
/// assert_eq!(cloned.value, 42);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TaggedValue<T> {
    pub tag: String,
    pub value: T,
}

/// Duplicates a `Copy` type by returning two copies.
///
/// Since `T` implements `Copy`, the value is copied rather than moved,
/// allowing us to return two independent copies.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::{duplicate_copy, Color};
///
/// let color = Color { r: 100, g: 150, b: 200 };
/// let (c1, c2) = duplicate_copy(color);
/// assert_eq!(c1, c2);
/// assert_eq!(color.r, 100);  // Original still valid due to Copy
/// ```
pub fn duplicate_copy<T: Copy>(value: T) -> (T, T) {
    (value, value)
}

/// Creates a clone of a referenced value.
///
/// Takes a reference to avoid moving the original value and returns
/// a cloned copy.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::{duplicate_clone, Label};
///
/// let label = Label { text: String::from("test") };
/// let cloned = duplicate_clone(&label);
/// assert_eq!(label.text, cloned.text);
/// ```
pub fn duplicate_clone<T: Clone>(value: &T) -> T {
    value.clone()
}

/// Clones all items in a slice into a new Vec.
///
/// # Examples
///
/// ```
/// use clone_copy_traits::clone_vec;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let cloned = clone_vec(&numbers);
/// assert_eq!(cloned, vec![1, 2, 3, 4, 5]);
///
/// let strings = vec![String::from("a"), String::from("b")];
/// let cloned_strings = clone_vec(&strings);
/// assert_eq!(cloned_strings, strings);
/// ```
pub fn clone_vec<T: Clone>(items: &[T]) -> Vec<T> {
    items.to_vec()
}
