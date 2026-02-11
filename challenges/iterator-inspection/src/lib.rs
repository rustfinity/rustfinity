/// Returns elements with their indices using enumerate().
///
/// # Arguments
/// * `items` - The slice to enumerate
///
/// # Returns
/// A Vec of (index, element) tuples
///
/// # Examples
/// ```
/// use iterator_inspection::indexed_elements;
/// assert_eq!(indexed_elements(&["a", "b", "c"]), vec![(0, "a"), (1, "b"), (2, "c")]);
/// ```
pub fn indexed_elements<T: Clone>(items: &[T]) -> Vec<(usize, T)> {
    items.iter().enumerate().map(|(i, x)| (i, x.clone())).collect()
}

/// Finds the index of the first occurrence of target.
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to find
///
/// # Returns
/// Some(index) if found, None otherwise
///
/// # Examples
/// ```
/// use iterator_inspection::find_index;
/// assert_eq!(find_index(&[10, 20, 30, 40], &30), Some(2));
/// assert_eq!(find_index(&[10, 20, 30], &50), None);
/// ```
pub fn find_index<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    items
        .iter()
        .enumerate()
        .find(|(_, x)| *x == target)
        .map(|(i, _)| i)
}

/// Returns each element paired with the next element (or None if last).
///
/// Uses peekable() to look ahead at the next element without consuming it.
///
/// # Arguments
/// * `items` - The slice to process
///
/// # Returns
/// A Vec of (element, Option<next_element>) tuples
///
/// # Examples
/// ```
/// use iterator_inspection::elements_with_next;
/// assert_eq!(
///     elements_with_next(&[1, 2, 3]),
///     vec![(1, Some(2)), (2, Some(3)), (3, None)]
/// );
/// ```
pub fn elements_with_next<T: Clone>(items: &[T]) -> Vec<(T, Option<T>)> {
    let mut iter = items.iter().peekable();
    let mut result = Vec::new();

    while let Some(current) = iter.next() {
        let next = iter.peek().map(|x| (*x).clone());
        result.push((current.clone(), next));
    }

    result
}

/// Groups consecutive identical elements and counts them.
///
/// Uses peekable() to compare current element with next to detect group boundaries.
///
/// # Arguments
/// * `items` - The slice to group
///
/// # Returns
/// A Vec of (element, count) tuples for each consecutive group
///
/// # Examples
/// ```
/// use iterator_inspection::group_consecutive_duplicates;
/// assert_eq!(
///     group_consecutive_duplicates(&[1, 1, 2, 3, 3, 3]),
///     vec![(1, 2), (2, 1), (3, 3)]
/// );
/// ```
pub fn group_consecutive_duplicates<T: Clone + PartialEq>(items: &[T]) -> Vec<(T, usize)> {
    if items.is_empty() {
        return Vec::new();
    }

    let mut iter = items.iter().peekable();
    let mut result = Vec::new();

    while let Some(current) = iter.next() {
        let mut count = 1;

        // Count consecutive duplicates
        while iter.peek().map(|x| *x == current).unwrap_or(false) {
            iter.next();
            count += 1;
        }

        result.push((current.clone(), count));
    }

    result
}

/// Finds the first element that equals its next neighbor.
///
/// Uses peekable() to compare each element with its successor.
///
/// # Arguments
/// * `items` - The slice to search
///
/// # Returns
/// Some(element) for the first repeated element, None if no consecutive duplicates
///
/// # Examples
/// ```
/// use iterator_inspection::find_first_repeated;
/// assert_eq!(find_first_repeated(&[1, 2, 2, 3]), Some(2));
/// assert_eq!(find_first_repeated(&[1, 2, 3]), None);
/// ```
pub fn find_first_repeated<T: Clone + PartialEq>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().peekable();

    while let Some(current) = iter.next() {
        if iter.peek().map(|next| *next == current).unwrap_or(false) {
            return Some(current.clone());
        }
    }

    None
}

/// Collects elements while recording a trace of each element.
///
/// Uses inspect() to push a Debug-formatted string for each element into the trace.
///
/// # Arguments
/// * `items` - The slice to collect
/// * `trace` - A mutable Vec to record the trace
///
/// # Returns
/// A Vec containing all elements
///
/// # Examples
/// ```
/// use iterator_inspection::collect_with_trace;
/// let mut trace = Vec::new();
/// let result = collect_with_trace(&[1, 2, 3], &mut trace);
/// assert_eq!(result, vec![1, 2, 3]);
/// assert_eq!(trace, vec!["1", "2", "3"]);
/// ```
pub fn collect_with_trace<T: Clone + std::fmt::Debug>(
    items: &[T],
    trace: &mut Vec<String>,
) -> Vec<T> {
    items
        .iter()
        .inspect(|x| trace.push(format!("{:?}", x)))
        .cloned()
        .collect()
}

/// Sums numbers while recording running totals at each step.
///
/// Uses inspect() to track the running total after each addition.
///
/// # Arguments
/// * `numbers` - The slice of numbers to sum
/// * `totals` - A mutable Vec to record running totals
///
/// # Returns
/// The final sum
///
/// # Examples
/// ```
/// use iterator_inspection::sum_with_running_total;
/// let mut totals = Vec::new();
/// let sum = sum_with_running_total(&[10, 20, 30], &mut totals);
/// assert_eq!(sum, 60);
/// assert_eq!(totals, vec![10, 30, 60]);
/// ```
pub fn sum_with_running_total(numbers: &[i32], totals: &mut Vec<i32>) -> i32 {
    let mut running_total = 0;

    numbers
        .iter()
        .inspect(|&x| {
            running_total += x;
            totals.push(running_total);
        })
        .sum()
}
