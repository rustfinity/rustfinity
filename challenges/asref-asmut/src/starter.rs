use std::path::Path;

/// Returns the length of any string-like type.
///
/// # Arguments
///
/// * `s` - Any type that implements `AsRef<str>`
pub fn string_length<S: AsRef<str>>(s: S) -> usize {
    // TODO: Use as_ref() to get a &str and return its length
    todo!()
}

/// Sums all elements in any slice-like collection of i32.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsRef<[i32]>`
pub fn slice_sum<V: AsRef<[i32]>>(values: V) -> i32 {
    // TODO: Use as_ref() to get a &[i32] and sum all elements
    todo!()
}

/// Checks if a slice-like collection contains a specific element.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsRef<[i32]>`
/// * `target` - The element to search for
pub fn contains_element<V: AsRef<[i32]>>(values: V, target: i32) -> bool {
    // TODO: Use as_ref() to get a &[i32] and check if it contains target
    todo!()
}

/// Doubles every element in a mutable slice-like collection.
///
/// # Arguments
///
/// * `values` - Any type that implements `AsMut<[i32]>`
pub fn double_all<V: AsMut<[i32]>>(values: &mut V) {
    // TODO: Use as_mut() to get a &mut [i32] and double each element
    // Hint: Use iter_mut() to iterate over mutable references
    todo!()
}

/// A custom text type that wraps a String and provides multiple AsRef implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    content: String,
}

impl Text {
    /// Creates a new Text from any type that can be converted into a String.
    pub fn new(s: impl Into<String>) -> Self {
        // TODO: Create a Text with the content set to s.into()
        todo!()
    }

    /// Returns the inner string content.
    pub fn into_inner(self) -> String {
        // TODO: Return the content field
        todo!()
    }
}

// TODO: Implement AsRef<str> for Text
// impl AsRef<str> for Text {
//     fn as_ref(&self) -> &str {
//         // Return a reference to the content as &str
//     }
// }

// TODO: Implement AsRef<[u8]> for Text
// impl AsRef<[u8]> for Text {
//     fn as_ref(&self) -> &[u8] {
//         // Return the bytes of the content
//         // Hint: String has an as_bytes() method
//     }
// }

// TODO: Implement AsMut<String> for Text
// impl AsMut<String> for Text {
//     fn as_mut(&mut self) -> &mut String {
//         // Return a mutable reference to the content
//     }
// }

/// Converts any string-like type to uppercase and returns it.
///
/// # Arguments
///
/// * `s` - Any type that implements `AsRef<str>`
pub fn print_as_uppercase<S: AsRef<str>>(s: S) -> String {
    // TODO: Use as_ref() to get a &str and call to_uppercase()
    todo!()
}

/// Appends a value to any mutable Vec-like collection.
///
/// # Arguments
///
/// * `buffer` - Any type that implements `AsMut<Vec<i32>>`
/// * `value` - The value to append
pub fn append_value<V: AsMut<Vec<i32>>>(buffer: &mut V, value: i32) {
    // TODO: Use as_mut() to get a &mut Vec<i32> and push the value
    todo!()
}

/// Extracts the file extension from any path-like type.
///
/// # Arguments
///
/// * `path` - Any type that implements `AsRef<Path>`
pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    // TODO: Use as_ref() to get a &Path
    // Then use extension() to get Option<&OsStr>
    // Convert to Option<String> using and_then() or map()
    // Hint: OsStr has a to_str() method that returns Option<&str>
    todo!()
}

// Example usage
pub fn main() {
    // String length works with &str and String
    println!("Length of 'hello': {}", string_length("hello"));
    println!("Length of String: {}", string_length(String::from("world")));

    // Slice sum works with arrays, slices, and vectors
    println!("Sum of array: {}", slice_sum(&[1, 2, 3]));
    println!("Sum of vec: {}", slice_sum(vec![10, 20]));

    // Contains element
    println!("Contains 2: {}", contains_element(&[1, 2, 3], 2));

    // Double all elements
    let mut arr = [1, 2, 3];
    double_all(&mut arr);
    println!("Doubled: {:?}", arr);

    // Text type
    let text = Text::new("Hello");
    println!("Text content: {}", text.into_inner());

    // Uppercase
    println!("Uppercase: {}", print_as_uppercase("hello world"));

    // Append value
    let mut vec = vec![1, 2];
    append_value(&mut vec, 3);
    println!("After append: {:?}", vec);

    // Path extension
    println!("Extension: {:?}", get_extension("file.txt"));
}
