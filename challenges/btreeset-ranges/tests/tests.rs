use btreeset_ranges::*;
use std::collections::BTreeSet;

// ==================== create_number_set tests ====================

#[test]
fn test_create_number_set_basic() {
    let set = create_number_set(&[3, 1, 4, 1, 5, 9, 2, 6]);
    assert_eq!(set.len(), 7); // duplicates removed
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![1, 2, 3, 4, 5, 6, 9]);
}

#[test]
fn test_create_number_set_empty() {
    let set = create_number_set(&[]);
    assert!(set.is_empty());
}

#[test]
fn test_create_number_set_single() {
    let set = create_number_set(&[42]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&42));
}

#[test]
fn test_create_number_set_all_duplicates() {
    let set = create_number_set(&[5, 5, 5, 5, 5]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&5));
}

#[test]
fn test_create_number_set_negative_numbers() {
    let set = create_number_set(&[-3, 0, 5, -1, 2]);
    let as_vec: Vec<_> = set.into_iter().collect();
    assert_eq!(as_vec, vec![-3, -1, 0, 2, 5]);
}

// ==================== get_range tests ====================

#[test]
fn test_get_range_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11, 13, 15]);
    assert_eq!(get_range(&set, 5, 12), vec![5, 7, 9, 11]);
}

#[test]
fn test_get_range_exact_bounds() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    // Start is inclusive, end is exclusive
    assert_eq!(get_range(&set, 3, 9), vec![3, 5, 7]);
}

#[test]
fn test_get_range_no_elements() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    assert_eq!(get_range(&set, 20, 30), vec![]);
}

#[test]
fn test_get_range_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(get_range(&set, 1, 10), vec![]);
}

#[test]
fn test_get_range_single_element() {
    let set = create_number_set(&[5]);
    assert_eq!(get_range(&set, 5, 6), vec![5]);
    assert_eq!(get_range(&set, 4, 5), vec![]);
}

#[test]
fn test_get_range_negative_numbers() {
    let set = create_number_set(&[-10, -5, 0, 5, 10]);
    assert_eq!(get_range(&set, -7, 3), vec![-5, 0]);
}

// ==================== get_range_inclusive tests ====================

#[test]
fn test_get_range_inclusive_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(get_range_inclusive(&set, 3, 9), vec![3, 5, 7, 9]);
}

#[test]
fn test_get_range_inclusive_single_element() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    assert_eq!(get_range_inclusive(&set, 5, 5), vec![5]);
}

#[test]
fn test_get_range_inclusive_all_elements() {
    let set = create_number_set(&[1, 2, 3]);
    assert_eq!(get_range_inclusive(&set, 1, 3), vec![1, 2, 3]);
}

#[test]
fn test_get_range_inclusive_empty_result() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    assert_eq!(get_range_inclusive(&set, 100, 200), vec![]);
}

#[test]
fn test_get_range_inclusive_negative() {
    let set = create_number_set(&[-5, -3, -1, 1, 3, 5]);
    assert_eq!(get_range_inclusive(&set, -3, 1), vec![-3, -1, 1]);
}

// ==================== get_elements_before tests ====================

#[test]
fn test_get_elements_before_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(get_elements_before(&set, 7), vec![1, 3, 5]);
}

#[test]
fn test_get_elements_before_threshold_exists() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    // 5 itself should NOT be included
    assert_eq!(get_elements_before(&set, 5), vec![1, 3]);
}

#[test]
fn test_get_elements_before_all_elements() {
    let set = create_number_set(&[1, 2, 3]);
    assert_eq!(get_elements_before(&set, 100), vec![1, 2, 3]);
}

#[test]
fn test_get_elements_before_none() {
    let set = create_number_set(&[5, 6, 7, 8]);
    assert_eq!(get_elements_before(&set, 5), vec![]);
}

#[test]
fn test_get_elements_before_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(get_elements_before(&set, 10), vec![]);
}

#[test]
fn test_get_elements_before_negative() {
    let set = create_number_set(&[-10, -5, 0, 5, 10]);
    assert_eq!(get_elements_before(&set, 0), vec![-10, -5]);
}

// ==================== get_elements_from tests ====================

#[test]
fn test_get_elements_from_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(get_elements_from(&set, 7), vec![7, 9, 11]);
}

#[test]
fn test_get_elements_from_threshold_exists() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    // 5 itself SHOULD be included
    assert_eq!(get_elements_from(&set, 5), vec![5, 7, 9]);
}

#[test]
fn test_get_elements_from_all_elements() {
    let set = create_number_set(&[5, 6, 7]);
    assert_eq!(get_elements_from(&set, 1), vec![5, 6, 7]);
}

#[test]
fn test_get_elements_from_none() {
    let set = create_number_set(&[1, 2, 3, 4]);
    assert_eq!(get_elements_from(&set, 10), vec![]);
}

#[test]
fn test_get_elements_from_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(get_elements_from(&set, 10), vec![]);
}

#[test]
fn test_get_elements_from_negative() {
    let set = create_number_set(&[-10, -5, 0, 5, 10]);
    assert_eq!(get_elements_from(&set, -5), vec![-5, 0, 5, 10]);
}

// ==================== count_in_range tests ====================

#[test]
fn test_count_in_range_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(count_in_range(&set, 3, 9), 4); // 3, 5, 7, 9
}

