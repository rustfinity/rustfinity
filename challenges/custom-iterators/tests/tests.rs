use custom_iterators::*;

// =============================================================================
// Fibonacci Tests
// =============================================================================

mod fibonacci_tests {
    use super::*;

    #[test]
    fn first_ten_fibonacci() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn first_fibonacci_is_zero() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
    }

    #[test]
    fn second_fibonacci_is_one() {
        let mut fib = Fibonacci::new();
        fib.next(); // 0
        assert_eq!(fib.next(), Some(1));
    }

    #[test]
    fn fibonacci_take_one() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(1).collect();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn fibonacci_take_zero() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(0).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn fibonacci_fifteenth() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.take(15).collect();
        assert_eq!(result[14], 377);
    }

    #[test]
    fn fibonacci_with_skip() {
        let fib = Fibonacci::new();
        let result: Vec<u64> = fib.skip(5).take(5).collect();
        assert_eq!(result, vec![5, 8, 13, 21, 34]);
    }

    #[test]
    fn fibonacci_sum_first_ten() {
        let fib = Fibonacci::new();
        let sum: u64 = fib.take(10).sum();
        assert_eq!(sum, 88);
    }
}

// =============================================================================
// StepRange Tests
// =============================================================================

mod step_range_tests {
    use super::*;

