/// Convert a string slice to an owned String.
pub fn to_owned_string(s: &str) -> String {
    // TODO: Convert the string slice to an owned String
    unimplemented!()
}

/// Count the number of Unicode characters in a string.
pub fn count_chars(s: &str) -> usize {
    // TODO: Count the number of characters
    unimplemented!()
}

/// Count the number of bytes in a string.
pub fn count_bytes(s: &str) -> usize {
    // TODO: Count the number of bytes
    unimplemented!()
}

/// Check if a string contains only ASCII characters.
pub fn is_ascii_only(s: &str) -> bool {
    // TODO: Check if the string contains only ASCII characters
    unimplemented!()
}

/// Return the first character of a string, or None if the string is empty.
pub fn first_char(s: &str) -> Option<char> {
    // TODO: Return the first character or None if empty
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
