/// Convert a string slice to an owned String.
pub fn to_owned_string(s: &str) -> String {
    // TODO: Convert the string slice to an owned String
    // Hint: Use .to_string() or String::from()
    unimplemented!()
}

/// Count the number of Unicode characters in a string.
pub fn count_chars(s: &str) -> usize {
    // TODO: Count the number of characters
    // Hint: Use .chars().count()
    unimplemented!()
}

/// Count the number of bytes in a string.
pub fn count_bytes(s: &str) -> usize {
    // TODO: Count the number of bytes
    // Hint: Use .len() or .bytes().count()
    unimplemented!()
}

/// Check if a string contains only ASCII characters.
pub fn is_ascii_only(s: &str) -> bool {
    // TODO: Check if the string contains only ASCII characters
    // Hint: Use .is_ascii()
    unimplemented!()
}

/// Return the first character of a string, or None if the string is empty.
pub fn first_char(s: &str) -> Option<char> {
    // TODO: Return the first character or None if empty
    // Hint: Use .chars().next()
    unimplemented!()
}

pub fn main() {
    let greeting = "Hello, world!";

    println!("Original: {}", greeting);
    println!("As owned String: {}", to_owned_string(greeting));
    println!("Character count: {}", count_chars(greeting));
    println!("Byte count: {}", count_bytes(greeting));
    println!("Is ASCII only: {}", is_ascii_only(greeting));
    println!("First character: {:?}", first_char(greeting));
}
