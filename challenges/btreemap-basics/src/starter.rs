use std::collections::BTreeMap;

/// Creates a BTreeMap from a slice of key-value pairs.
///
/// The BTreeMap will automatically sort entries by key.
pub fn create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32> {
    // TODO: Iterate over pairs and collect into a BTreeMap
    unimplemented!()
}

/// Gets a value from the map by key.
///
/// Returns `Some(value)` if the key exists, `None` otherwise.
pub fn get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32> {
    // TODO: Use the .get() method to look up the key
    // Remember to handle the Option<&i32> -> Option<i32> conversion
    unimplemented!()
}

/// Returns all keys from the map in sorted order.
pub fn get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String> {
    // TODO: Use the .keys() method to get an iterator over keys
    unimplemented!()
}

/// Returns all values from the map in the order of their sorted keys.
pub fn get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32> {
    // TODO: Use the .values() method to get an iterator over values
    unimplemented!()
}

/// Returns all key-value pairs where the key is in the range [start, end).
///
/// This is a half-open range: includes start, excludes end.
pub fn get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)> {
    // TODO: Use the .range() method with a range pattern
    // Remember that range returns an iterator of references
    unimplemented!()
}

/// Returns the first (smallest key) entry in the map.
pub fn get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Use the .first_key_value() method
    // Handle the Option<(&K, &V)> -> Option<(K, V)> conversion
    unimplemented!()
}

/// Returns the last (largest key) entry in the map.
pub fn get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    // TODO: Use the .last_key_value() method
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
