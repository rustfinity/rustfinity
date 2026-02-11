/// Flattens a vector of vectors into a single vector.
///
/// # Arguments
///
/// * `nested` - A vector of vectors to flatten
///
/// # Returns
///
/// A single vector containing all elements from all inner vectors
pub fn flatten_nested(nested: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Extracts all Some values from a vector of Options, discarding None values.
///
/// # Arguments
///
/// * `options` - A vector of Option<i32> values
///
/// # Returns
///
/// A vector containing only the values that were Some
pub fn flatten_options(options: Vec<Option<i32>>) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Extracts all Ok values from a vector of Results, discarding Err values.
///
/// # Arguments
///
/// * `results` - A vector of Result<i32, &str> values
///
/// # Returns
///
/// A vector containing only the values that were Ok
pub fn flatten_results(results: Vec<Result<i32, &str>>) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Gets all characters from a slice of string slices.
///
/// # Arguments
///
/// * `words` - A slice of string slices
///
/// # Returns
///
/// A vector of all characters from all words concatenated
pub fn chars_from_words(words: &[&str]) -> Vec<char> {
    // TODO: Implement this function
    unimplemented!()
}

/// Expands (start, end) tuples into sequences (inclusive ranges).
///
/// # Arguments
///
/// * `ranges` - A slice of (start, end) tuples representing inclusive ranges
///
/// # Returns
///
/// A vector containing all integers from all ranges
pub fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Flattens only the outer layer of a triply nested vector.
///
/// # Arguments
///
/// * `nested` - A vector of vectors of vectors
///
/// # Returns
///
/// A vector of vectors (one level less nested)
pub fn flatten_to_depth_one(nested: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    // TODO: Implement this function
    unimplemented!()
}

/// Splits each line into words and collects all words.
///
/// # Arguments
///
/// * `lines` - A slice of string slices, each representing a line of text
///
/// # Returns
///
/// A vector of all words from all lines
pub fn words_from_lines(lines: &[&str]) -> Vec<String> {
    // TODO: Implement this function
    unimplemented!()
}

/// Flattens a nested vector and then filters with a predicate.
///
/// # Arguments
///
/// * `nested` - A vector of vectors to flatten
/// * `predicate` - A function that returns true for elements to keep
///
/// # Returns
///
/// A flattened vector containing only elements that satisfy the predicate
pub fn flatten_and_filter<T, F>(nested: Vec<Vec<T>>, predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    // TODO: Implement this function
    unimplemented!()
}

// Example usage
pub fn main() {
    // flatten_nested
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat = flatten_nested(nested);
    println!("Flattened: {:?}", flat);

    // flatten_options
    let options = vec![Some(1), None, Some(3), None, Some(5)];
    let values = flatten_options(options);
    println!("From options: {:?}", values);

    // chars_from_words
    let words = &["hello", "world"];
    let chars = chars_from_words(words);
    println!("Characters: {:?}", chars);

    // expand_ranges
    let ranges = &[(1, 3), (10, 12)];
    let expanded = expand_ranges(ranges);
    println!("Expanded ranges: {:?}", expanded);

    // words_from_lines
    let lines = &["hello world", "foo bar baz"];
    let words = words_from_lines(lines);
    println!("Words from lines: {:?}", words);
}
