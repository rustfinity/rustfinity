use std::collections::BTreeMap;

/// Creates a BTreeMap from a slice of key-value pairs.
///
/// The BTreeMap will automatically sort entries by key.
///
/// # Arguments
///
/// * `pairs` - A slice of (String, i32) tuples to insert into the map
///
/// # Returns
///
/// A BTreeMap containing all the key-value pairs
///
/// # Examples
///
/// ```
/// use btreemap_basics::create_sorted_map;
///
/// let pairs = vec![
///     ("cherry".to_string(), 3),
///     ("apple".to_string(), 1),
/// ];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(map.get("apple"), Some(&1));
/// ```
pub fn create_sorted_map(pairs: &[(String, i32)]) -> BTreeMap<String, i32> {
    pairs.iter().cloned().collect()
}

/// Gets a value from the map by key.
///
/// # Arguments
///
/// * `map` - The BTreeMap to search
/// * `key` - The key to look up
///
/// # Returns
///
/// `Some(value)` if the key exists, `None` otherwise
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_value};
///
/// let pairs = vec![("apple".to_string(), 1)];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(get_value(&map, "apple"), Some(1));
/// assert_eq!(get_value(&map, "banana"), None);
/// ```
pub fn get_value(map: &BTreeMap<String, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()
}

/// Returns all keys from the map in sorted order.
///
/// # Arguments
///
/// * `map` - The BTreeMap to get keys from
///
/// # Returns
///
/// A Vec containing all keys in sorted order
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_keys_in_order};
///
/// let pairs = vec![
///     ("cherry".to_string(), 3),
///     ("apple".to_string(), 1),
/// ];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(get_keys_in_order(&map), vec!["apple", "cherry"]);
/// ```
pub fn get_keys_in_order(map: &BTreeMap<String, i32>) -> Vec<String> {
    map.keys().cloned().collect()
}

/// Returns all values from the map in the order of their sorted keys.
///
/// # Arguments
///
/// * `map` - The BTreeMap to get values from
///
/// # Returns
///
/// A Vec containing all values in key-sorted order
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_values_in_key_order};
///
/// let pairs = vec![
///     ("cherry".to_string(), 3),
///     ("apple".to_string(), 1),
/// ];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(get_values_in_key_order(&map), vec![1, 3]);
/// ```
pub fn get_values_in_key_order(map: &BTreeMap<String, i32>) -> Vec<i32> {
    map.values().copied().collect()
}

/// Returns all key-value pairs where the key is in the range [start, end).
///
/// This is a half-open range: includes start, excludes end.
///
/// # Arguments
///
/// * `map` - The BTreeMap to query
/// * `start` - The inclusive start of the range
/// * `end` - The exclusive end of the range
///
/// # Returns
///
/// A Vec of (key, value) pairs within the range
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_range};
///
/// let pairs = vec![
///     ("apple".to_string(), 1),
///     ("banana".to_string(), 2),
///     ("cherry".to_string(), 3),
/// ];
/// let map = create_sorted_map(&pairs);
/// let range = get_range(&map, "apple", "cherry");
/// assert_eq!(range, vec![
///     ("apple".to_string(), 1),
///     ("banana".to_string(), 2),
/// ]);
/// ```
pub fn get_range(map: &BTreeMap<String, i32>, start: &str, end: &str) -> Vec<(String, i32)> {
    map.range(start.to_string()..end.to_string())
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/// Returns the first (smallest key) entry in the map.
///
/// # Arguments
///
/// * `map` - The BTreeMap to query
///
/// # Returns
///
/// `Some((key, value))` for the smallest key, or `None` if the map is empty
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_first};
///
/// let pairs = vec![
///     ("cherry".to_string(), 3),
///     ("apple".to_string(), 1),
/// ];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(get_first(&map), Some(("apple".to_string(), 1)));
/// ```
pub fn get_first(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    map.first_key_value().map(|(k, v)| (k.clone(), *v))
}

/// Returns the last (largest key) entry in the map.
///
/// # Arguments
///
/// * `map` - The BTreeMap to query
///
/// # Returns
///
/// `Some((key, value))` for the largest key, or `None` if the map is empty
///
/// # Examples
///
/// ```
/// use btreemap_basics::{create_sorted_map, get_last};
///
/// let pairs = vec![
///     ("cherry".to_string(), 3),
///     ("apple".to_string(), 1),
/// ];
/// let map = create_sorted_map(&pairs);
/// assert_eq!(get_last(&map), Some(("cherry".to_string(), 3)));
/// ```
pub fn get_last(map: &BTreeMap<String, i32>) -> Option<(String, i32)> {
    map.last_key_value().map(|(k, v)| (k.clone(), *v))
}
