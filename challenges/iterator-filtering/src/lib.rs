/// Filters a slice to return only even numbers.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values to filter
///
/// # Returns
///
/// A Vec containing only the even numbers from the input
///
/// # Examples
///
/// ```
/// use iterator_filtering::filter_even;
/// assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
/// assert_eq!(filter_even(&[1, 3, 5]), vec![]);
/// ```
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect()
}

/// Filters a slice using a custom predicate function.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values to filter
/// * `predicate` - A function that returns true for elements to keep
///
/// # Returns
///
/// A Vec containing only the numbers that satisfy the predicate
///
/// # Examples
///
/// ```
/// use iterator_filtering::filter_by_predicate;
/// assert_eq!(filter_by_predicate(&[1, 2, 3, 4, 5], |&x| x > 3), vec![4, 5]);
/// assert_eq!(filter_by_predicate(&[1, 2, 3], |&x| x % 2 == 1), vec![1, 3]);
/// ```
pub fn filter_by_predicate<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    numbers.iter().filter(|&x| predicate(x)).cloned().collect()
}

/// Parses a slice of strings to integers, keeping only valid parses.
///
/// # Arguments
///
/// * `strings` - A slice of string slices to parse
///
/// # Returns
///
/// A Vec containing successfully parsed integers
///
/// # Examples
///
/// ```
/// use iterator_filtering::parse_valid_numbers;
/// assert_eq!(parse_valid_numbers(&["1", "hello", "3", "world"]), vec![1, 3]);
/// assert_eq!(parse_valid_numbers(&["42", "-5", "0"]), vec![42, -5, 0]);
/// ```
pub fn parse_valid_numbers(strings: &[&str]) -> Vec<i32> {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

/// Generic filter_map operation that applies a function returning Option<U>.
///
/// # Arguments
///
/// * `items` - A slice of items to process
/// * `f` - A function that returns Some(value) for items to keep, None to skip
///
/// # Returns
///
/// A Vec of transformed values where the function returned Some
///
/// # Examples
///
/// ```
/// use iterator_filtering::filter_map_with;
/// let result: Vec<i32> = filter_map_with(&[1, 2, 3, 4], |x| if x % 2 == 0 { Some(x * 2) } else { None });
/// assert_eq!(result, vec![4, 8]);
/// ```
pub fn filter_map_with<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    T: Clone,
    F: Fn(T) -> Option<U>,
{
    items.iter().cloned().filter_map(f).collect()
}

/// Takes elements from the beginning while they are positive (> 0).
///
/// Stops at the first non-positive element.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values
///
/// # Returns
///
/// A Vec of positive numbers from the start of the slice
///
/// # Examples
///
/// ```
/// use iterator_filtering::take_while_positive;
/// assert_eq!(take_while_positive(&[3, 5, -1, 2, 4]), vec![3, 5]);
/// assert_eq!(take_while_positive(&[-1, 2, 3]), vec![]);
/// ```
pub fn take_while_positive(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .take_while(|&&x| x > 0)
        .cloned()
        .collect()
}

/// Skips elements from the beginning while they are negative (< 0).
///
/// After the first non-negative element, takes all remaining elements.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values
///
/// # Returns
///
/// A Vec starting from the first non-negative element
///
/// # Examples
///
/// ```
/// use iterator_filtering::skip_while_negative;
/// assert_eq!(skip_while_negative(&[-3, -1, 2, -4, 5]), vec![2, -4, 5]);
/// assert_eq!(skip_while_negative(&[1, -2, -3]), vec![1, -2, -3]);
/// ```
pub fn skip_while_negative(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .skip_while(|&&x| x < 0)
        .cloned()
        .collect()
}

/// Filters numbers to keep only those within a range [min, max] inclusive.
///
/// # Arguments
///
/// * `numbers` - A slice of i32 values to filter
/// * `min` - The minimum value (inclusive)
/// * `max` - The maximum value (inclusive)
///
/// # Returns
///
/// A Vec of numbers within the specified range
///
/// # Examples
///
/// ```
/// use iterator_filtering::filter_in_range;
/// assert_eq!(filter_in_range(&[1, 5, 10, 15, 20], 5, 15), vec![5, 10, 15]);
/// assert_eq!(filter_in_range(&[1, 2, 3], 10, 20), vec![]);
/// ```
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| x >= min && x <= max)
        .cloned()
        .collect()
}

/// Finds the first element that matches a predicate.
///
/// # Arguments
///
/// * `items` - A slice of items to search
/// * `predicate` - A function that returns true for the desired element
///
/// # Returns
///
/// Some(element) if found, None otherwise
///
/// # Examples
///
/// ```
/// use iterator_filtering::first_matching;
/// assert_eq!(first_matching(&[1, 2, 3, 4, 5], |&x| x > 3), Some(4));
/// assert_eq!(first_matching(&[1, 2, 3], |&x| x > 10), None);
/// ```
pub fn first_matching<T: Clone, F>(items: &[T], predicate: F) -> Option<T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|&x| predicate(x)).cloned()
}
