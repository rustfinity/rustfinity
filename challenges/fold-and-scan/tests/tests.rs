use fold_and_scan::*;

// ============================================================================
// sum_with_fold tests
// ============================================================================

#[test]
fn test_sum_with_fold_basic() {
    assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_sum_with_fold_empty() {
    assert_eq!(sum_with_fold(&[]), 0);
}

#[test]
fn test_sum_with_fold_single() {
    assert_eq!(sum_with_fold(&[42]), 42);
}

#[test]
fn test_sum_with_fold_negative() {
    assert_eq!(sum_with_fold(&[-1, -2, -3]), -6);
}

#[test]
fn test_sum_with_fold_mixed() {
    assert_eq!(sum_with_fold(&[-5, 10, -3, 8]), 10);
}

#[test]
fn test_sum_with_fold_zeros() {
    assert_eq!(sum_with_fold(&[0, 0, 0]), 0);
}

// ============================================================================
// product_with_fold tests
// ============================================================================

#[test]
fn test_product_with_fold_basic() {
    assert_eq!(product_with_fold(&[1, 2, 3, 4]), 24);
}

#[test]
fn test_product_with_fold_empty() {
    assert_eq!(product_with_fold(&[]), 1);
}

#[test]
fn test_product_with_fold_single() {
    assert_eq!(product_with_fold(&[7]), 7);
}

#[test]
fn test_product_with_fold_with_zero() {
    assert_eq!(product_with_fold(&[1, 2, 0, 4]), 0);
}

#[test]
fn test_product_with_fold_negative() {
    assert_eq!(product_with_fold(&[-2, 3, -4]), 24);
}

#[test]
fn test_product_with_fold_large_numbers() {
    // Test that i64 handles larger products
    assert_eq!(product_with_fold(&[1000, 1000, 1000]), 1_000_000_000);
}

#[test]
fn test_product_with_fold_ones() {
    assert_eq!(product_with_fold(&[1, 1, 1, 1]), 1);
}

// ============================================================================
// concat_strings tests
// ============================================================================

#[test]
fn test_concat_strings_basic() {
    assert_eq!(concat_strings(&["a", "b", "c"], "-"), "a-b-c");
}

#[test]
fn test_concat_strings_empty() {
    assert_eq!(concat_strings(&[], "-"), "");
}

#[test]
fn test_concat_strings_single() {
    assert_eq!(concat_strings(&["hello"], ", "), "hello");
}

#[test]
fn test_concat_strings_spaces() {
    assert_eq!(concat_strings(&["Hello", "World"], " "), "Hello World");
}

#[test]
fn test_concat_strings_empty_separator() {
    assert_eq!(concat_strings(&["a", "b", "c"], ""), "abc");
}

#[test]
fn test_concat_strings_long_separator() {
    assert_eq!(concat_strings(&["x", "y"], " -> "), "x -> y");
}

#[test]
fn test_concat_strings_with_empty_strings() {
    assert_eq!(concat_strings(&["", "a", ""], "-"), "-a-");
}

// ============================================================================
// running_sum tests
// ============================================================================

#[test]
fn test_running_sum_basic() {
    assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn test_running_sum_empty() {
    assert_eq!(running_sum(&[]), Vec::<i32>::new());
}

#[test]
fn test_running_sum_single() {
    assert_eq!(running_sum(&[5]), vec![5]);
}

#[test]
fn test_running_sum_negative() {
    assert_eq!(running_sum(&[-1, -2, -3]), vec![-1, -3, -6]);
}

#[test]
fn test_running_sum_mixed() {
    assert_eq!(running_sum(&[10, -5, 3, -2]), vec![10, 5, 8, 6]);
}

#[test]
fn test_running_sum_zeros() {
    assert_eq!(running_sum(&[0, 0, 5, 0]), vec![0, 0, 5, 5]);
}

// ============================================================================
// running_max tests
// ============================================================================

#[test]
fn test_running_max_basic() {
    assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);
}

#[test]
fn test_running_max_empty() {
    assert_eq!(running_max(&[]), Vec::<i32>::new());
}

#[test]
fn test_running_max_single() {
    assert_eq!(running_max(&[42]), vec![42]);
}

#[test]
fn test_running_max_ascending() {
    assert_eq!(running_max(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_running_max_descending() {
    assert_eq!(running_max(&[5, 4, 3, 2, 1]), vec![5, 5, 5, 5, 5]);
}

#[test]
fn test_running_max_negative() {
    assert_eq!(running_max(&[-5, -3, -4, -1, -2]), vec![-5, -3, -3, -1, -1]);
}

#[test]
fn test_running_max_same_values() {
    assert_eq!(running_max(&[7, 7, 7]), vec![7, 7, 7]);
}

// ============================================================================
// take_while_sum_under tests
// ============================================================================

#[test]
fn test_take_while_sum_under_basic() {
    assert_eq!(take_while_sum_under(&[1, 2, 3, 4, 5], 10), vec![1, 2, 3]);
}

#[test]
fn test_take_while_sum_under_empty() {
    assert_eq!(take_while_sum_under(&[], 10), Vec::<i32>::new());
}

#[test]
fn test_take_while_sum_under_all_pass() {
    assert_eq!(take_while_sum_under(&[1, 2, 3], 100), vec![1, 2, 3]);
}

#[test]
fn test_take_while_sum_under_none_pass() {
    assert_eq!(take_while_sum_under(&[10, 20], 5), Vec::<i32>::new());
}

#[test]
fn test_take_while_sum_under_first_only() {
    assert_eq!(take_while_sum_under(&[5, 5, 5], 10), vec![5]);
}

#[test]
fn test_take_while_sum_under_exact_limit() {
    // Sum of [2, 3] = 5, next would be 5 + 5 = 10, which is NOT < 10
    assert_eq!(take_while_sum_under(&[2, 3, 5], 10), vec![2, 3]);
}

#[test]
fn test_take_while_sum_under_single_element() {
    assert_eq!(take_while_sum_under(&[5], 10), vec![5]);
    assert_eq!(take_while_sum_under(&[15], 10), Vec::<i32>::new());
}

#[test]
fn test_take_while_sum_under_zero_limit() {
    assert_eq!(take_while_sum_under(&[1, 2, 3], 0), Vec::<i32>::new());
}

// ============================================================================
// count_occurrences tests
// ============================================================================

#[test]
fn test_count_occurrences_basic() {
    assert_eq!(count_occurrences(&[1, 2, 1, 3, 1], &1), 3);
}

#[test]
fn test_count_occurrences_empty() {
    assert_eq!(count_occurrences(&[] as &[i32], &1), 0);
}

#[test]
fn test_count_occurrences_not_found() {
    assert_eq!(count_occurrences(&[1, 2, 3], &4), 0);
}

#[test]
fn test_count_occurrences_all_match() {
    assert_eq!(count_occurrences(&[5, 5, 5, 5], &5), 4);
}

#[test]
fn test_count_occurrences_strings() {
    assert_eq!(count_occurrences(&["a", "b", "a", "c", "a"], &"a"), 3);
}

#[test]
fn test_count_occurrences_single() {
    assert_eq!(count_occurrences(&[42], &42), 1);
    assert_eq!(count_occurrences(&[42], &0), 0);
}

// ============================================================================
// running_average tests
// ============================================================================

#[test]
fn test_running_average_basic() {
    assert_eq!(running_average(&[2.0, 4.0, 6.0]), vec![2.0, 3.0, 4.0]);
}

#[test]
fn test_running_average_empty() {
    assert_eq!(running_average(&[]), Vec::<f64>::new());
}

#[test]
fn test_running_average_single() {
    assert_eq!(running_average(&[5.0]), vec![5.0]);
}

#[test]
fn test_running_average_same_values() {
    assert_eq!(running_average(&[3.0, 3.0, 3.0]), vec![3.0, 3.0, 3.0]);
}

#[test]
fn test_running_average_ascending() {
    let result = running_average(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(result, vec![1.0, 1.5, 2.0, 2.5, 3.0]);
}

#[test]
fn test_running_average_with_zero() {
    assert_eq!(running_average(&[0.0, 10.0, 5.0]), vec![0.0, 5.0, 5.0]);
}

#[test]
fn test_running_average_negative() {
    assert_eq!(running_average(&[-2.0, -4.0, -6.0]), vec![-2.0, -3.0, -4.0]);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_fold_sum_equals_running_sum_last() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = sum_with_fold(&numbers);
    let running = running_sum(&numbers);
    assert_eq!(running.last().copied(), Some(sum));
}

#[test]
fn test_fold_product_equals_scan_product_last() {
    let numbers = vec![1, 2, 3, 4];
    let product = product_with_fold(&numbers);

    // Compute running product using scan
    let running_product: Vec<i64> = numbers
        .iter()
        .scan(1i64, |state, &x| {
            *state *= x as i64;
            Some(*state)
        })
        .collect();

    assert_eq!(running_product.last().copied(), Some(product));
}

#[test]
fn test_running_max_monotonically_non_decreasing() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let maxes = running_max(&numbers);

    for i in 1..maxes.len() {
        assert!(maxes[i] >= maxes[i - 1], "Running max should be non-decreasing");
    }
}

#[test]
fn test_take_while_and_count() {
    let numbers = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]; // 10 ones
    let taken = take_while_sum_under(&numbers, 5);
    assert_eq!(taken.len(), 4); // Can take 4 ones (sum = 4 < 5)
}

#[test]
fn test_concat_then_count() {
    let words = vec!["hello", "world", "hello"];
    let joined = concat_strings(&words, " ");
    assert!(joined.contains("hello"));
    assert_eq!(count_occurrences(&words, &"hello"), 2);
}

#[test]
fn test_running_average_precision() {
    let numbers = vec![1.0, 2.0, 3.0];
    let averages = running_average(&numbers);

    // Check precision
    assert!((averages[0] - 1.0).abs() < f64::EPSILON);
    assert!((averages[1] - 1.5).abs() < f64::EPSILON);
    assert!((averages[2] - 2.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_pipeline() {
    // Take numbers while sum < 15, then get running max
    let numbers = vec![5, 3, 2, 8, 1];
    let taken = take_while_sum_under(&numbers, 15); // [5, 3, 2] (sum = 10)
    assert_eq!(taken, vec![5, 3, 2]);

    let maxes = running_max(&taken);
    assert_eq!(maxes, vec![5, 5, 5]);
}

#[test]
fn test_count_with_custom_type() {
    #[derive(PartialEq)]
    struct Point { x: i32, y: i32 }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 0, y: 0 },
    ];

    let origin = Point { x: 0, y: 0 };
    assert_eq!(count_occurrences(&points, &origin), 2);
}
