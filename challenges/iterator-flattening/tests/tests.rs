use iterator_flattening::*;

// ============================================================================
// Tests for flatten_nested
// ============================================================================

mod flatten_nested_tests {
    use super::*;

    #[test]
    fn empty_outer() {
        let nested: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten_nested(nested), vec![]);
    }

    #[test]
    fn empty_inner_vectors() {
        let nested: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        assert_eq!(flatten_nested(nested), vec![]);
    }

    #[test]
    fn single_vector() {
        let nested = vec![vec![1, 2, 3]];
        assert_eq!(flatten_nested(nested), vec![1, 2, 3]);
    }

    #[test]
    fn multiple_vectors() {
        let nested = vec![vec![1, 2], vec![3], vec![4, 5, 6]];
        assert_eq!(flatten_nested(nested), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn mixed_empty_and_filled() {
        let nested = vec![vec![], vec![1], vec![], vec![2, 3], vec![]];
        assert_eq!(flatten_nested(nested), vec![1, 2, 3]);
    }

    #[test]
    fn single_element_vectors() {
        let nested = vec![vec![1], vec![2], vec![3], vec![4]];
        assert_eq!(flatten_nested(nested), vec![1, 2, 3, 4]);
    }

    #[test]
    fn negative_numbers() {
        let nested = vec![vec![-1, -2], vec![0], vec![1, 2]];
        assert_eq!(flatten_nested(nested), vec![-1, -2, 0, 1, 2]);
    }
}

// ============================================================================
// Tests for flatten_options
// ============================================================================

mod flatten_options_tests {
    use super::*;

    #[test]
    fn empty_vector() {
        let options: Vec<Option<i32>> = vec![];
        assert_eq!(flatten_options(options), vec![]);
    }

    #[test]
    fn all_none() {
        let options = vec![None, None, None];
        assert_eq!(flatten_options(options), vec![]);
    }

    #[test]
    fn all_some() {
        let options = vec![Some(1), Some(2), Some(3)];
        assert_eq!(flatten_options(options), vec![1, 2, 3]);
    }

    #[test]
    fn mixed_some_and_none() {
        let options = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(flatten_options(options), vec![1, 3, 5]);
    }

    #[test]
    fn starts_with_none() {
        let options = vec![None, None, Some(42)];
        assert_eq!(flatten_options(options), vec![42]);
    }

    #[test]
    fn ends_with_none() {
        let options = vec![Some(1), Some(2), None, None];
        assert_eq!(flatten_options(options), vec![1, 2]);
    }

    #[test]
    fn single_some() {
        let options = vec![Some(99)];
        assert_eq!(flatten_options(options), vec![99]);
    }

    #[test]
    fn single_none() {
        let options = vec![None];
        assert_eq!(flatten_options(options), vec![]);
    }
}

// ============================================================================
// Tests for flatten_results
// ============================================================================

mod flatten_results_tests {
    use super::*;

    #[test]
    fn empty_vector() {
        let results: Vec<Result<i32, &str>> = vec![];
        assert_eq!(flatten_results(results), vec![]);
    }

    #[test]
    fn all_ok() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(flatten_results(results), vec![1, 2, 3]);
    }

    #[test]
    fn all_err() {
        let results: Vec<Result<i32, &str>> = vec![Err("a"), Err("b"), Err("c")];
        assert_eq!(flatten_results(results), vec![]);
    }

    #[test]
    fn mixed_ok_and_err() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3), Err("worse")];
        assert_eq!(flatten_results(results), vec![1, 3]);
    }

    #[test]
    fn single_ok() {
        let results: Vec<Result<i32, &str>> = vec![Ok(42)];
        assert_eq!(flatten_results(results), vec![42]);
    }

    #[test]
    fn single_err() {
        let results: Vec<Result<i32, &str>> = vec![Err("error")];
        assert_eq!(flatten_results(results), vec![]);
    }

    #[test]
    fn negative_values() {
        let results: Vec<Result<i32, &str>> = vec![Ok(-5), Err("x"), Ok(-10)];
        assert_eq!(flatten_results(results), vec![-5, -10]);
    }
}

// ============================================================================
// Tests for chars_from_words
// ============================================================================

mod chars_from_words_tests {
    use super::*;

    #[test]
    fn empty_slice() {
        let words: &[&str] = &[];
        assert_eq!(chars_from_words(words), vec![]);
    }

