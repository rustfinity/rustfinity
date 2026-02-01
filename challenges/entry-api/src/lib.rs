use std::collections::HashMap;

/// Counts the frequency of each word in the text.
/// Words are converted to lowercase and split on whitespace.
///
/// # Arguments
///
/// * `text` - A string slice containing words separated by whitespace
///
/// # Returns
///
/// A HashMap where keys are lowercase words and values are their frequencies
///
/// # Examples
///
/// ```
/// use entry_api::count_words;
///
/// let counts = count_words("hello world hello");
/// assert_eq!(counts["hello"], 2);
/// assert_eq!(counts["world"], 1);
/// ```
pub fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let word_lower = word.to_lowercase();
        *word_counts.entry(word_lower).or_insert(0) += 1;
    }
    word_counts
}

/// Groups words by their length.
///
/// # Arguments
///
/// * `words` - A slice of string slices to group
///
/// # Returns
///
/// A HashMap where keys are word lengths and values are vectors of words with that length
///
/// # Examples
///
/// ```
/// use entry_api::group_by_length;
///
/// let grouped = group_by_length(&["hi", "hello", "hey"]);
/// assert_eq!(grouped[&2], vec!["hi"]);
/// assert_eq!(grouped[&5], vec!["hello"]);
/// assert_eq!(grouped[&3], vec!["hey"]);
/// ```
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for word in words {
        groups
            .entry(word.len())
            .or_default()
            .push(word.to_string());
    }
    groups
}

/// Gets a cached value or computes and caches it if not present.
/// The compute function is only called if the key doesn't exist.
///
/// # Arguments
///
/// * `cache` - A mutable reference to the cache HashMap
/// * `key` - The key to look up or insert
/// * `compute` - A closure that computes the value if the key is not present
///
/// # Returns
///
/// The cached or newly computed value
///
/// # Examples
///
/// ```
/// use entry_api::get_or_compute;
/// use std::collections::HashMap;
///
/// let mut cache: HashMap<String, i32> = HashMap::new();
/// let value = get_or_compute(&mut cache, "key", || 42);
/// assert_eq!(value, 42);
/// assert_eq!(cache["key"], 42);
/// ```
pub fn get_or_compute<F>(cache: &mut HashMap<String, i32>, key: &str, compute: F) -> i32
where
    F: FnOnce() -> i32,
{
    *cache.entry(key.to_string()).or_insert_with(compute)
}

/// Increments the value at the key by 1 if it exists, otherwise inserts the initial value.
///
/// # Arguments
///
/// * `map` - A mutable reference to the HashMap
/// * `key` - The key to update or insert
/// * `init` - The initial value to insert if the key doesn't exist
///
/// # Examples
///
/// ```
/// use entry_api::increment_or_init;
/// use std::collections::HashMap;
///
/// let mut scores: HashMap<String, i32> = HashMap::new();
/// increment_or_init(&mut scores, "alice", 100);
/// assert_eq!(scores["alice"], 100);
/// increment_or_init(&mut scores, "alice", 100);
/// assert_eq!(scores["alice"], 101);
/// ```
pub fn increment_or_init(map: &mut HashMap<String, i32>, key: &str, init: i32) {
    map.entry(key.to_string())
        .and_modify(|v| *v += 1)
        .or_insert(init);
}

/// Merges two HashMaps. If a key exists in both, the values are summed.
///
/// # Arguments
///
/// * `map1` - The first HashMap
/// * `map2` - The second HashMap
///
/// # Returns
///
/// A new HashMap with merged values
///
/// # Examples
///
/// ```
/// use entry_api::merge_maps;
/// use std::collections::HashMap;
///
/// let map1: HashMap<String, i32> = [("a".to_string(), 1)].into();
/// let map2: HashMap<String, i32> = [("a".to_string(), 2)].into();
/// let merged = merge_maps(map1, map2);
/// assert_eq!(merged["a"], 3);
/// ```
pub fn merge_maps(
    map1: HashMap<String, i32>,
    map2: HashMap<String, i32>,
) -> HashMap<String, i32> {
    let mut result = map1;
    for (key, value) in map2 {
        *result.entry(key).or_insert(0) += value;
    }
    result
}

/// Creates a map where each key is an item and the value is the index of its first occurrence.
///
/// # Arguments
///
/// * `items` - A slice of string slices
///
/// # Returns
///
/// A HashMap mapping items to their first occurrence index
///
/// # Examples
///
/// ```
/// use entry_api::first_occurrence;
///
/// let firsts = first_occurrence(&["a", "b", "a"]);
/// assert_eq!(firsts["a"], 0);
/// assert_eq!(firsts["b"], 1);
/// ```
pub fn first_occurrence(items: &[&str]) -> HashMap<String, usize> {
    let mut occurrences = HashMap::new();
    for (index, item) in items.iter().enumerate() {
        occurrences.entry(item.to_string()).or_insert(index);
    }
    occurrences
}

/// Adds a value to the vector at the given key.
/// If the key doesn't exist, creates a new vector with that value.
///
/// # Arguments
///
/// * `map` - A mutable reference to the HashMap
/// * `key` - The key to update or insert
/// * `value` - The value to add to the vector
///
/// # Examples
///
/// ```
/// use entry_api::update_or_default;
/// use std::collections::HashMap;
///
/// let mut groups: HashMap<String, Vec<i32>> = HashMap::new();
/// update_or_default(&mut groups, "nums", 1);
/// update_or_default(&mut groups, "nums", 2);
/// assert_eq!(groups["nums"], vec![1, 2]);
/// ```
pub fn update_or_default(map: &mut HashMap<String, Vec<i32>>, key: &str, value: i32) {
    map.entry(key.to_string()).or_default().push(value);
}
