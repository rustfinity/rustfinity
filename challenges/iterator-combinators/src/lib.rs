/// Combines two slices into a single Vec using chain().
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec containing all elements from first, followed by all elements from second
///
/// # Examples
/// ```
/// use iterator_combinators::chain_sequences;
/// assert_eq!(chain_sequences(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
/// ```
pub fn chain_sequences<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    first.iter().chain(second.iter()).cloned().collect()
}

/// Pairs elements from two slices using zip().
///
/// Stops when either slice is exhausted (uses the shorter length).
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec of tuples pairing elements from both slices
///
/// # Examples
/// ```
/// use iterator_combinators::zip_pairs;
/// assert_eq!(zip_pairs(&[1, 2, 3], &["a", "b"]), vec![(1, "a"), (2, "b")]);
/// ```
pub fn zip_pairs<T: Clone, U: Clone>(first: &[T], second: &[U]) -> Vec<(T, U)> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect()
}

/// Returns the first n elements of a slice using take().
///
/// If n is greater than the slice length, returns all elements.
///
/// # Arguments
/// * `items` - The slice to take from
/// * `n` - The number of elements to take
///
/// # Returns
/// A Vec containing at most n elements from the beginning
///
/// # Examples
/// ```
/// use iterator_combinators::take_first;
/// assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
/// ```
pub fn take_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().take(n).cloned().collect()
}

/// Skips the first n elements and returns the rest using skip().
///
/// If n is greater than or equal to the slice length, returns an empty Vec.
///
/// # Arguments
/// * `items` - The slice to skip from
/// * `n` - The number of elements to skip
///
/// # Returns
/// A Vec containing all elements after skipping the first n
///
/// # Examples
/// ```
/// use iterator_combinators::skip_first;
/// assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
/// ```
pub fn skip_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().skip(n).cloned().collect()
}

/// Reverses a sequence using rev().
///
/// # Arguments
/// * `items` - The slice to reverse
///
/// # Returns
/// A Vec with elements in reverse order
///
/// # Examples
/// ```
/// use iterator_combinators::reverse_sequence;
/// assert_eq!(reverse_sequence(&[1, 2, 3]), vec![3, 2, 1]);
/// ```
pub fn reverse_sequence<T: Clone>(items: &[T]) -> Vec<T> {
    items.iter().rev().cloned().collect()
}

/// Interleaves elements from two slices, alternating between them.
///
/// Takes one element from first, then one from second, repeating.
/// If slices have different lengths, remaining elements from the longer slice are appended.
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec with elements interleaved from both slices
///
/// # Examples
/// ```
/// use iterator_combinators::interleave;
/// assert_eq!(interleave(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    let min_len = first.len().min(second.len());

    // Interleave the common length portion
    let mut result: Vec<T> = first
        .iter()
        .zip(second.iter())
        .flat_map(|(a, b)| [a.clone(), b.clone()])
        .collect();

    // Append remaining elements from the longer slice
    if first.len() > min_len {
        result.extend(first[min_len..].iter().cloned());
    } else if second.len() > min_len {
        result.extend(second[min_len..].iter().cloned());
    }

    result
}

/// Creates pairs of consecutive elements using zip() with a skipped version.
///
/// Returns pairs of (element[i], element[i+1]) for all valid i.
///
/// # Arguments
/// * `items` - The slice to create sliding pairs from
///
/// # Returns
/// A Vec of tuples containing consecutive element pairs
///
/// # Examples
/// ```
/// use iterator_combinators::sliding_pairs;
/// assert_eq!(sliding_pairs(&[1, 2, 3, 4]), vec![(1, 2), (2, 3), (3, 4)]);
/// ```
pub fn sliding_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)> {
    items
        .iter()
        .zip(items.iter().skip(1))
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect()
}