#[test]
fn test_count_in_range_all() {
    let set = create_number_set(&[1, 2, 3, 4, 5]);
    assert_eq!(count_in_range(&set, 1, 5), 5);
}

#[test]
fn test_count_in_range_none() {
    let set = create_number_set(&[1, 3, 5, 7]);
    assert_eq!(count_in_range(&set, 10, 20), 0);
}

#[test]
fn test_count_in_range_single() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    assert_eq!(count_in_range(&set, 5, 5), 1);
}

#[test]
fn test_count_in_range_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(count_in_range(&set, 1, 10), 0);
}

#[test]
fn test_count_in_range_negative() {
    let set = create_number_set(&[-5, -3, -1, 0, 1, 3, 5]);
    assert_eq!(count_in_range(&set, -3, 1), 4); // -3, -1, 0, 1
}

// ==================== find_closest_less_than tests ====================

#[test]
fn test_find_closest_less_than_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(find_closest_less_than(&set, 6), Some(5));
}

#[test]
fn test_find_closest_less_than_exact_match() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    // Value 5 exists, should return 3 (strictly less than 5)
    assert_eq!(find_closest_less_than(&set, 5), Some(3));
}

#[test]
fn test_find_closest_less_than_none() {
    let set = create_number_set(&[5, 7, 9]);
    assert_eq!(find_closest_less_than(&set, 5), None);
    assert_eq!(find_closest_less_than(&set, 3), None);
}

#[test]
fn test_find_closest_less_than_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(find_closest_less_than(&set, 10), None);
}

#[test]
fn test_find_closest_less_than_all_less() {
    let set = create_number_set(&[1, 2, 3, 4, 5]);
    assert_eq!(find_closest_less_than(&set, 100), Some(5));
}

#[test]
fn test_find_closest_less_than_negative() {
    let set = create_number_set(&[-10, -5, 0, 5]);
    assert_eq!(find_closest_less_than(&set, -3), Some(-5));
    assert_eq!(find_closest_less_than(&set, 0), Some(-5));
}

// ==================== find_closest_greater_than tests ====================

#[test]
fn test_find_closest_greater_than_basic() {
    let set = create_number_set(&[1, 3, 5, 7, 9, 11]);
    assert_eq!(find_closest_greater_than(&set, 6), Some(7));
}

#[test]
fn test_find_closest_greater_than_exact_match() {
    let set = create_number_set(&[1, 3, 5, 7, 9]);
    // Value 5 exists, should return 7 (strictly greater than 5)
    assert_eq!(find_closest_greater_than(&set, 5), Some(7));
}

#[test]
fn test_find_closest_greater_than_none() {
    let set = create_number_set(&[1, 3, 5]);
    assert_eq!(find_closest_greater_than(&set, 5), None);
    assert_eq!(find_closest_greater_than(&set, 7), None);
}

#[test]
fn test_find_closest_greater_than_empty_set() {
    let set: BTreeSet<i32> = BTreeSet::new();
    assert_eq!(find_closest_greater_than(&set, 10), None);
}

#[test]
fn test_find_closest_greater_than_all_greater() {
    let set = create_number_set(&[100, 200, 300]);
    assert_eq!(find_closest_greater_than(&set, 1), Some(100));
}

#[test]
fn test_find_closest_greater_than_negative() {
    let set = create_number_set(&[-10, -5, 0, 5]);
    assert_eq!(find_closest_greater_than(&set, -7), Some(-5));
    assert_eq!(find_closest_greater_than(&set, -5), Some(0));
}

// ==================== Integration tests ====================

#[test]
fn test_range_operations_workflow() {
    let set = create_number_set(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

    // Find elements in middle range
    let middle = get_range_inclusive(&set, 30, 70);
    assert_eq!(middle, vec![30, 40, 50, 60, 70]);

    // Count elements in that range
    assert_eq!(count_in_range(&set, 30, 70), 5);

    // Find boundary elements
    assert_eq!(find_closest_less_than(&set, 30), Some(20));
    assert_eq!(find_closest_greater_than(&set, 70), Some(80));
}

#[test]
fn test_before_and_from_complete_set() {
    let set = create_number_set(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let before = get_elements_before(&set, 6);
    let from = get_elements_from(&set, 6);

    // Together they should cover all elements
    assert_eq!(before, vec![1, 2, 3, 4, 5]);
    assert_eq!(from, vec![6, 7, 8, 9, 10]);

    // Combined length equals total
    assert_eq!(before.len() + from.len(), set.len());
}

#[test]
fn test_sparse_set_queries() {
    // Set with large gaps
    let set = create_number_set(&[1, 100, 1000, 10000]);

    assert_eq!(get_range(&set, 50, 5000), vec![100, 1000]);
    assert_eq!(find_closest_less_than(&set, 500), Some(100));
    assert_eq!(find_closest_greater_than(&set, 500), Some(1000));
}

#[test]
fn test_consecutive_numbers() {
    let set = create_number_set(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Every range query should work as expected with consecutive numbers
    assert_eq!(get_range(&set, 3, 8), vec![3, 4, 5, 6, 7]);
    assert_eq!(get_range_inclusive(&set, 3, 8), vec![3, 4, 5, 6, 7, 8]);
    assert_eq!(count_in_range(&set, 3, 8), 6);
}