    #[test]
    fn single_word() {
        assert_eq!(chars_from_words(&["hello"]), vec!['h', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn multiple_words() {
        assert_eq!(
            chars_from_words(&["hi", "there"]),
            vec!['h', 'i', 't', 'h', 'e', 'r', 'e']
        );
    }

    #[test]
    fn single_char_words() {
        assert_eq!(chars_from_words(&["a", "b", "c"]), vec!['a', 'b', 'c']);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(chars_from_words(&["", "a", "", "b", ""]), vec!['a', 'b']);
    }

    #[test]
    fn unicode_chars() {
        assert_eq!(
            chars_from_words(&["héllo", "wörld"]),
            vec!['h', 'é', 'l', 'l', 'o', 'w', 'ö', 'r', 'l', 'd']
        );
    }

    #[test]
    fn emoji() {
        let chars = chars_from_words(&["hi👋"]);
        assert_eq!(chars, vec!['h', 'i', '👋']);
    }
}

// ============================================================================
// Tests for expand_ranges
// ============================================================================

mod expand_ranges_tests {
    use super::*;

    #[test]
    fn empty_ranges() {
        let ranges: &[(i32, i32)] = &[];
        assert_eq!(expand_ranges(ranges), vec![]);
    }

    #[test]
    fn single_range() {
        assert_eq!(expand_ranges(&[(1, 5)]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn multiple_ranges() {
        assert_eq!(expand_ranges(&[(1, 3), (5, 6)]), vec![1, 2, 3, 5, 6]);
    }

    #[test]
    fn single_element_range() {
        assert_eq!(expand_ranges(&[(5, 5)]), vec![5]);
    }

    #[test]
    fn adjacent_ranges() {
        assert_eq!(expand_ranges(&[(1, 2), (3, 4)]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn overlapping_ranges() {
        // Overlapping ranges will produce duplicates
        assert_eq!(expand_ranges(&[(1, 3), (2, 4)]), vec![1, 2, 3, 2, 3, 4]);
    }

    #[test]
    fn negative_ranges() {
        assert_eq!(expand_ranges(&[(-3, -1)]), vec![-3, -2, -1]);
    }

    #[test]
    fn crossing_zero() {
        assert_eq!(expand_ranges(&[(-2, 2)]), vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn multiple_single_element_ranges() {
        assert_eq!(expand_ranges(&[(1, 1), (3, 3), (5, 5)]), vec![1, 3, 5]);
    }
}

// ============================================================================
// Tests for flatten_to_depth_one
// ============================================================================

mod flatten_to_depth_one_tests {
    use super::*;

    #[test]
    fn empty_outer() {
        let deep: Vec<Vec<Vec<i32>>> = vec![];
        assert_eq!(flatten_to_depth_one(deep), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn single_middle_vector() {
        let deep = vec![vec![vec![1, 2, 3]]];
        assert_eq!(flatten_to_depth_one(deep), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn basic_case() {
        let deep = vec![vec![vec![1, 2], vec![3]], vec![vec![4]]];
        assert_eq!(flatten_to_depth_one(deep), vec![vec![1, 2], vec![3], vec![4]]);
    }

    #[test]
    fn multiple_inner_vectors() {
        let deep = vec![
            vec![vec![1], vec![2], vec![3]],
            vec![vec![4], vec![5]],
        ];
        assert_eq!(
            flatten_to_depth_one(deep),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );
    }

    #[test]
    fn empty_middle_vectors() {
        let deep = vec![vec![], vec![vec![1, 2]], vec![]];
        assert_eq!(flatten_to_depth_one(deep), vec![vec![1, 2]]);
    }

    #[test]
    fn preserves_empty_inner_vectors() {
        let deep = vec![vec![vec![], vec![1], vec![]]];
        assert_eq!(flatten_to_depth_one(deep), vec![vec![], vec![1], vec![]]);
    }
}

// ============================================================================
// Tests for words_from_lines
// ============================================================================

mod words_from_lines_tests {
    use super::*;

    #[test]
    fn empty_lines() {
        let lines: &[&str] = &[];
        let result: Vec<String> = words_from_lines(lines);
        assert!(result.is_empty());
    }

    #[test]
    fn single_line_single_word() {
        assert_eq!(words_from_lines(&["hello"]), vec!["hello"]);
    }

    #[test]
    fn single_line_multiple_words() {
        assert_eq!(
            words_from_lines(&["hello world"]),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn multiple_lines() {
        assert_eq!(
            words_from_lines(&["hello world", "foo bar"]),
            vec!["hello", "world", "foo", "bar"]
        );
    }

    #[test]
    fn lines_with_extra_whitespace() {
        assert_eq!(
            words_from_lines(&["  hello   world  ", "  foo  "]),
            vec!["hello", "world", "foo"]
        );
    }

    #[test]
    fn empty_line() {
        assert_eq!(
            words_from_lines(&["hello", "", "world"]),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn whitespace_only_line() {
        assert_eq!(
            words_from_lines(&["hello", "   ", "world"]),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn tabs_and_newlines_in_words() {
        assert_eq!(
            words_from_lines(&["hello\tworld", "foo"]),
            vec!["hello", "world", "foo"]
        );
    }

    #[test]
    fn unicode_words() {
        assert_eq!(
            words_from_lines(&["héllo wörld", "日本語"]),
            vec!["héllo", "wörld", "日本語"]
        );
    }
}

// ============================================================================
// Tests for flatten_and_filter
// ============================================================================

mod flatten_and_filter_tests {
    use super::*;

    #[test]
    fn empty_outer() {
        let nested: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten_and_filter(nested, |_| true), vec![]);
    }

    #[test]
    fn filter_even() {
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(flatten_and_filter(nested, |&x| x % 2 == 0), vec![2, 4, 6]);
    }

    #[test]
    fn filter_odd() {
        let nested = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(flatten_and_filter(nested, |&x| x % 2 == 1), vec![1, 3, 5]);
    }

    #[test]
    fn filter_all() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(flatten_and_filter(nested, |_| true), vec![1, 2, 3, 4]);
    }

    #[test]
    fn filter_none() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let result: Vec<i32> = flatten_and_filter(nested, |_| false);
        assert!(result.is_empty());
    }

    #[test]
    fn filter_greater_than() {
        let nested = vec![vec![1, 5, 3], vec![7, 2, 8]];
        assert_eq!(flatten_and_filter(nested, |&x| x > 4), vec![5, 7, 8]);
    }

    #[test]
    fn with_empty_inner_vectors() {
        let nested = vec![vec![], vec![1, 2], vec![], vec![3]];
        assert_eq!(flatten_and_filter(nested, |&x| x > 1), vec![2, 3]);
    }

    #[test]
    fn with_strings() {
        let nested = vec![
            vec!["apple".to_string(), "banana".to_string()],
            vec!["cherry".to_string(), "date".to_string()],
        ];
        let long_words = flatten_and_filter(nested, |s| s.len() > 5);
        assert_eq!(long_words, vec!["banana", "cherry"]);
    }
}

// ============================================================================
// Integration tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn process_nested_data_pipeline() {
        // Simulate processing nested data
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        // Flatten and filter for values divisible by 3
        let divisible_by_3 = flatten_and_filter(data, |&x| x % 3 == 0);
        assert_eq!(divisible_by_3, vec![3, 6, 9]);
    }

    #[test]
    fn text_processing_pipeline() {
        // Extract all characters from multiple lines
        let lines = &["Hello World", "Rust Programming"];
        let words = words_from_lines(lines);
        assert_eq!(words.len(), 4);

        // Get chars from the words
        let word_refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let chars = chars_from_words(&word_refs);
        assert_eq!(chars.len(), "HelloWorldRustProgramming".len());
    }

    #[test]
    fn numeric_range_expansion() {
        // Expand ranges and verify total count
        let ranges = &[(1, 10), (20, 25), (100, 105)];
        let expanded = expand_ranges(ranges);
        assert_eq!(expanded.len(), 10 + 6 + 6); // 10 + 6 + 6 = 22
        assert_eq!(expanded.first(), Some(&1));
        assert_eq!(expanded.last(), Some(&105));
    }

    #[test]
    fn option_and_result_processing() {
        // Simulate parsing numbers where some fail
        let strings = vec!["1", "bad", "3", "worse", "5"];
        let parsed: Vec<Option<i32>> = strings
            .iter()
            .map(|s| s.parse().ok())
            .collect();

        let values = flatten_options(parsed);
        assert_eq!(values, vec![1, 3, 5]);
    }

    #[test]
    fn deeply_nested_partial_flatten() {
        // Flatten only one level, keeping inner structure
        let matrix_of_rows = vec![
            vec![vec![1, 2], vec![3, 4]],  // First matrix
            vec![vec![5, 6], vec![7, 8]],  // Second matrix
        ];

        let all_rows = flatten_to_depth_one(matrix_of_rows);
        assert_eq!(all_rows.len(), 4);
        assert_eq!(all_rows[0], vec![1, 2]);
        assert_eq!(all_rows[3], vec![7, 8]);
    }

    #[test]
    fn combined_flatten_operations() {
        // First flatten nested, then flatten options in another operation
        let nested_optional: Vec<Vec<Option<i32>>> = vec![
            vec![Some(1), None, Some(2)],
            vec![None, Some(3)],
        ];

        // Flatten the outer structure
        let flat_options: Vec<Option<i32>> = nested_optional.into_iter().flatten().collect();
        assert_eq!(flat_options.len(), 5);

        // Then extract the Some values
        let values = flatten_options(flat_options);
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn complex_filter_after_flatten() {
        // Nested data with various conditions
        let data = vec![
            vec![-5, -3, 0, 2, 4],
            vec![1, -1, 3, -2],
            vec![10, -10, 5],
        ];

        // Flatten and keep only positive numbers
        let positive = flatten_and_filter(data.clone(), |&x| x > 0);
        assert_eq!(positive, vec![2, 4, 1, 3, 10, 5]);

        // Flatten and keep only even numbers
        let data2 = vec![
            vec![-5, -3, 0, 2, 4],
            vec![1, -1, 3, -2],
            vec![10, -10, 5],
        ];
        let even = flatten_and_filter(data2, |&x| x % 2 == 0);
        assert_eq!(even, vec![0, 2, 4, -2, 10, -10]);
    }
}
