use std::collections::HashMap;

/// Creates a HashMap with at least the specified capacity.
///
/// The actual capacity may be larger due to implementation details,
/// but it will always be at least the requested amount.
///
/// # Arguments
///
/// * `capacity` - The minimum number of elements the map should be able to hold without reallocating
///
/// # Returns
///
/// An empty HashMap with at least the specified capacity
///
/// # Examples
///
/// ```
/// use hashmap_advanced::create_with_capacity;
///
/// let map = create_with_capacity(100);
/// assert!(map.capacity() >= 100);
/// assert_eq!(map.len(), 0);
/// ```
pub fn create_with_capacity(capacity: usize) -> HashMap<String, i32> {
    HashMap::with_capacity(capacity)
}

/// Reserves space for at least `additional` more elements.
///
/// After calling this function, the map will have capacity for at least
/// `map.len() + additional` elements without reallocating.
///
/// # Arguments
///
/// * `map` - A mutable reference to the HashMap
/// * `additional` - The number of additional elements to reserve space for
///
/// # Examples
///
/// ```
/// use hashmap_advanced::reserve_additional;
/// use std::collections::HashMap;
///
/// let mut map: HashMap<String, i32> = HashMap::new();
/// map.insert("a".to_string(), 1);
/// reserve_additional(&mut map, 100);
/// assert!(map.capacity() >= 101);
/// ```
pub fn reserve_additional(map: &mut HashMap<String, i32>, additional: usize) {
    map.reserve(additional);
}

/// Shrinks the map's capacity to fit its current number of elements.
///
/// The capacity will be reduced to be as close to the length as possible,
/// though the exact capacity after shrinking depends on the implementation.
///
/// # Arguments
///
/// * `map` - A mutable reference to the HashMap
///
/// # Examples
///
/// ```
/// use hashmap_advanced::shrink_map;
/// use std::collections::HashMap;
///
/// let mut map: HashMap<String, i32> = HashMap::with_capacity(1000);
/// map.insert("a".to_string(), 1);
/// map.insert("b".to_string(), 2);
/// let old_capacity = map.capacity();
/// shrink_map(&mut map);
/// assert!(map.capacity() <= old_capacity);
/// ```
pub fn shrink_map(map: &mut HashMap<String, i32>) {
    map.shrink_to_fit();
}

/// Efficiently inserts multiple items by pre-allocating capacity.
///
/// This function creates a new HashMap with capacity for all items,
/// then inserts each item, avoiding reallocations during insertion.
///
/// # Arguments
///
/// * `items` - A slice of key-value pairs to insert
///
/// # Returns
///
/// A HashMap containing all the inserted items
///
/// # Examples
///
/// ```
/// use hashmap_advanced::bulk_insert;
///
/// let items = [("a", 1), ("b", 2), ("c", 3)];
/// let map = bulk_insert(&items);
/// assert_eq!(map.len(), 3);
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map["c"], 3);
/// ```
pub fn bulk_insert(items: &[(&str, i32)]) -> HashMap<String, i32> {
    let mut map = HashMap::with_capacity(items.len());
    for (key, value) in items {
        map.insert(key.to_string(), *value);
    }
    map
}

/// Returns the length and capacity of a HashMap.
///
/// # Arguments
///
/// * `map` - A reference to the HashMap
///
/// # Returns
///
/// A tuple of (length, capacity)
///
/// # Examples
///
/// ```
/// use hashmap_advanced::{create_with_capacity, get_capacity_stats};
///
/// let map = create_with_capacity(50);
/// let (len, cap) = get_capacity_stats(&map);
/// assert_eq!(len, 0);
/// assert!(cap >= 50);
/// ```
pub fn get_capacity_stats(map: &HashMap<String, i32>) -> (usize, usize) {
    (map.len(), map.capacity())
}

/// Clears all elements from the map and shrinks its capacity.
///
/// This is useful when you want to release memory after processing
/// a large number of items that are no longer needed.
///
/// # Arguments
///
/// * `map` - A mutable reference to the HashMap
///
/// # Examples
///
/// ```
/// use hashmap_advanced::{bulk_insert, clear_and_shrink, get_capacity_stats};
///
/// let mut map = bulk_insert(&[("a", 1), ("b", 2), ("c", 3)]);
/// assert_eq!(map.len(), 3);
/// clear_and_shrink(&mut map);
/// assert_eq!(map.len(), 0);
/// ```
pub fn clear_and_shrink(map: &mut HashMap<String, i32>) {
    map.clear();
    map.shrink_to_fit();
}

/// Groups items by a key function.
///
/// Each item's string key is passed to the key function to determine
/// which group it belongs to. Items with the same group key have their
/// integer values collected into a vector.
///
/// # Arguments
///
/// * `items` - A slice of (String, i32) pairs to group
/// * `key_fn` - A function that takes a string reference and returns the grouping key
///
/// # Returns
///
/// A HashMap where keys are group identifiers and values are vectors of integers
///
/// # Examples
///
/// ```
/// use hashmap_advanced::group_by_key;
///
/// let items = vec![
///     ("apple".to_string(), 1),
///     ("apricot".to_string(), 2),
///     ("banana".to_string(), 3),
/// ];
/// let grouped = group_by_key(&items, |s| s.chars().next().unwrap().to_string());
/// assert_eq!(grouped["a"], vec![1, 2]);
/// assert_eq!(grouped["b"], vec![3]);
/// ```
pub fn group_by_key<F>(items: &[(String, i32)], key_fn: F) -> HashMap<String, Vec<i32>>
where
    F: Fn(&str) -> String,
{
    let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
    for (key, value) in items {
        let group_key = key_fn(key);
        groups.entry(group_key).or_default().push(*value);
    }
    groups
}

/// Merges multiple HashMaps into one with pre-allocated capacity.
///
/// If a key appears in multiple maps, the values are summed.
/// The resulting map is pre-allocated to hold all entries efficiently.
///
/// # Arguments
///
/// * `maps` - A vector of HashMaps to merge
///
/// # Returns
///
/// A single HashMap containing all merged entries
///
/// # Examples
///
/// ```
/// use hashmap_advanced::merge_with_capacity;
/// use std::collections::HashMap;
///
/// let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
/// let map2: HashMap<String, i32> = [("a".to_string(), 2), ("b".to_string(), 3)].into();
/// let merged = merge_with_capacity(vec![map1, map2]);
/// assert_eq!(merged["a"], 3);  // 1 + 2
/// assert_eq!(merged["b"], 3);
/// ```
pub fn merge_with_capacity(maps: Vec<HashMap<String, i32>>) -> HashMap<String, i32> {
    // Calculate total capacity needed (upper bound - some keys may overlap)
    let total_capacity: usize = maps.iter().map(|m| m.len()).sum();
    let mut result = HashMap::with_capacity(total_capacity);

    for map in maps {
        for (key, value) in map {
            *result.entry(key).or_insert(0) += value;
        }
    }

    result
}
