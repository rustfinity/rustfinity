/// Combines two slices into a single Vec using chain().
///
/// # Arguments
/// * `first` - The first slice
/// * `second` - The second slice
///
/// # Returns
/// A Vec containing all elements from first, followed by all elements from second
pub fn chain_sequences<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn zip_pairs<T: Clone, U: Clone>(first: &[T], second: &[U]) -> Vec<(T, U)> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn take_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn skip_first<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
}

/// Reverses a sequence using rev().
///
/// # Arguments
/// * `items` - The slice to reverse
///
/// # Returns
/// A Vec with elements in reverse order
pub fn reverse_sequence<T: Clone>(items: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn sliding_pairs<T: Clone>(items: &[T]) -> Vec<(T, T)> {
    // TODO: Implement this function
    unimplemented!()
}

pub fn main() {
    // chain_sequences example
    let result = chain_sequences(&[1, 2], &[3, 4]);
    println!("chain_sequences([1, 2], [3, 4]) = {:?}", result);

    // zip_pairs example
    let result = zip_pairs(&[1, 2, 3], &["a", "b"]);
    println!("zip_pairs([1, 2, 3], [\"a\", \"b\"]) = {:?}", result);

    // take_first example
    let result = take_first(&[1, 2, 3, 4, 5], 3);
    println!("take_first([1, 2, 3, 4, 5], 3) = {:?}", result);

    // skip_first example
    let result = skip_first(&[1, 2, 3, 4, 5], 2);
    println!("skip_first([1, 2, 3, 4, 5], 2) = {:?}", result);

    // reverse_sequence example
    let result = reverse_sequence(&[1, 2, 3]);
    println!("reverse_sequence([1, 2, 3]) = {:?}", result);

    // interleave example
    let result = interleave(&[1, 3, 5], &[2, 4, 6]);
    println!("interleave([1, 3, 5], [2, 4, 6]) = {:?}", result);

    // sliding_pairs example
    let result = sliding_pairs(&[1, 2, 3, 4]);
    println!("sliding_pairs([1, 2, 3, 4]) = {:?}", result);
}
