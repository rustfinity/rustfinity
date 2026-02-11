use hashset_operations::*;
use std::collections::HashSet;

// ==================== unique_elements tests ====================

#[test]
fn test_unique_elements_with_duplicates() {
    let items = vec![1, 2, 3, 2, 1, 4, 3];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 4);
    assert!(unique.contains(&1));
    assert!(unique.contains(&2));
    assert!(unique.contains(&3));
    assert!(unique.contains(&4));
}

#[test]
fn test_unique_elements_all_unique() {
    let items = vec![1, 2, 3, 4, 5];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 5);
}

#[test]
fn test_unique_elements_all_same() {
    let items = vec![7, 7, 7, 7, 7];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 1);
    assert!(unique.contains(&7));
}

#[test]
fn test_unique_elements_empty() {
    let items: Vec<i32> = vec![];
    let unique = unique_elements(&items);
    assert!(unique.is_empty());
}

#[test]
fn test_unique_elements_single() {
    let items = vec![42];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 1);
    assert!(unique.contains(&42));
}

#[test]
fn test_unique_elements_negative_numbers() {
    let items = vec![-1, -2, -1, 0, -2, 1];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 4);
    assert!(unique.contains(&-1));
    assert!(unique.contains(&-2));
    assert!(unique.contains(&0));
    assert!(unique.contains(&1));
}

// ==================== count_unique tests ====================

#[test]
fn test_count_unique_with_duplicates() {
    assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
}

#[test]
fn test_count_unique_all_unique() {
    assert_eq!(count_unique(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_count_unique_all_same() {
    assert_eq!(count_unique(&[1, 1, 1, 1]), 1);
}

#[test]
fn test_count_unique_empty() {
    assert_eq!(count_unique(&[]), 0);
}

#[test]
fn test_count_unique_single() {
    assert_eq!(count_unique(&[42]), 1);
}

// ==================== find_common tests ====================

#[test]
fn test_find_common_overlapping() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let common = find_common(&set1, &set2);
    let expected: HashSet<i32> = [2, 3].into_iter().collect();
    assert_eq!(common, expected);
}

#[test]
fn test_find_common_disjoint() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
    let common = find_common(&set1, &set2);
    assert!(common.is_empty());
}

#[test]
fn test_find_common_identical() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let common = find_common(&set1, &set2);
    assert_eq!(common, set1);
}

#[test]
fn test_find_common_one_empty() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = HashSet::new();
    let common = find_common(&set1, &set2);
    assert!(common.is_empty());
}

#[test]
fn test_find_common_subset() {
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = [2, 3].into_iter().collect();
    let common = find_common(&set1, &set2);
    let expected: HashSet<i32> = [2, 3].into_iter().collect();
    assert_eq!(common, expected);
}

// ==================== find_all tests ====================

#[test]
fn test_find_all_overlapping() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let all = find_all(&set1, &set2);
    let expected: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    assert_eq!(all, expected);
}

#[test]
fn test_find_all_disjoint() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
    let all = find_all(&set1, &set2);
    let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6].into_iter().collect();
    assert_eq!(all, expected);
}

#[test]
fn test_find_all_identical() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let all = find_all(&set1, &set2);
    assert_eq!(all, set1);
}

#[test]
fn test_find_all_one_empty() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = HashSet::new();
    let all = find_all(&set1, &set2);
    assert_eq!(all, set1);
}

#[test]
fn test_find_all_both_empty() {
    let set1: HashSet<i32> = HashSet::new();
    let set2: HashSet<i32> = HashSet::new();
    let all = find_all(&set1, &set2);
    assert!(all.is_empty());
}

// ==================== find_difference tests ====================

#[test]
fn test_find_difference_overlapping() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let diff = find_difference(&set1, &set2);
    let expected: HashSet<i32> = [1].into_iter().collect();
    assert_eq!(diff, expected);
}

#[test]
fn test_find_difference_disjoint() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
    let diff = find_difference(&set1, &set2);
    assert_eq!(diff, set1);
}

#[test]
fn test_find_difference_identical() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let diff = find_difference(&set1, &set2);
    assert!(diff.is_empty());
}

#[test]
fn test_find_difference_first_empty() {
    let set1: HashSet<i32> = HashSet::new();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let diff = find_difference(&set1, &set2);
    assert!(diff.is_empty());
}

