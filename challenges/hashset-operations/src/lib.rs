use std::collections::HashSet;

/// Returns a HashSet containing only the unique elements from the input slice.
///
/// # Arguments
///
/// * `items` - A slice of i32 values that may contain duplicates
///
/// # Returns
///
/// A HashSet containing each unique value from the input
///
/// # Examples
///
/// ```
/// use hashset_operations::unique_elements;
///
/// let items = vec![1, 2, 3, 2, 1, 4, 3];
/// let unique = unique_elements(&items);
/// assert_eq!(unique.len(), 4);
/// assert!(unique.contains(&1));
/// assert!(unique.contains(&4));
/// ```
pub fn unique_elements(items: &[i32]) -> HashSet<i32> {
    items.iter().cloned().collect()
}

/// Returns the count of unique elements in the input slice.
///
/// # Arguments
///
/// * `items` - A slice of i32 values that may contain duplicates
///
/// # Returns
///
/// The number of unique values in the slice
///
/// # Examples
///
/// ```
/// use hashset_operations::count_unique;
///
/// assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
/// assert_eq!(count_unique(&[]), 0);
/// ```
pub fn count_unique(items: &[i32]) -> usize {
    unique_elements(items).len()
}

/// Returns elements that appear in both sets (intersection).
///
/// # Arguments
///
/// * `set1` - The first HashSet
/// * `set2` - The second HashSet
///
/// # Returns
///
/// A new HashSet containing only elements present in both input sets
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use hashset_operations::find_common;
///
/// let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
/// let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
/// let common = find_common(&set1, &set2);
/// assert_eq!(common, [2, 3].into_iter().collect());
/// ```
pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).cloned().collect()
}

/// Returns elements that appear in either set (union).
///
/// # Arguments
///
/// * `set1` - The first HashSet
/// * `set2` - The second HashSet
///
/// # Returns
///
/// A new HashSet containing all elements from both input sets
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use hashset_operations::find_all;
///
/// let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
/// let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
/// let all = find_all(&set1, &set2);
/// assert_eq!(all, [1, 2, 3, 4].into_iter().collect());
/// ```
pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.union(set2).cloned().collect()
}

/// Returns elements in set1 that are not in set2 (difference).
///
/// # Arguments
///
/// * `set1` - The first HashSet
/// * `set2` - The second HashSet
///
/// # Returns
///
/// A new HashSet containing elements that are in set1 but not in set2
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use hashset_operations::find_difference;
///
/// let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
/// let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
/// let diff = find_difference(&set1, &set2);
/// assert_eq!(diff, [1].into_iter().collect());
/// ```
pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.difference(set2).cloned().collect()
}

/// Returns elements that are in exactly one of the sets (symmetric difference).
///
/// # Arguments
///
/// * `set1` - The first HashSet
/// * `set2` - The second HashSet
///
/// # Returns
///
/// A new HashSet containing elements that are in either set but not in both
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use hashset_operations::find_symmetric_difference;
///
/// let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
/// let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
/// let sym_diff = find_symmetric_difference(&set1, &set2);
/// assert_eq!(sym_diff, [1, 4].into_iter().collect());
/// ```
pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.symmetric_difference(set2).cloned().collect()
}

/// Checks if all elements of potential_subset are contained in potential_superset.
///
/// # Arguments
///
/// * `potential_subset` - The set to check as a potential subset
/// * `potential_superset` - The set to check as a potential superset
///
/// # Returns
///
/// `true` if all elements of potential_subset are in potential_superset, `false` otherwise
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use hashset_operations::is_subset;
///
/// let small: HashSet<i32> = [2, 3].into_iter().collect();
/// let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
/// assert!(is_subset(&small, &large));
/// assert!(!is_subset(&large, &small));
/// ```
pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) -> bool {
    potential_subset.is_subset(potential_superset)
}
