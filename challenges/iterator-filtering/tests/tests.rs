use iterator_filtering::*;

// ==================== filter_even tests ====================

#[test]
fn test_filter_even_mixed() {
    assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
}

#[test]
fn test_filter_even_all_even() {
    assert_eq!(filter_even(&[2, 4, 6, 8]), vec![2, 4, 6, 8]);
}

#[test]
fn test_filter_even_all_odd() {
    assert_eq!(filter_even(&[1, 3, 5, 7]), vec![]);
}

#[test]
fn test_filter_even_empty() {
    assert_eq!(filter_even(&[]), vec![]);
}

#[test]
fn test_filter_even_with_negatives() {
    assert_eq!(filter_even(&[-4, -3, -2, -1, 0, 1, 2]), vec![-4, -2, 0, 2]);
}

#[test]
fn test_filter_even_single_even() {
    assert_eq!(filter_even(&[4]), vec![4]);
}

#[test]
fn test_filter_even_single_odd() {
    assert_eq!(filter_even(&[5]), vec![]);
}

// ==================== filter_by_predicate tests ====================

#[test]
fn test_filter_by_predicate_greater_than() {
    assert_eq!(filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x > 3), vec![4, 5]);
}

#[test]
fn test_filter_by_predicate_less_than() {
    assert_eq!(filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x < 3), vec![1, 2]);
}

#[test]
fn test_filter_by_predicate_odd() {
    assert_eq!(filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x % 2 == 1), vec![1, 3, 5]);
}

#[test]
fn test_filter_by_predicate_none_match() {
    assert_eq!(filter_by_predicate(&[1, 2, 3], |&x| x > 10), vec![]);
}

#[test]
fn test_filter_by_predicate_all_match() {
    assert_eq!(filter_by_predicate(&[1, 2, 3], |&x| x > 0), vec![1, 2, 3]);
}

#[test]
fn test_filter_by_predicate_empty() {
    assert_eq!(filter_by_predicate(&[], |&x| x > 0), vec![]);
}

#[test]
fn test_filter_by_predicate_divisible_by_3() {
    assert_eq!(filter_by_predicate(&[1, 3, 5, 6, 9, 10], |&x| x % 3 == 0), vec![3, 6, 9]);
}

// ==================== parse_valid_numbers tests ====================

#[test]
fn test_parse_valid_numbers_mixed() {
    assert_eq!(parse_valid_numbers(&["1", "hello", "3", "world"]), vec![1, 3]);
}

#[test]
fn test_parse_valid_numbers_all_valid() {
    assert_eq!(parse_valid_numbers(&["42", "-5", "0", "100"]), vec![42, -5, 0, 100]);
}

#[test]
fn test_parse_valid_numbers_all_invalid() {
    assert_eq!(parse_valid_numbers(&["abc", "def", "xyz"]), vec![]);
}

#[test]
fn test_parse_valid_numbers_empty() {
    assert_eq!(parse_valid_numbers(&[]), vec![]);
}

#[test]
fn test_parse_valid_numbers_with_whitespace() {
    // parse::<i32>() doesn't accept whitespace
    assert_eq!(parse_valid_numbers(&[" 1", "2 ", " 3 "]), vec![]);
}

#[test]
fn test_parse_valid_numbers_floats_fail() {
    assert_eq!(parse_valid_numbers(&["1.5", "2.0", "3"]), vec![3]);
}

#[test]
fn test_parse_valid_numbers_negative() {
    assert_eq!(parse_valid_numbers(&["-1", "-2", "-3"]), vec![-1, -2, -3]);
}

#[test]
fn test_parse_valid_numbers_large_numbers() {
    assert_eq!(parse_valid_numbers(&["2147483647", "-2147483648"]), vec![2147483647, -2147483648]);
}

// ==================== filter_map_with tests ====================

#[test]
fn test_filter_map_with_double_even() {
    let result: Vec<i32> = filter_map_with(&[1, 2, 3, 4], |x| if x % 2 == 0 { Some(x * 2) } else { None });
    assert_eq!(result, vec![4, 8]);
}

#[test]
fn test_filter_map_with_square_positive() {
    let result: Vec<i32> = filter_map_with(&[-2, -1, 0, 1, 2], |x| if x > 0 { Some(x * x) } else { None });
    assert_eq!(result, vec![1, 4]);
}

#[test]
fn test_filter_map_with_empty() {
    let result: Vec<i32> = filter_map_with(&[], |x: i32| Some(x));
    assert_eq!(result, vec![]);
}