#[test]
fn test_find_difference_second_empty() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = HashSet::new();
    let diff = find_difference(&set1, &set2);
    assert_eq!(diff, set1);
}

#[test]
fn test_find_difference_subset() {
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = [2, 3].into_iter().collect();
    let diff = find_difference(&set1, &set2);
    let expected: HashSet<i32> = [1, 4, 5].into_iter().collect();
    assert_eq!(diff, expected);
}

// ==================== find_symmetric_difference tests ====================

#[test]
fn test_find_symmetric_difference_overlapping() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let sym_diff = find_symmetric_difference(&set1, &set2);
    let expected: HashSet<i32> = [1, 4].into_iter().collect();
    assert_eq!(sym_diff, expected);
}

#[test]
fn test_find_symmetric_difference_disjoint() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
    let sym_diff = find_symmetric_difference(&set1, &set2);
    let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6].into_iter().collect();
    assert_eq!(sym_diff, expected);
}

#[test]
fn test_find_symmetric_difference_identical() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let sym_diff = find_symmetric_difference(&set1, &set2);
    assert!(sym_diff.is_empty());
}

#[test]
fn test_find_symmetric_difference_one_empty() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = HashSet::new();
    let sym_diff = find_symmetric_difference(&set1, &set2);
    assert_eq!(sym_diff, set1);
}

#[test]
fn test_find_symmetric_difference_both_empty() {
    let set1: HashSet<i32> = HashSet::new();
    let set2: HashSet<i32> = HashSet::new();
    let sym_diff = find_symmetric_difference(&set1, &set2);
    assert!(sym_diff.is_empty());
}

// ==================== is_subset tests ====================

#[test]
fn test_is_subset_true() {
    let small: HashSet<i32> = [2, 3].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    assert!(is_subset(&small, &large));
}

#[test]
fn test_is_subset_false() {
    let small: HashSet<i32> = [2, 3, 5].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    assert!(!is_subset(&small, &large));
}

#[test]
fn test_is_subset_equal_sets() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert!(is_subset(&set1, &set2));
}

#[test]
fn test_is_subset_empty_is_subset_of_any() {
    let empty: HashSet<i32> = HashSet::new();
    let non_empty: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert!(is_subset(&empty, &non_empty));
}

#[test]
fn test_is_subset_empty_of_empty() {
    let empty1: HashSet<i32> = HashSet::new();
    let empty2: HashSet<i32> = HashSet::new();
    assert!(is_subset(&empty1, &empty2));
}

#[test]
fn test_is_subset_reverse_not_subset() {
    let small: HashSet<i32> = [2, 3].into_iter().collect();
    let large: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    assert!(!is_subset(&large, &small));
}

#[test]
fn test_is_subset_disjoint() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [4, 5, 6].into_iter().collect();
    assert!(!is_subset(&set1, &set2));
}

// ==================== Additional edge case tests ====================

#[test]
fn test_operations_with_large_numbers() {
    let items = vec![i32::MAX, i32::MIN, 0, i32::MAX, i32::MIN];
    let unique = unique_elements(&items);
    assert_eq!(unique.len(), 3);
    assert!(unique.contains(&i32::MAX));
    assert!(unique.contains(&i32::MIN));
    assert!(unique.contains(&0));
}

#[test]
fn test_symmetric_difference_commutative() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let sym_diff1 = find_symmetric_difference(&set1, &set2);
    let sym_diff2 = find_symmetric_difference(&set2, &set1);
    assert_eq!(sym_diff1, sym_diff2);
}

#[test]
fn test_union_intersection_relationship() {
    // |A ∪ B| = |A| + |B| - |A ∩ B|
    let set1: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();
    let union = find_all(&set1, &set2);
    let intersection = find_common(&set1, &set2);
    assert_eq!(union.len(), set1.len() + set2.len() - intersection.len());
}

#[test]
fn test_difference_not_commutative() {
    let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let diff1 = find_difference(&set1, &set2);
    let diff2 = find_difference(&set2, &set1);
    assert_ne!(diff1, diff2);
    let expected1: HashSet<i32> = [1].into_iter().collect();
    let expected2: HashSet<i32> = [4].into_iter().collect();
    assert_eq!(diff1, expected1);
    assert_eq!(diff2, expected2);
}