    #[test]
    fn ascending_step_two() {
        let range = StepRange::new(0, 10, 2);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn ascending_step_one() {
        let range = StepRange::new(1, 5, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn descending_step() {
        let range = StepRange::new(5, 0, -1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn descending_step_two() {
        let range = StepRange::new(10, 0, -2);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![10, 8, 6, 4, 2]);
    }

    #[test]
    fn step_range_empty_ascending_wrong_direction() {
        // Start < end but step is negative
        let range = StepRange::new(0, 10, -1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn step_range_empty_descending_wrong_direction() {
        // Start > end but step is positive
        let range = StepRange::new(10, 0, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn step_range_empty_same_start_end() {
        let range = StepRange::new(5, 5, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn step_range_zero_step() {
        let range = StepRange::new(0, 10, 0);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn step_range_negative_numbers() {
        let range = StepRange::new(-5, 0, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn step_range_negative_to_negative() {
        let range = StepRange::new(-10, -5, 2);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![-10, -8, -6]);
    }

    #[test]
    fn step_range_large_step() {
        let range = StepRange::new(0, 100, 30);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![0, 30, 60, 90]);
    }

    #[test]
    fn step_range_single_element() {
        let range = StepRange::new(5, 6, 1);
        let result: Vec<i32> = range.collect();
        assert_eq!(result, vec![5]);
    }
}

// =============================================================================
// CycleN Tests
// =============================================================================

mod cycle_n_tests {
    use super::*;

    #[test]
    fn cycle_twice() {
        let cycle = CycleN::new(&[1, 2, 3], 2);
        let result: Vec<i32> = cycle.collect();
        assert_eq!(result, vec![1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn cycle_once() {
        let cycle = CycleN::new(&[1, 2, 3], 1);
        let result: Vec<i32> = cycle.collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn cycle_zero_times() {
        let cycle = CycleN::new(&[1, 2, 3], 0);
        let result: Vec<i32> = cycle.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn cycle_empty_slice() {
        let cycle: CycleN<i32> = CycleN::new(&[], 5);
        let result: Vec<i32> = cycle.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn cycle_single_element() {
        let cycle = CycleN::new(&[42], 3);
        let result: Vec<i32> = cycle.collect();
        assert_eq!(result, vec![42, 42, 42]);
    }

    #[test]
    fn cycle_three_times() {
        let cycle = CycleN::new(&['a', 'b'], 3);
        let result: Vec<char> = cycle.collect();
        assert_eq!(result, vec!['a', 'b', 'a', 'b', 'a', 'b']);
    }

    #[test]
    fn cycle_strings() {
        let cycle = CycleN::new(&["hello".to_string(), "world".to_string()], 2);
        let result: Vec<String> = cycle.collect();
        assert_eq!(result, vec!["hello", "world", "hello", "world"]);
    }

    #[test]
    fn cycle_with_take() {
        let cycle = CycleN::new(&[1, 2, 3], 10);
        let result: Vec<i32> = cycle.take(5).collect();
        assert_eq!(result, vec![1, 2, 3, 1, 2]);
    }
}

// =============================================================================
// Collatz Tests
// =============================================================================

mod collatz_tests {
    use super::*;

    #[test]
    fn collatz_from_6() {
        let collatz = Collatz::new(6);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn collatz_from_1() {
        let collatz = Collatz::new(1);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn collatz_from_2() {
        let collatz = Collatz::new(2);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![2, 1]);
    }

    #[test]
    fn collatz_from_7() {
        let collatz = Collatz::new(7);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn collatz_from_27() {
        let collatz = Collatz::new(27);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result.len(), 112); // Famous for having 111 steps
        assert_eq!(result[0], 27);
        assert_eq!(result[result.len() - 1], 1);
    }

    #[test]
    fn collatz_from_0() {
        let collatz = Collatz::new(0);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn collatz_from_power_of_two() {
        let collatz = Collatz::new(16);
        let result: Vec<u64> = collatz.collect();
        assert_eq!(result, vec![16, 8, 4, 2, 1]);
    }

    #[test]
    fn collatz_count_steps() {
        let collatz = Collatz::new(6);
        assert_eq!(collatz.count(), 9);
    }

    #[test]
    fn collatz_max_value() {
        let collatz = Collatz::new(7);
        let max = collatz.max();
        assert_eq!(max, Some(52));
    }
}

// =============================================================================
// Windows Tests
// =============================================================================

mod windows_tests {
    use super::*;

    #[test]
    fn windows_size_three() {
        let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    }

    #[test]
    fn windows_size_two() {
        let windows = Windows::new(&[1, 2, 3, 4], 2);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(
            result,
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
    }

    #[test]
    fn windows_size_one() {
        let windows = Windows::new(&[1, 2, 3], 1);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn windows_size_equal_to_length() {
        let windows = Windows::new(&[1, 2, 3], 3);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn windows_size_larger_than_length() {
        let windows = Windows::new(&[1, 2, 3], 5);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn windows_empty_input() {
        let windows: Windows<i32> = Windows::new(&[], 2);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn windows_size_zero() {
        let windows = Windows::new(&[1, 2, 3], 0);
        let result: Vec<Vec<i32>> = windows.collect();
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn windows_with_strings() {
        let windows = Windows::new(&["a".to_string(), "b".to_string(), "c".to_string()], 2);
        let result: Vec<Vec<String>> = windows.collect();
        assert_eq!(
            result,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()]
            ]
        );
    }

    #[test]
    fn windows_count() {
        let windows = Windows::new(&[1, 2, 3, 4, 5, 6], 3);
        assert_eq!(windows.count(), 4);
    }
}

// =============================================================================
// Unfold Tests
// =============================================================================

mod unfold_tests {
    use super::*;

    #[test]
    fn unfold_powers_of_two() {
        let powers = Unfold::new(1u32, |&n| {
            let next = n * 2;
            if next <= 100 {
                Some(next)
            } else {
                None
            }
        });
        let result: Vec<u32> = powers.collect();
        assert_eq!(result, vec![1, 2, 4, 8, 16, 32, 64]);
    }

    #[test]
    fn unfold_countdown() {
        let countdown = Unfold::new(5i32, |&n| if n > 0 { Some(n - 1) } else { None });
        let result: Vec<i32> = countdown.collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn unfold_fibonacci_like() {
        // Using unfold to generate pairs and extract first element
        let fib = Unfold::new((0u64, 1u64), |&(a, b)| Some((b, a + b)));
        let result: Vec<u64> = fib.take(10).map(|(a, _)| a).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn unfold_immediate_stop() {
        let iter = Unfold::new(42i32, |_| None);
        let result: Vec<i32> = iter.collect();
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn unfold_single_step() {
        let iter = Unfold::new(1i32, |&n| if n < 2 { Some(n + 1) } else { None });
        let result: Vec<i32> = iter.collect();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn unfold_with_strings() {
        let iter = Unfold::new("a".to_string(), |s| {
            if s.len() < 4 {
                Some(format!("{}a", s))
            } else {
                None
            }
        });
        let result: Vec<String> = iter.collect();
        assert_eq!(result, vec!["a", "aa", "aaa", "aaaa"]);
    }

    #[test]
    fn unfold_geometric_sequence() {
        let geometric = Unfold::new(1.0f64, |&x| {
            let next = x * 0.5;
            if next >= 0.1 {
                Some(next)
            } else {
                None
            }
        });
        let result: Vec<f64> = geometric.collect();
        assert_eq!(result.len(), 4);
        assert!((result[0] - 1.0).abs() < f64::EPSILON);
        assert!((result[1] - 0.5).abs() < f64::EPSILON);
        assert!((result[2] - 0.25).abs() < f64::EPSILON);
        assert!((result[3] - 0.125).abs() < f64::EPSILON);
    }

    #[test]
    fn unfold_with_take() {
        let naturals = Unfold::new(1u32, |&n| Some(n + 1));
        let result: Vec<u32> = naturals.take(5).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn fibonacci_filter_even() {
        let fib = Fibonacci::new();
        let even_fibs: Vec<u64> = fib.take(15).filter(|&n| n % 2 == 0).collect();
        assert_eq!(even_fibs, vec![0, 2, 8, 34, 144]);
    }

    #[test]
    fn step_range_sum() {
        let range = StepRange::new(1, 11, 2);
        let sum: i32 = range.sum();
        assert_eq!(sum, 25); // 1 + 3 + 5 + 7 + 9
    }

    #[test]
    fn cycle_and_enumerate() {
        let cycle = CycleN::new(&['a', 'b'], 2);
        let indexed: Vec<(usize, char)> = cycle.enumerate().collect();
        assert_eq!(indexed, vec![(0, 'a'), (1, 'b'), (2, 'a'), (3, 'b')]);
    }

    #[test]
    fn collatz_all_greater_than_zero() {
        let mut collatz = Collatz::new(100);
        assert!(collatz.all(|n| n > 0));
    }

    #[test]
    fn windows_map_sum() {
        let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
        let sums: Vec<i32> = windows.map(|w| w.iter().sum()).collect();
        assert_eq!(sums, vec![6, 9, 12]); // 1+2+3, 2+3+4, 3+4+5
    }

    #[test]
    fn unfold_fold() {
        let iter = Unfold::new(1i32, |&n| if n < 5 { Some(n + 1) } else { None });
        let sum: i32 = iter.fold(0, |acc, n| acc + n);
        assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
    }

    #[test]
    fn combine_step_range_and_windows() {
        let range = StepRange::new(0, 6, 1);
        let values: Vec<i32> = range.collect();
        let windows = Windows::new(&values, 2);
        let pairs: Vec<Vec<i32>> = windows.collect();
        assert_eq!(
            pairs,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
        );
    }

    #[test]
    fn fibonacci_zip_with_indices() {
        let fib = Fibonacci::new();
        let indexed: Vec<(usize, u64)> = fib.take(5).enumerate().collect();
        assert_eq!(
            indexed,
            vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3)]
        );
    }

    #[test]
    fn chain_multiple_iterators() {
        let range1 = StepRange::new(0, 3, 1);
        let range2 = StepRange::new(10, 13, 1);
        let combined: Vec<i32> = range1.chain(range2).collect();
        assert_eq!(combined, vec![0, 1, 2, 10, 11, 12]);
    }
}
