use std::collections::BTreeSet;
use std::ops::Bound;

/// Creates a BTreeSet from a slice of numbers.
///
/// The resulting set will contain all unique elements in sorted order.
pub fn create_number_set(numbers: &[i32]) -> BTreeSet<i32> {
    // TODO: Convert the slice into a BTreeSet
    // Hint: Use .iter().copied().collect()
    unimplemented!()
}

/// Returns all elements in the range [start, end) (inclusive start, exclusive end).
pub fn get_range(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    // TODO: Use .range(start..end) to get elements in the half-open range
    // Remember to .copied().collect() the results
    unimplemented!()
}

/// Returns all elements in the range [start, end] (both inclusive).
pub fn get_range_inclusive(set: &BTreeSet<i32>, start: i32, end: i32) -> Vec<i32> {
    // TODO: Use .range(start..=end) for an inclusive range
    unimplemented!()
}

/// Returns all elements strictly less than the threshold.
pub fn get_elements_before(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    // TODO: Use .range(..threshold) for an unbounded lower range
    unimplemented!()
}

/// Returns all elements greater than or equal to the threshold.
pub fn get_elements_from(set: &BTreeSet<i32>, threshold: i32) -> Vec<i32> {
    // TODO: Use .range(threshold..) for an unbounded upper range
    unimplemented!()
}

/// Counts how many elements fall within the range [start, end] (inclusive).
pub fn count_in_range(set: &BTreeSet<i32>, start: i32, end: i32) -> usize {
    // TODO: Use .range(...).count() to count elements
    unimplemented!()
}

/// Finds the largest element in the set that is strictly less than the given value.
///
/// Returns None if no such element exists.
pub fn find_closest_less_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    // TODO: Use .range(..value) and get the last element
    // Hint: .next_back() gets the last element from an iterator
    unimplemented!()
}

/// Finds the smallest element in the set that is strictly greater than the given value.
///
/// Returns None if no such element exists.
pub fn find_closest_greater_than(set: &BTreeSet<i32>, value: i32) -> Option<i32> {
    // TODO: Use .range() with Bound::Excluded for the lower bound
    // Hint: .range((Bound::Excluded(value), Bound::Unbounded))
    unimplemented!()
}

// Example usage
pub fn main() {
    // Create a set of numbers
    let set = create_number_set(&[1, 3, 5, 7, 9, 11, 13, 15]);
    println!("Set: {:?}", set);

    // Get elements in range [5, 12)
    let range = get_range(&set, 5, 12);
    println!("Range [5, 12): {:?}", range);

    // Get elements in inclusive range [5, 11]
    let range_inclusive = get_range_inclusive(&set, 5, 11);
    println!("Range [5, 11]: {:?}", range_inclusive);

    // Get elements before 7
    let before = get_elements_before(&set, 7);
    println!("Elements < 7: {:?}", before);

    // Get elements from 9 onwards
    let from = get_elements_from(&set, 9);
    println!("Elements >= 9: {:?}", from);

    // Count elements in range [3, 11]
    let count = count_in_range(&set, 3, 11);
    println!("Count in [3, 11]: {}", count);

    // Find closest element less than 10
    let closest_less = find_closest_less_than(&set, 10);
    println!("Closest < 10: {:?}", closest_less);

    // Find closest element greater than 10
    let closest_greater = find_closest_greater_than(&set, 10);
    println!("Closest > 10: {:?}", closest_greater);
}
