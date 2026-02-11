use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// Converts a Vec to a HashSet, removing duplicate elements.
///
/// # Arguments
///
/// * `vec` - A vector of elements
///
/// # Returns
///
/// A HashSet containing the unique elements from the vector
///
/// # Examples
///
/// ```
/// use collection_conversions::vec_to_hashset;
///
/// let set = vec_to_hashset(vec![1, 2, 2, 3, 3, 3]);
/// assert_eq!(set.len(), 3);
/// assert!(set.contains(&1));
/// assert!(set.contains(&2));
/// assert!(set.contains(&3));
/// ```
pub fn vec_to_hashset<T: Eq + Hash>(vec: Vec<T>) -> HashSet<T> {
    vec.into_iter().collect()
}

/// Converts a Vec to a BTreeSet, removing duplicates and sorting elements.
///
/// # Arguments
///
/// * `vec` - A vector of elements
///
/// # Returns
///
/// A BTreeSet containing the unique elements from the vector in sorted order
///
/// # Examples
///
/// ```
/// use collection_conversions::vec_to_btreeset;
///
/// let set = vec_to_btreeset(vec![3, 1, 2, 2]);
/// let as_vec: Vec<_> = set.into_iter().collect();
/// assert_eq!(as_vec, vec![1, 2, 3]);
/// ```
pub fn vec_to_btreeset<T: Ord>(vec: Vec<T>) -> BTreeSet<T> {
    vec.into_iter().collect()
}

/// Converts a HashSet to a sorted Vec.
///
/// # Arguments
///
/// * `set` - A HashSet of elements
///
/// # Returns
///
/// A Vec containing all elements from the set in sorted order
///
/// # Examples
///
/// ```
/// use collection_conversions::hashset_to_sorted_vec;
/// use std::collections::HashSet;
///
/// let set: HashSet<i32> = [3, 1, 4, 1, 5].into_iter().collect();
/// let sorted = hashset_to_sorted_vec(set);
/// assert_eq!(sorted, vec![1, 3, 4, 5]);
/// ```
pub fn hashset_to_sorted_vec<T: Ord>(set: HashSet<T>) -> Vec<T> {
    let mut vec: Vec<T> = set.into_iter().collect();
    vec.sort();
    vec
}

/// Converts a Vec of key-value tuples to a HashMap.
///
/// If duplicate keys exist, later values overwrite earlier ones.
///
/// # Arguments
///
/// * `pairs` - A vector of (key, value) tuples
///
/// # Returns
///
/// A HashMap built from the key-value pairs
///
/// # Examples
///
/// ```
/// use collection_conversions::pairs_to_hashmap;
///
/// let map = pairs_to_hashmap(vec![("a", 1), ("b", 2)]);
/// assert_eq!(map.get("a"), Some(&1));
/// assert_eq!(map.get("b"), Some(&2));
/// ```
pub fn pairs_to_hashmap<K: Eq + Hash, V>(pairs: Vec<(K, V)>) -> HashMap<K, V> {
    pairs.into_iter().collect()
}

/// Converts a Vec of key-value tuples to a BTreeMap (sorted by key).
///
/// If duplicate keys exist, later values overwrite earlier ones.
///
/// # Arguments
///
/// * `pairs` - A vector of (key, value) tuples
///
/// # Returns
///
/// A BTreeMap built from the key-value pairs, with keys in sorted order
///
/// # Examples
///
/// ```
/// use collection_conversions::pairs_to_btreemap;
///
/// let map = pairs_to_btreemap(vec![("c", 3), ("a", 1), ("b", 2)]);
/// let keys: Vec<_> = map.keys().collect();
/// assert_eq!(keys, vec![&"a", &"b", &"c"]);
/// ```
pub fn pairs_to_btreemap<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    pairs.into_iter().collect()
}

/// Converts a HashMap to a Vec of key-value tuples.
///
/// Note: The order of elements in the resulting Vec is not guaranteed.
///
/// # Arguments
///
/// * `map` - A HashMap to convert
///
/// # Returns
///
/// A Vec of (key, value) tuples
///
/// # Examples
///
/// ```
/// use collection_conversions::hashmap_to_pairs;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("x", 10);
/// let pairs = hashmap_to_pairs(map);
/// assert_eq!(pairs.len(), 1);
/// assert!(pairs.contains(&("x", 10)));
/// ```
pub fn hashmap_to_pairs<K, V>(map: HashMap<K, V>) -> Vec<(K, V)> {
    map.into_iter().collect()
}

/// Flattens multiple Vecs into a single Vec, preserving order.
///
/// # Arguments
///
/// * `vecs` - A vector of vectors to merge
///
/// # Returns
///
/// A single Vec containing all elements from all input vectors
///
/// # Examples
///
/// ```
/// use collection_conversions::merge_vecs;
///
/// let merged = merge_vecs(vec![vec![1, 2], vec![3, 4], vec![5]]);
/// assert_eq!(merged, vec![1, 2, 3, 4, 5]);
/// ```
pub fn merge_vecs<T>(vecs: Vec<Vec<T>>) -> Vec<T> {
    vecs.into_iter().flatten().collect()
}

/// Chains two Vecs together into one.
///
/// # Arguments
///
/// * `first` - The first vector
/// * `second` - The second vector
///
/// # Returns
///
/// A Vec with first's elements followed by second's elements
///
/// # Examples
///
/// ```
/// use collection_conversions::chain_and_collect;
///
/// let chained = chain_and_collect(vec![1, 2], vec![3, 4]);
/// assert_eq!(chained, vec![1, 2, 3, 4]);
/// ```
pub fn chain_and_collect<T>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    first.into_iter().chain(second).collect()
}
