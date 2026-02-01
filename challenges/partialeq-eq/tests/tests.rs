use partialeq_eq::*;
use std::collections::{HashMap, HashSet};

// ============================================================================
// Point tests - derived PartialEq and Eq
// ============================================================================

#[test]
fn point_equal_same_values() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert_eq!(p1, p2);
}

#[test]
fn point_not_equal_different_x() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 2 };
    assert_ne!(p1, p2);
}

#[test]
fn point_not_equal_different_y() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 4 };
    assert_ne!(p1, p2);
}

#[test]
fn point_not_equal_both_different() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    assert_ne!(p1, p2);
}

#[test]
fn point_equal_zeros() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 0, y: 0 };
    assert_eq!(p1, p2);
}

#[test]
fn point_negative_values() {
    let p1 = Point { x: -5, y: -10 };
    let p2 = Point { x: -5, y: -10 };
    assert_eq!(p1, p2);
}

#[test]
fn point_mixed_signs() {
    let p1 = Point { x: -5, y: 10 };
    let p2 = Point { x: -5, y: 10 };
    assert_eq!(p1, p2);
}

#[test]
fn point_reflexive() {
    let p = Point { x: 42, y: 99 };
    assert_eq!(p, p);
}

#[test]
fn point_symmetric() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert_eq!(p1, p2);
    assert_eq!(p2, p1);
}

// ============================================================================
// CaseInsensitiveString tests - manual PartialEq with case-insensitive comparison
// ============================================================================

