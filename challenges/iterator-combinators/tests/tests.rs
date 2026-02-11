use iterator_combinators::*;

// ==================== chain_sequences tests ====================

#[test]
fn test_chain_sequences_basic() {
    assert_eq!(chain_sequences(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_chain_sequences_empty_first() {
    let empty: &[i32] = &[];
    assert_eq!(chain_sequences(empty, &[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_chain_sequences_empty_second() {
    let empty: &[i32] = &[];
    assert_eq!(chain_sequences(&[1, 2, 3], empty), vec![1, 2, 3]);
}

#[test]
fn test_chain_sequences_both_empty() {
    let empty: &[i32] = &[];
    assert_eq!(chain_sequences(empty, empty), Vec::<i32>::new());
}

#[test]
fn test_chain_sequences_strings() {
    assert_eq!(
        chain_sequences(&["a", "b"], &["c", "d"]),
        vec!["a", "b", "c", "d"]
    );
}

#[test]
fn test_chain_sequences_single_elements() {
    assert_eq!(chain_sequences(&[1], &[2]), vec![1, 2]);
}

// ==================== zip_pairs tests ====================

#[test]
fn test_zip_pairs_equal_length() {
    assert_eq!(
        zip_pairs(&[1, 2, 3], &["a", "b", "c"]),
        vec![(1, "a"), (2, "b"), (3, "c")]
    );
}

#[test]
fn test_zip_pairs_first_shorter() {
    assert_eq!(
        zip_pairs(&[1, 2], &["a", "b", "c", "d"]),
        vec![(1, "a"), (2, "b")]
    );
}

#[test]
fn test_zip_pairs_second_shorter() {
    assert_eq!(
        zip_pairs(&[1, 2, 3, 4], &["a", "b"]),
        vec![(1, "a"), (2, "b")]
    );
}

#[test]
fn test_zip_pairs_empty_first() {
    let empty: &[i32] = &[];
    let result: Vec<(i32, &str)> = zip_pairs(empty, &["a", "b"]);
    assert!(result.is_empty());
}

#[test]
fn test_zip_pairs_empty_second() {
    let empty: &[&str] = &[];
    let result: Vec<(i32, &str)> = zip_pairs(&[1, 2], empty);
    assert!(result.is_empty());
}

#[test]
fn test_zip_pairs_single_elements() {
    assert_eq!(zip_pairs(&[42], &["only"]), vec![(42, "only")]);
}

#[test]
fn test_zip_pairs_numbers() {
    assert_eq!(
        zip_pairs(&[1, 2, 3], &[10, 20, 30]),
        vec![(1, 10), (2, 20), (3, 30)]
    );
}

// ==================== take_first tests ====================

#[test]
fn test_take_first_basic() {
    assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
}

#[test]
fn test_take_first_zero() {
    assert_eq!(take_first(&[1, 2, 3], 0), Vec::<i32>::new());
}

#[test]
fn test_take_first_all() {
    assert_eq!(take_first(&[1, 2, 3], 3), vec![1, 2, 3]);
}

#[test]
fn test_take_first_more_than_length() {
    assert_eq!(take_first(&[1, 2, 3], 10), vec![1, 2, 3]);
}

#[test]
fn test_take_first_empty() {
    let empty: &[i32] = &[];
    assert_eq!(take_first(empty, 5), Vec::<i32>::new());
}

#[test]
fn test_take_first_one() {
    assert_eq!(take_first(&[1, 2, 3, 4, 5], 1), vec![1]);
}

#[test]
fn test_take_first_strings() {
    assert_eq!(
        take_first(&["a", "b", "c", "d"], 2),
        vec!["a", "b"]
    );
}

// ==================== skip_first tests ====================

#[test]
fn test_skip_first_basic() {
    assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
}

#[test]
fn test_skip_first_zero() {
    assert_eq!(skip_first(&[1, 2, 3], 0), vec![1, 2, 3]);
}

#[test]
fn test_skip_first_all() {
    assert_eq!(skip_first(&[1, 2, 3], 3), Vec::<i32>::new());
}

#[test]
fn test_skip_first_more_than_length() {
    assert_eq!(skip_first(&[1, 2, 3], 10), Vec::<i32>::new());
}

#[test]
fn test_skip_first_empty() {
    let empty: &[i32] = &[];
    assert_eq!(skip_first(empty, 5), Vec::<i32>::new());
}

#[test]
fn test_skip_first_one() {
    assert_eq!(skip_first(&[1, 2, 3, 4, 5], 1), vec![2, 3, 4, 5]);
}

#[test]
fn test_skip_first_strings() {
    assert_eq!(
        skip_first(&["a", "b", "c", "d"], 2),
        vec!["c", "d"]
    );
}

// ==================== reverse_sequence tests ====================

#[test]
fn test_reverse_sequence_basic() {
    assert_eq!(reverse_sequence(&[1, 2, 3]), vec![3, 2, 1]);
}

#[test]
fn test_reverse_sequence_single() {
    assert_eq!(reverse_sequence(&[42]), vec![42]);
}

#[test]
fn test_reverse_sequence_empty() {
    let empty: &[i32] = &[];
    assert_eq!(reverse_sequence(empty), Vec::<i32>::new());
}

#[test]
fn test_reverse_sequence_two_elements() {
    assert_eq!(reverse_sequence(&[1, 2]), vec![2, 1]);
}

#[test]
fn test_reverse_sequence_strings() {
    assert_eq!(
        reverse_sequence(&["a", "b", "c"]),
        vec!["c", "b", "a"]
    );
}

#[test]
fn test_reverse_sequence_long() {
    assert_eq!(
        reverse_sequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    );
}

// ==================== interleave tests ====================

#[test]
fn test_interleave_equal_length() {
    assert_eq!(interleave(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_interleave_first_longer() {
    assert_eq!(interleave(&[1, 3, 5, 7], &[2, 4]), vec![1, 2, 3, 4, 5, 7]);
}

#[test]
fn test_interleave_second_longer() {
    assert_eq!(interleave(&[1, 3], &[2, 4, 6, 8]), vec![1, 2, 3, 4, 6, 8]);
}

#[test]
fn test_interleave_empty_first() {
    let empty: &[i32] = &[];
    assert_eq!(interleave(empty, &[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_interleave_empty_second() {
    let empty: &[i32] = &[];
    assert_eq!(interleave(&[1, 2, 3], empty), vec![1, 2, 3]);
}

#[test]
fn test_interleave_both_empty() {
    let empty: &[i32] = &[];
    assert_eq!(interleave(empty, empty), Vec::<i32>::new());
}

#[test]
fn test_interleave_single_elements() {
    assert_eq!(interleave(&[1], &[2]), vec![1, 2]);
}

#[test]
fn test_interleave_strings() {
    assert_eq!(
        interleave(&["a", "c", "e"], &["b", "d", "f"]),
        vec!["a", "b", "c", "d", "e", "f"]
    );
}

// ==================== sliding_pairs tests ====================

#[test]
fn test_sliding_pairs_basic() {
    assert_eq!(
        sliding_pairs(&[1, 2, 3, 4]),
        vec![(1, 2), (2, 3), (3, 4)]
    );
}

#[test]
fn test_sliding_pairs_two_elements() {
    assert_eq!(sliding_pairs(&[1, 2]), vec![(1, 2)]);
}

#[test]
fn test_sliding_pairs_single_element() {
    assert_eq!(sliding_pairs(&[1]), Vec::<(i32, i32)>::new());
}

#[test]
fn test_sliding_pairs_empty() {
    let empty: &[i32] = &[];
    assert_eq!(sliding_pairs(empty), Vec::<(i32, i32)>::new());
}

#[test]
fn test_sliding_pairs_five_elements() {
    assert_eq!(
        sliding_pairs(&[1, 2, 3, 4, 5]),
        vec![(1, 2), (2, 3), (3, 4), (4, 5)]
    );
}

#[test]
fn test_sliding_pairs_strings() {
    assert_eq!(
        sliding_pairs(&["a", "b", "c"]),
        vec![("a", "b"), ("b", "c")]
    );
}

#[test]
fn test_sliding_pairs_duplicates() {
    assert_eq!(
        sliding_pairs(&[1, 1, 2, 2]),
        vec![(1, 1), (1, 2), (2, 2)]
    );
}

// ==================== Integration tests ====================

#[test]
fn test_chain_then_take() {
    // Combine chain and take
    let combined = chain_sequences(&[1, 2, 3], &[4, 5, 6]);
    let result = take_first(&combined, 4);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_chain_then_skip() {
    // Combine chain and skip
    let combined = chain_sequences(&[1, 2], &[3, 4, 5]);
    let result = skip_first(&combined, 2);
    assert_eq!(result, vec![3, 4, 5]);
}

#[test]
fn test_reverse_then_take() {
    // Reverse and take first elements (which were last)
    let reversed = reverse_sequence(&[1, 2, 3, 4, 5]);
    let result = take_first(&reversed, 2);
    assert_eq!(result, vec![5, 4]);
}

#[test]
fn test_interleave_then_reverse() {
    // Interleave and then reverse
    let interleaved = interleave(&[1, 3], &[2, 4]);
    let result = reverse_sequence(&interleaved);
    assert_eq!(result, vec![4, 3, 2, 1]);
}

#[test]
fn test_zip_indices() {
    // Common pattern: zip with indices
    let items = ["a", "b", "c"];
    let indices: Vec<usize> = (0..items.len()).collect();
    let result = zip_pairs(&indices, &items);
    assert_eq!(result, vec![(0, "a"), (1, "b"), (2, "c")]);
}

#[test]
fn test_sliding_pairs_differences() {
    // Use sliding pairs to compute differences
    let numbers = [10, 15, 12, 20, 18];
    let pairs = sliding_pairs(&numbers);
    let differences: Vec<i32> = pairs.iter().map(|(a, b)| b - a).collect();
    assert_eq!(differences, vec![5, -3, 8, -2]);
}

#[test]
fn test_complex_pipeline() {
    // Chain -> skip -> take -> reverse
    let first = [1, 2, 3];
    let second = [4, 5, 6, 7, 8];
    let chained = chain_sequences(&first, &second);
    let skipped = skip_first(&chained, 2);
    let taken = take_first(&skipped, 4);
    let result = reverse_sequence(&taken);
    assert_eq!(result, vec![6, 5, 4, 3]);
}
