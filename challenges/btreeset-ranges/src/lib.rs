use std::collections::BTreeSet;
use std::ops::Bound;

/// Creates a BTreeSet from a slice of numbers.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values
///
/// # Returns
///
/// A BTreeSet containing all unique elements from the slice in sorted order
///
/// # Examples
///
/// ```
/// use btreeset_ranges::create_number_set;
///
/// let set = create_number_set(&[3, 1, 4, 1, 5, 9, 2, 6]);
/// assert_eq!(set.len(), 7); // duplicates removed
/// assert!(set.contains(&1));
/// assert!(set.contains(&9));
/// ```
pub fn create_number_set(numbers: &[i32]) -> BTreeSet<i32> {
    numbers.iter().copied().collect()
}

/// Returns all elements in the range [start, end) (inclusive start, exclusive end).
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `start` - The inclusive lower bound
/// * `end` - The exclusive upper bound
///
/// # Returns
///
/// A Vec containing elements where start <= element < end
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, get_range};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(get_range(&set, 3, 9), vec![3, 5, 7]);
/// ```
pub fn get_range(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    set.range(start..end).copied().collect()
}

/// Returns all elements in the range [start, end] (both inclusive).
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `start` - The inclusive lower bound
/// * `end` - The inclusive upper bound
///
/// # Returns
///
/// A Vec containing elements where start <= element <= end
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, get_range_inclusive};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(get_range_inclusive(&set, 3, 9), vec![3, 5, 7, 9]);
/// ```
pub fn get_range_inclusive(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    set.range(start..=end).copied().collect()
}

/// Returns all elements strictly less than the threshold.
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `threshold` - The exclusive upper bound
///
/// # Returns
///
/// A Vec containing elements where element < threshold
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, get_elements_before};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(get_elements_before(&set, 7), vec![1, 3, 5]);
/// ```
pub fn get_elements_before(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    set.range(..threshold).copied().collect()
}

/// Returns all elements greater than or equal to the threshold.
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `threshold` - The inclusive lower bound
///
/// # Returns
///
/// A Vec containing elements where element >= threshold
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, get_elements_from};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(get_elements_from(&set, 7), vec![7, 9, 11]);
/// ```
pub fn get_elements_from(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    set.range(threshold..).copied().collect()
}

/// Counts how many elements fall within the range [start, end] (inclusive).
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `start` - The inclusive lower bound
/// * `end` - The inclusive upper bound
///
/// # Returns
///
/// The count of elements where start <= element <= end
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, count_in_range};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(count_in_range(&set, 3, 9), 4); // 3, 5, 7, 9
/// ```
pub fn count_in_range(set: &BTreeSet<i32>, start: i32, end: i32) -> usize {
    set.range(start..=end).count()
}

/// Finds the largest element in the set that is strictly less than the given value.
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `value` - The exclusive upper bound
///
/// # Returns
///
/// `Some(element)` if an element exists that is less than value, `None` otherwise
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, find_closest_less_than};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(find_closest_less_than(&set, 6), Some(5));
/// assert_eq!(find_closest_less_than(&set, 1), None);
/// ```
pub fn find_closest_less_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    set.range(..value).next_back().copied()
}

/// Finds the smallest element in the set that is strictly greater than the given value.
///
/// # Arguments
///
/// * `set` - A reference to the BTreeSet
/// * `value` - The exclusive lower bound
///
/// # Returns
///
/// `Some(element)` if an element exists that is greater than value, `None` otherwise
///
/// # Examples
///
/// ```
/// use btreeset_ranges::{create_number_set, find_closest_greater_than};
///
/// let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
/// assert_eq!(find_closest_greater_than(&set, 6), Some(7));
/// assert_eq!(find_closest_greater_than(&set, 11), None);
/// ```
pub fn find_closest_greater_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    set.range((Bound::Excluded(value), Bound::Unbounded))
        .next()
        .copied()
}
