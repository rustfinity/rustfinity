use std::collections::HashMap;

/// Inserts a key-value pair into the hashmap or updates the value if the key exists.
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    // Your code here...
    unimplemented!()
}

/// Retrieves the value associated with a key from the hashmap.
pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    // Your code here...
    unimplemented!()
}

// Example usage
pub fn main() {
    let mut store = HashMap::new();

    // Insert a new key-value pair
    insert_or_update(&mut store, "name".to_string(), "Alice".to_string());

    // Update an existing key
    insert_or_update(&mut store, "name".to_string(), "Bob".to_string());

    // Retrieve the value by key
    let value = get_value(&store, "name".to_string());
    assert_eq!(value, Some("Bob".to_string()));

    // Try to get a non-existent key
    let missing = get_value(&store, "age".to_string());
    assert_eq!(missing, None);
}