#[test]
fn case_insensitive_same_case() {
    let s1 = CaseInsensitiveString::new("hello");
    let s2 = CaseInsensitiveString::new("hello");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_different_case() {
    let s1 = CaseInsensitiveString::new("Hello");
    let s2 = CaseInsensitiveString::new("HELLO");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_mixed_case() {
    let s1 = CaseInsensitiveString::new("HeLLo WoRLd");
    let s2 = CaseInsensitiveString::new("hello world");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_different_strings() {
    let s1 = CaseInsensitiveString::new("Hello");
    let s2 = CaseInsensitiveString::new("World");
    assert_ne!(s1, s2);
}

#[test]
fn case_insensitive_empty_strings() {
    let s1 = CaseInsensitiveString::new("");
    let s2 = CaseInsensitiveString::new("");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_numbers_same() {
    let s1 = CaseInsensitiveString::new("Test123");
    let s2 = CaseInsensitiveString::new("test123");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_numbers_different() {
    let s1 = CaseInsensitiveString::new("Test123");
    let s2 = CaseInsensitiveString::new("Test456");
    assert_ne!(s1, s2);
}

#[test]
fn case_insensitive_special_chars() {
    let s1 = CaseInsensitiveString::new("Hello, World!");
    let s2 = CaseInsensitiveString::new("HELLO, WORLD!");
    assert_eq!(s1, s2);
}

#[test]
fn case_insensitive_whitespace_matters() {
    let s1 = CaseInsensitiveString::new("Hello World");
    let s2 = CaseInsensitiveString::new("HelloWorld");
    assert_ne!(s1, s2);
}

#[test]
fn case_insensitive_value_preserved() {
    let s = CaseInsensitiveString::new("HeLLo");
    assert_eq!(s.value(), "HeLLo");
}

#[test]
fn case_insensitive_unicode() {
    let s1 = CaseInsensitiveString::new("Café");
    let s2 = CaseInsensitiveString::new("café");
    assert_eq!(s1, s2);
}

// ============================================================================
// ApproximateFloat tests - epsilon-based PartialEq without Eq
// ============================================================================

#[test]
fn approx_float_exact_equal() {
    let f1 = ApproximateFloat(1.0);
    let f2 = ApproximateFloat(1.0);
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_within_epsilon() {
    let f1 = ApproximateFloat(1.0);
    let f2 = ApproximateFloat(1.00005);
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_at_epsilon_boundary() {
    let f1 = ApproximateFloat(1.0);
    let f2 = ApproximateFloat(1.00009);  // Just under epsilon
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_outside_epsilon() {
    let f1 = ApproximateFloat(1.0);
    let f2 = ApproximateFloat(1.001);
    assert_ne!(f1, f2);
}

#[test]
fn approx_float_negative_within_epsilon() {
    let f1 = ApproximateFloat(-1.0);
    let f2 = ApproximateFloat(-1.00005);
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_negative_outside_epsilon() {
    let f1 = ApproximateFloat(-1.0);
    let f2 = ApproximateFloat(-1.001);
    assert_ne!(f1, f2);
}

#[test]
fn approx_float_zero() {
    let f1 = ApproximateFloat(0.0);
    let f2 = ApproximateFloat(0.00005);
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_large_values() {
    let f1 = ApproximateFloat(1000000.0);
    let f2 = ApproximateFloat(1000000.00005);
    assert_eq!(f1, f2);
}

#[test]
fn approx_float_very_different() {
    let f1 = ApproximateFloat(1.0);
    let f2 = ApproximateFloat(2.0);
    assert_ne!(f1, f2);
}

#[test]
fn approx_float_reflexive() {
    let f = ApproximateFloat(3.14159);
    assert_eq!(f, f);
}

// ============================================================================
// UserId tests - derived PartialEq, Eq, and Hash for collection use
// ============================================================================

#[test]
fn userid_equal_same_value() {
    let id1 = UserId(42);
    let id2 = UserId(42);
    assert_eq!(id1, id2);
}

#[test]
fn userid_not_equal_different() {
    let id1 = UserId(1);
    let id2 = UserId(2);
    assert_ne!(id1, id2);
}

#[test]
fn userid_zero() {
    let id1 = UserId(0);
    let id2 = UserId(0);
    assert_eq!(id1, id2);
}

#[test]
fn userid_max_value() {
    let id1 = UserId(u64::MAX);
    let id2 = UserId(u64::MAX);
    assert_eq!(id1, id2);
}

#[test]
fn userid_in_hashset() {
    let mut ids: HashSet<UserId> = HashSet::new();
    ids.insert(UserId(1));
    ids.insert(UserId(2));
    ids.insert(UserId(3));

    assert!(ids.contains(&UserId(1)));
    assert!(ids.contains(&UserId(2)));
    assert!(ids.contains(&UserId(3)));
    assert!(!ids.contains(&UserId(99)));
}

#[test]
fn userid_hashset_no_duplicates() {
    let mut ids: HashSet<UserId> = HashSet::new();
    ids.insert(UserId(1));
    ids.insert(UserId(1));
    ids.insert(UserId(1));

    assert_eq!(ids.len(), 1);
}

#[test]
fn userid_in_hashmap_key() {
    let mut map: HashMap<UserId, String> = HashMap::new();
    map.insert(UserId(1), String::from("Alice"));
    map.insert(UserId(2), String::from("Bob"));

    assert_eq!(map.get(&UserId(1)), Some(&String::from("Alice")));
    assert_eq!(map.get(&UserId(2)), Some(&String::from("Bob")));
    assert_eq!(map.get(&UserId(3)), None);
}

// ============================================================================
// Person tests - manual PartialEq based on id only
// ============================================================================

#[test]
fn person_equal_same_id_same_name() {
    let p1 = Person { name: String::from("Alice"), id: 1 };
    let p2 = Person { name: String::from("Alice"), id: 1 };
    assert_eq!(p1, p2);
}

#[test]
fn person_equal_same_id_different_name() {
    let p1 = Person { name: String::from("Alice"), id: 1 };
    let p2 = Person { name: String::from("Alice Smith"), id: 1 };
    assert_eq!(p1, p2);
}

#[test]
fn person_not_equal_different_id() {
    let p1 = Person { name: String::from("Alice"), id: 1 };
    let p2 = Person { name: String::from("Alice"), id: 2 };
    assert_ne!(p1, p2);
}

#[test]
fn person_not_equal_different_people() {
    let alice = Person { name: String::from("Alice"), id: 1 };
    let bob = Person { name: String::from("Bob"), id: 2 };
    assert_ne!(alice, bob);
}

#[test]
fn person_reflexive() {
    let p = Person { name: String::from("Test"), id: 42 };
    assert_eq!(p, p);
}

#[test]
fn person_symmetric() {
    let p1 = Person { name: String::from("Alice"), id: 1 };
    let p2 = Person { name: String::from("Bob"), id: 1 };  // Same id
    assert_eq!(p1, p2);
    assert_eq!(p2, p1);
}

#[test]
fn person_transitive() {
    let p1 = Person { name: String::from("A"), id: 1 };
    let p2 = Person { name: String::from("B"), id: 1 };
    let p3 = Person { name: String::from("C"), id: 1 };
    assert_eq!(p1, p2);
    assert_eq!(p2, p3);
    assert_eq!(p1, p3);
}

#[test]
fn person_zero_id() {
    let p1 = Person { name: String::from("Zero"), id: 0 };
    let p2 = Person { name: String::from("Also Zero"), id: 0 };
    assert_eq!(p1, p2);
}

// ============================================================================
// Status tests - derived PartialEq and Eq for enum
// ============================================================================

#[test]
fn status_equal_active() {
    assert_eq!(Status::Active, Status::Active);
}

#[test]
fn status_equal_inactive() {
    assert_eq!(Status::Inactive, Status::Inactive);
}

#[test]
fn status_equal_pending() {
    assert_eq!(Status::Pending, Status::Pending);
}

#[test]
fn status_not_equal_active_inactive() {
    assert_ne!(Status::Active, Status::Inactive);
}

#[test]
fn status_not_equal_active_pending() {
    assert_ne!(Status::Active, Status::Pending);
}

#[test]
fn status_not_equal_inactive_pending() {
    assert_ne!(Status::Inactive, Status::Pending);
}

#[test]
fn status_all_variants_different() {
    let variants = [Status::Active, Status::Inactive, Status::Pending];
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

// ============================================================================
// are_all_equal tests
// ============================================================================

#[test]
fn are_all_equal_empty() {
    assert!(are_all_equal::<i32>(&[]));
}

#[test]
fn are_all_equal_single() {
    assert!(are_all_equal(&[42]));
}

#[test]
fn are_all_equal_all_same() {
    assert!(are_all_equal(&[1, 1, 1, 1]));
}

#[test]
fn are_all_equal_first_different() {
    assert!(!are_all_equal(&[2, 1, 1, 1]));
}

#[test]
fn are_all_equal_last_different() {
    assert!(!are_all_equal(&[1, 1, 1, 2]));
}

#[test]
fn are_all_equal_middle_different() {
    assert!(!are_all_equal(&[1, 1, 2, 1]));
}

#[test]
fn are_all_equal_strings() {
    assert!(are_all_equal(&["hello", "hello", "hello"]));
    assert!(!are_all_equal(&["hello", "world", "hello"]));
}

#[test]
fn are_all_equal_points() {
    let p = Point { x: 1, y: 2 };
    assert!(are_all_equal(&[p.clone(), p.clone(), p.clone()]));
}

#[test]
fn are_all_equal_points_different() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    assert!(!are_all_equal(&[p1, p2]));
}

#[test]
fn are_all_equal_status() {
    assert!(are_all_equal(&[Status::Active, Status::Active]));
    assert!(!are_all_equal(&[Status::Active, Status::Inactive]));
}

// ============================================================================
// count_matches tests
// ============================================================================

#[test]
fn count_matches_empty() {
    assert_eq!(count_matches::<i32>(&[], &1), 0);
}

#[test]
fn count_matches_none_found() {
    assert_eq!(count_matches(&[1, 2, 3], &5), 0);
}

#[test]
fn count_matches_one_found() {
    assert_eq!(count_matches(&[1, 2, 3], &2), 1);
}

#[test]
fn count_matches_multiple_found() {
    assert_eq!(count_matches(&[1, 2, 1, 3, 1], &1), 3);
}

#[test]
fn count_matches_all_same() {
    assert_eq!(count_matches(&[5, 5, 5, 5], &5), 4);
}

#[test]
fn count_matches_strings() {
    let words = ["apple", "banana", "apple", "cherry", "apple"];
    assert_eq!(count_matches(&words, &"apple"), 3);
    assert_eq!(count_matches(&words, &"banana"), 1);
    assert_eq!(count_matches(&words, &"grape"), 0);
}

#[test]
fn count_matches_case_insensitive_strings() {
    let strings = [
        CaseInsensitiveString::new("Hello"),
        CaseInsensitiveString::new("HELLO"),
        CaseInsensitiveString::new("World"),
        CaseInsensitiveString::new("hello"),
    ];
    let target = CaseInsensitiveString::new("hello");
    assert_eq!(count_matches(&strings, &target), 3);
}

#[test]
fn count_matches_approx_floats() {
    let floats = [
        ApproximateFloat(1.0),
        ApproximateFloat(1.00005),  // Within epsilon of 1.0
        ApproximateFloat(2.0),
        ApproximateFloat(1.00009),  // Within epsilon of 1.0
    ];
    let target = ApproximateFloat(1.0);
    assert_eq!(count_matches(&floats, &target), 3);
}

// ============================================================================
// find_first_match tests
// ============================================================================

#[test]
fn find_first_match_empty() {
    assert_eq!(find_first_match::<i32>(&[], &1), None);
}

#[test]
fn find_first_match_not_found() {
    assert_eq!(find_first_match(&[1, 2, 3], &5), None);
}

#[test]
fn find_first_match_at_start() {
    assert_eq!(find_first_match(&[10, 20, 30], &10), Some(0));
}

#[test]
fn find_first_match_in_middle() {
    assert_eq!(find_first_match(&[10, 20, 30, 40], &30), Some(2));
}

#[test]
fn find_first_match_at_end() {
    assert_eq!(find_first_match(&[10, 20, 30], &30), Some(2));
}

#[test]
fn find_first_match_multiple_occurrences() {
    assert_eq!(find_first_match(&[1, 2, 1, 3, 1], &1), Some(0));
}

#[test]
fn find_first_match_single_element_found() {
    assert_eq!(find_first_match(&[42], &42), Some(0));
}

#[test]
fn find_first_match_single_element_not_found() {
    assert_eq!(find_first_match(&[42], &99), None);
}

#[test]
fn find_first_match_strings() {
    let words = ["apple", "banana", "cherry"];
    assert_eq!(find_first_match(&words, &"banana"), Some(1));
    assert_eq!(find_first_match(&words, &"grape"), None);
}

#[test]
fn find_first_match_points() {
    let points = [
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 2 },
    ];
    assert_eq!(find_first_match(&points, &Point { x: 1, y: 1 }), Some(1));
    assert_eq!(find_first_match(&points, &Point { x: 5, y: 5 }), None);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn integration_filter_by_status() {
    let items = vec![
        (String::from("Task 1"), Status::Active),
        (String::from("Task 2"), Status::Inactive),
        (String::from("Task 3"), Status::Active),
        (String::from("Task 4"), Status::Pending),
    ];

    let active_count = items.iter().filter(|(_, status)| *status == Status::Active).count();
    assert_eq!(active_count, 2);
}

#[test]
fn integration_deduplicate_userids() {
    let ids = vec![UserId(1), UserId(2), UserId(1), UserId(3), UserId(2)];
    let unique: HashSet<UserId> = ids.into_iter().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn integration_person_lookup() {
    let mut people: HashMap<u64, Person> = HashMap::new();
    people.insert(1, Person { name: String::from("Alice"), id: 1 });
    people.insert(2, Person { name: String::from("Bob"), id: 2 });

    let alice = people.get(&1).unwrap();
    assert_eq!(alice, &Person { name: String::from("Different Name"), id: 1 });  // Same id = equal
}

#[test]
fn integration_case_insensitive_word_count() {
    let words = [
        CaseInsensitiveString::new("Hello"),
        CaseInsensitiveString::new("WORLD"),
        CaseInsensitiveString::new("hello"),
        CaseInsensitiveString::new("World"),
    ];

    let hello_count = count_matches(&words, &CaseInsensitiveString::new("HELLO"));
    let world_count = count_matches(&words, &CaseInsensitiveString::new("world"));

    assert_eq!(hello_count, 2);
    assert_eq!(world_count, 2);
}

#[test]
fn integration_approximate_clustering() {
    // Find all values approximately equal to a target
    let values = [
        ApproximateFloat(1.0),
        ApproximateFloat(1.00005),
        ApproximateFloat(2.0),
        ApproximateFloat(2.00003),
        ApproximateFloat(3.0),
    ];

    let near_one: Vec<_> = values.iter()
        .filter(|v| **v == ApproximateFloat(1.0))
        .collect();
    let near_two: Vec<_> = values.iter()
        .filter(|v| **v == ApproximateFloat(2.0))
        .collect();

    assert_eq!(near_one.len(), 2);
    assert_eq!(near_two.len(), 2);
}

#[test]
fn integration_find_and_count() {
    let numbers = [5, 10, 15, 10, 20, 10];

    // Find first occurrence of 10
    let first_idx = find_first_match(&numbers, &10);
    assert_eq!(first_idx, Some(1));

    // Count all occurrences of 10
    let count = count_matches(&numbers, &10);
    assert_eq!(count, 3);

    // Check if all remaining elements after first are the same
    let rest = &numbers[first_idx.unwrap() + 1..];
    assert!(!are_all_equal(rest));  // [15, 10, 20, 10] - not all equal
}

#[test]
fn integration_equality_chain() {
    // Test transitivity with Points
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: 1, y: 2 };

    assert!(a == b && b == c && a == c);

    // Test with Person (id-based equality)
    let p1 = Person { name: String::from("Alice"), id: 100 };
    let p2 = Person { name: String::from("Bob"), id: 100 };
    let p3 = Person { name: String::from("Charlie"), id: 100 };

    assert!(p1 == p2 && p2 == p3 && p1 == p3);
}

#[test]
fn integration_combined_operations() {
    let statuses = [
        Status::Active,
        Status::Pending,
        Status::Active,
        Status::Inactive,
        Status::Active,
    ];

    // Check if all are active
    assert!(!are_all_equal(&statuses));

    // Find first active
    assert_eq!(find_first_match(&statuses, &Status::Active), Some(0));

    // Count active
    assert_eq!(count_matches(&statuses, &Status::Active), 3);
}