#[test]
fn test_filter_map_with_all_none() {
    let result: Vec<i32> = filter_map_with(&[1, 2, 3], |_| None);
    assert_eq!(result, vec![]);
}

#[test]
fn test_filter_map_with_all_some() {
    let result: Vec<i32> = filter_map_with(&[1, 2, 3], |x| Some(x + 10));
    assert_eq!(result, vec![11, 12, 13]);
}

#[test]
fn test_filter_map_with_string_lengths() {
    let result: Vec<usize> = filter_map_with(&["a", "bb", "ccc"], |s: &str| {
        if s.len() > 1 { Some(s.len()) } else { None }
    });
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn test_filter_map_with_type_conversion() {
    let result: Vec<String> = filter_map_with(&[1, 2, 3, 4, 5], |x| {
        if x % 2 == 0 { Some(format!("even:{}", x)) } else { None }
    });
    assert_eq!(result, vec!["even:2", "even:4"]);
}

// ==================== take_while_positive tests ====================

#[test]
fn test_take_while_positive_basic() {
    assert_eq!(take_while_positive(&[3, 5, -1, 2, 4]), vec![3, 5]);
}

#[test]
fn test_take_while_positive_starts_negative() {
    assert_eq!(take_while_positive(&[-1, 2, 3]), vec![]);
}

#[test]
fn test_take_while_positive_all_positive() {
    assert_eq!(take_while_positive(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_take_while_positive_all_negative() {
    assert_eq!(take_while_positive(&[-1, -2, -3]), vec![]);
}

#[test]
fn test_take_while_positive_empty() {
    assert_eq!(take_while_positive(&[]), vec![]);
}

#[test]
fn test_take_while_positive_stops_at_zero() {
    assert_eq!(take_while_positive(&[1, 2, 0, 3, 4]), vec![1, 2]);
}

#[test]
fn test_take_while_positive_single_positive() {
    assert_eq!(take_while_positive(&[5]), vec![5]);
}

#[test]
fn test_take_while_positive_single_negative() {
    assert_eq!(take_while_positive(&[-5]), vec![]);
}

// ==================== skip_while_negative tests ====================

#[test]
fn test_skip_while_negative_basic() {
    assert_eq!(skip_while_negative(&[-3, -1, 2, -4, 5]), vec![2, -4, 5]);
}

#[test]
fn test_skip_while_negative_starts_positive() {
    assert_eq!(skip_while_negative(&[1, -2, -3]), vec![1, -2, -3]);
}

#[test]
fn test_skip_while_negative_all_negative() {
    assert_eq!(skip_while_negative(&[-1, -2, -3]), vec![]);
}

#[test]
fn test_skip_while_negative_all_positive() {
    assert_eq!(skip_while_negative(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_skip_while_negative_empty() {
    assert_eq!(skip_while_negative(&[]), vec![]);
}

#[test]
fn test_skip_while_negative_stops_at_zero() {
    assert_eq!(skip_while_negative(&[-1, -2, 0, -3, 4]), vec![0, -3, 4]);
}

#[test]
fn test_skip_while_negative_single_negative() {
    assert_eq!(skip_while_negative(&[-5]), vec![]);
}

#[test]
fn test_skip_while_negative_single_positive() {
    assert_eq!(skip_while_negative(&[5]), vec![5]);
}

// ==================== filter_in_range tests ====================

#[test]
fn test_filter_in_range_basic() {
    assert_eq!(filter_in_range(&[1, 5, 10, 15, 20], 5, 15), vec![5, 10, 15]);
}

#[test]
fn test_filter_in_range_none_in_range() {
    assert_eq!(filter_in_range(&[1, 2, 3], 10, 20), vec![]);
}

#[test]
fn test_filter_in_range_all_in_range() {
    assert_eq!(filter_in_range(&[5, 6, 7], 1, 10), vec![5, 6, 7]);
}

#[test]
fn test_filter_in_range_empty() {
    assert_eq!(filter_in_range(&[], 1, 10), vec![]);
}

#[test]
fn test_filter_in_range_single_value_range() {
    assert_eq!(filter_in_range(&[1, 2, 3, 4, 5], 3, 3), vec![3]);
}

#[test]
fn test_filter_in_range_negative_range() {
    assert_eq!(filter_in_range(&[-5, -3, -1, 0, 1, 3, 5], -3, 1), vec![-3, -1, 0, 1]);
}

#[test]
fn test_filter_in_range_boundary_values() {
    assert_eq!(filter_in_range(&[0, 1, 2, 3, 4, 5], 0, 5), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_filter_in_range_large_range() {
    assert_eq!(filter_in_range(&[100, 200, 300], 0, 1000), vec![100, 200, 300]);
}

// ==================== first_matching tests ====================

#[test]
fn test_first_matching_found() {
    assert_eq!(first_matching(&[1, 2, 3, 4, 5], |&x| x > 3), Some(4));
}

#[test]
fn test_first_matching_not_found() {
    assert_eq!(first_matching(&[1, 2, 3], |&x| x > 10), None);
}

#[test]
fn test_first_matching_empty() {
    assert_eq!(first_matching::<i32, _>(&[], |&x| x > 0), None);
}

#[test]
fn test_first_matching_first_element() {
    assert_eq!(first_matching(&[5, 1, 2, 3], |&x| x > 4), Some(5));
}

#[test]
fn test_first_matching_last_element() {
    assert_eq!(first_matching(&[1, 2, 3, 5], |&x| x > 4), Some(5));
}

#[test]
fn test_first_matching_multiple_matches() {
    // Should return the first match
    assert_eq!(first_matching(&[1, 4, 5, 6], |&x| x > 3), Some(4));
}

#[test]
fn test_first_matching_with_strings() {
    let strings = vec!["apple", "banana", "cherry"];
    assert_eq!(first_matching(&strings, |s| s.starts_with('b')), Some("banana"));
}

#[test]
fn test_first_matching_with_strings_not_found() {
    let strings = vec!["apple", "banana", "cherry"];
    assert_eq!(first_matching(&strings, |s| s.starts_with('z')), None);
}

// ==================== Integration tests ====================

#[test]
fn test_chained_filtering() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Filter even numbers, then filter those in range
    let evens = filter_even(&numbers);
    let result = filter_in_range(&evens, 4, 8);
    assert_eq!(result, vec![4, 6, 8]);
}

#[test]
fn test_parse_then_filter() {
    let strings = vec!["1", "20", "hello", "5", "100", "world"];
    let numbers = parse_valid_numbers(&strings);
    let small_numbers = filter_by_predicate(&numbers, |&x| x < 50);
    assert_eq!(small_numbers, vec![1, 20, 5]);
}

#[test]
fn test_take_and_skip_complement() {
    let numbers = vec![1, 2, 3, 4, 5];
    // take_while + skip_while should cover all elements for a boundary condition
    let taken = take_while_positive(&numbers);  // [1, 2, 3, 4, 5]
    assert_eq!(taken.len(), 5);

    let numbers_with_negative = vec![-1, -2, 3, 4, 5];
    let skipped = skip_while_negative(&numbers_with_negative);  // [3, 4, 5]
    assert_eq!(skipped, vec![3, 4, 5]);
}

#[test]
fn test_filter_map_chain() {
    // Parse strings, filter positives, then double
    let strings = vec!["1", "-2", "3", "invalid", "-4", "5"];
    let numbers = parse_valid_numbers(&strings);  // [1, -2, 3, -4, 5]
    let result: Vec<i32> = filter_map_with(&numbers, |x| {
        if x > 0 { Some(x * 2) } else { None }
    });
    assert_eq!(result, vec![2, 6, 10]);
}

#[test]
fn test_first_matching_with_filter() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = filter_even(&numbers);
    let first_greater_than_5 = first_matching(&evens, |&x| x > 5);
    assert_eq!(first_greater_than_5, Some(6));
}

#[test]
fn test_complex_pipeline() {
    // Start with strings, parse, filter range, find first matching condition
    let data = vec!["5", "10", "abc", "15", "20", "xyz", "25", "30"];
    let numbers = parse_valid_numbers(&data);  // [5, 10, 15, 20, 25, 30]
    let in_range = filter_in_range(&numbers, 10, 25);  // [10, 15, 20, 25]
    let first_odd = first_matching(&in_range, |&x| x % 2 == 1);
    assert_eq!(first_odd, Some(15));
}

#[test]
fn test_edge_case_all_filtered_out() {
    let numbers = vec![1, 3, 5, 7, 9];
    let evens = filter_even(&numbers);
    assert!(evens.is_empty());

    let first = first_matching(&evens, |&x| x > 0);
    assert_eq!(first, None);
}

#[test]
fn test_filter_with_custom_struct() {
    #[derive(Clone, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Charlie".to_string(), age: 20 },
    ];

    let adult = first_matching(&people, |p| p.age >= 25);
    assert_eq!(adult, Some(Person { name: "Alice".to_string(), age: 25 }));

    let elderly = first_matching(&people, |p| p.age >= 65);
    assert_eq!(elderly, None);
}
