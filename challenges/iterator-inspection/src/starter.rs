/// Returns elements with their indices using enumerate().
///
/// # Arguments
/// * `items` - The slice to enumerate
///
/// # Returns
/// A Vec of (index, element) tuples
pub fn indexed_elements<T: Clone>(items: &[T]) -> Vec<(usize, T)> {
    // TODO: Implement this function
    unimplemented!()
}

/// Finds the index of the first occurrence of target.
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to find
///
/// # Returns
/// Some(index) if found, None otherwise
pub fn find_index<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn elements_with_next<T: Clone>(items: &[T]) -> Vec<(T, Option<T>)> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn group_consecutive_duplicates<T: Clone + PartialEq>(items: &[T]) -> Vec<(T, usize)> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn find_first_repeated<T: Clone + PartialEq>(items: &[T]) -> Option<T> {
    // TODO: Implement this function
    unimplemented!()
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
pub fn collect_with_trace<T: Clone + std::fmt::Debug>(
    items: &[T],
    trace: &mut Vec<String>,
) -> Vec<T> {
    // TODO: Use .inspect() to push a formatted string to trace for each element
    // Use format!("{:?}", x) to create the debug string
    // Then collect the elements
    unimplemented!()
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
pub fn sum_with_running_total(numbers: &[i32], totals: &mut Vec<i32>) -> i32 {
    // TODO: Use a mutable running_total variable
    // Use inspect() to update the running total and push to totals
    // Then sum the iterator
    unimplemented!()
}

pub fn main() {
    // indexed_elements example
    let indexed = indexed_elements(&["a", "b", "c"]);
    println!("indexed_elements([\"a\", \"b\", \"c\"]) = {:?}", indexed);

    // find_index example
    let index = find_index(&[10, 20, 30, 40], &30);
    println!("find_index([10, 20, 30, 40], &30) = {:?}", index);

    // elements_with_next example
    let with_next = elements_with_next(&[1, 2, 3]);
    println!("elements_with_next([1, 2, 3]) = {:?}", with_next);

    // group_consecutive_duplicates example
    let grouped = group_consecutive_duplicates(&[1, 1, 2, 3, 3, 3]);
    println!(
        "group_consecutive_duplicates([1, 1, 2, 3, 3, 3]) = {:?}",
        grouped
    );

    // find_first_repeated example
    let repeated = find_first_repeated(&[1, 2, 2, 3]);
    println!("find_first_repeated([1, 2, 2, 3]) = {:?}", repeated);

    // collect_with_trace example
    let mut trace = Vec::new();
    let collected = collect_with_trace(&[1, 2, 3], &mut trace);
    println!("collect_with_trace([1, 2, 3]) = {:?}", collected);
    println!("trace = {:?}", trace);

    // sum_with_running_total example
    let mut totals = Vec::new();
    let sum = sum_with_running_total(&[10, 20, 30], &mut totals);
    println!("sum_with_running_total([10, 20, 30]) = {}", sum);
    println!("totals = {:?}", totals);
}
