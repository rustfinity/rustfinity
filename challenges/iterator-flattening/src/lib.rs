/// Flattens a vector of vectors into a single vector.
///
/// # Arguments
///
/// * `nested` - A vector of vectors to flatten
///
/// # Returns
///
/// A single vector containing all elements from all inner vectors
///
/// # Examples
///
/// ```
/// use iterator_flattening::flatten_nested;
/// assert_eq!(flatten_nested(vec![vec![1, 2], vec![3], vec![4, 5, 6]]), vec![1, 2, 3, 4, 5, 6]);
/// assert_eq!(flatten_nested(vec![vec![], vec![1], vec![]]), vec![1]);
/// ```
pub fn flatten_nested(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::flatten_options;
/// assert_eq!(flatten_options(vec![Some(1), None, Some(3)]), vec![1, 3]);
/// assert_eq!(flatten_options(vec![None, None]), vec![]);
/// ```
pub fn flatten_options(options: Vec<Option<i32>>) -> Vec<i32> {
    options.into_iter().flatten().collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::flatten_results;
/// let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
/// assert_eq!(flatten_results(results), vec![1, 3]);
/// ```
pub fn flatten_results(results: Vec<Result<i32, &str>>) -> Vec<i32> {
    results.into_iter().flatten().collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::chars_from_words;
/// assert_eq!(chars_from_words(&["hi", "there"]), vec!['h', 'i', 't', 'h', 'e', 'r', 'e']);
/// assert_eq!(chars_from_words(&["a", "b"]), vec!['a', 'b']);
/// ```
pub fn chars_from_words(words: &[&str]) -> Vec<char> {
    words.iter().flat_map(|s| s.chars()).collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::expand_ranges;
/// assert_eq!(expand_ranges(&[(1, 3), (5, 6)]), vec![1, 2, 3, 5, 6]);
/// assert_eq!(expand_ranges(&[(5, 5)]), vec![5]);
/// ```
pub fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    ranges.iter().flat_map(|&(start, end)| start..=end).collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::flatten_to_depth_one;
/// let deep = vec![vec![vec![1, 2], vec![3]], vec![vec![4]]];
/// assert_eq!(flatten_to_depth_one(deep), vec![vec![1, 2], vec![3], vec![4]]);
/// ```
pub fn flatten_to_depth_one(nested: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    nested.into_iter().flatten().collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::words_from_lines;
/// assert_eq!(words_from_lines(&["hello world", "foo bar"]), vec!["hello", "world", "foo", "bar"]);
/// assert_eq!(words_from_lines(&["one"]), vec!["one"]);
/// ```
pub fn words_from_lines(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(String::from)
        .collect()
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
///
/// # Examples
///
/// ```
/// use iterator_flattening::flatten_and_filter;
/// let nested = vec![vec![1, 2, 3], vec![4, 5, 6]];
/// assert_eq!(flatten_and_filter(nested, |&x| x % 2 == 0), vec![2, 4, 6]);
/// ```
pub fn flatten_and_filter<T, F>(nested: Vec<Vec<T>>, predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    nested
        .into_iter()
        .flatten()
        .filter(predicate)
        .collect()
}
