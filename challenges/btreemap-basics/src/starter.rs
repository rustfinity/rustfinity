use std::collections::BTreeMap;

/// Creates a BTreeMap from a slice of key-value pairs.
///
/// The BTreeMap will automatically sort entries by key.
pub fn create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32> {
    // TODO: Collect pairs into a BTreeMap
    unimplemented!()
}

/// Gets a value from the map by key.
///
/// Returns `Some(value)` if the key exists, `None` otherwise.
pub fn get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32> {
    // TODO: Look up the key
    unimplemented!()
}

/// Returns all keys from the map in sorted order.
pub fn get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String> {
    // TODO: Get all keys
    unimplemented!()
}

/// Returns all values from the map in the order of their sorted keys.
pub fn get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32> {
    // TODO: Get all values
    unimplemented!()
}

/// Returns all key-value pairs where the key is in the range [start, end).
///
/// This is a half-open range: includes start, excludes end.
pub fn get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)> {
    // TODO: Get entries in the range
    unimplemented!()
}

/// Returns the first (smallest key) entry in the map.
pub fn get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Get the first entry
    unimplemented!()
}

/// Returns the last (largest key) entry in the map.
pub fn get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Get the last entry
    unimplemented!()
}

// Example usage
pub fn main() {
    let pairs = vec![
        ("cherry".to_string(), 3),
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
    ];

    let map = create_sorted_map(&pairs);
    println!("Map: {:?}", map);

    println!("Value for 'apple': {:?}", get_value(&map, "apple"));
    println!("Keys in order: {:?}", get_keys_in_order(&map));
    println!("Values in key order: {:?}", get_values_in_key_order(&map));

    let range = get_range(&map, "apple", "cherry");
    println!("Range [apple, cherry): {:?}", range);

    println!("First entry: {:?}", get_first(&map));
    println!("Last entry: {:?}", get_last(&map));
}
