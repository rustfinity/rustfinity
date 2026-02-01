use std::collections::HashSet;

/// Returns a HashSet containing only the unique elements from the input slice.
pub fn unique_elements(items: &[i32]) -> HashSet<i32> {
    // TODO: Collect items into a HashSet
    unimplemented!()
}

/// Returns the count of unique elements in the input slice.
pub fn count_unique(items: &[i32]) -> usize {
    // TODO: Count unique elements
    unimplemented!()
}

/// Returns elements that appear in both sets (intersection).
pub fn find_common(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find intersection
    unimplemented!()
}

/// Returns elements that appear in either set (union).
pub fn find_all(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find union
    unimplemented!()
}

/// Returns elements in set1 that are not in set2 (difference).
pub fn find_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find difference
    unimplemented!()
}

/// Returns elements that are in exactly one of the sets (symmetric difference).
pub fn find_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // TODO: Find symmetric difference
    unimplemented!()
}

/// Checks if all elements of potential_subset are contained in potential_superset.
pub fn is_subset(potential_subset: &HashSet<i32>, potential_superset: &HashSet<i32>) -> bool {
    // TODO: Check subset relationship
    unimplemented!()
}

pub fn main() {
    // Example: unique_elements
    let items = vec![1, 2, 3, 2, 1, 4, 3];
    let unique = unique_elements(&items);
    println!("Unique elements: {:?}", unique);

    // Example: count_unique
    let count = count_unique(&[1, 2, 2, 3, 3, 3]);
    println!("Count of unique elements: {}", count);

    // Example: set operations
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();

    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    println!("Intersection: {:?}", find_common(&set1, &set2));
    println!("Union: {:?}", find_all(&set1, &set2));
    println!("Difference (set1 - set2): {:?}", find_difference(&set1, &set2));
    println!("Symmetric difference: {:?}", find_symmetric_difference(&set1, &set2));

    // Example: is_subset
    let small: HashSet<i32> = [2, 3].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    println!("Is {:?} subset of {:?}? {}", small, large, is_subset(&small, &large));
}
