/// Sums all numbers using fold().
///
/// # Arguments
/// * `numbers` - The slice of numbers to sum
///
/// # Returns
/// The sum of all numbers (0 for empty slice)
///
/// # Examples
/// ```
/// use fold_and_scan::sum_with_fold;
/// assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);
/// assert_eq!(sum_with_fold(&[]), 0);
/// ```
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, &x| acc + x)
}

/// Multiplies all numbers using fold().
///
/// Uses i64 to handle larger products safely.
///
/// # Arguments
/// * `numbers` - The slice of numbers to multiply
///
/// # Returns
/// The product of all numbers (1 for empty slice)
///
/// # Examples
/// ```
/// use fold_and_scan::product_with_fold;
/// assert_eq!(product_with_fold(&[1, 2, 3, 4]), 24);
/// assert_eq!(product_with_fold(&[]), 1);
/// ```
pub fn product_with_fold(numbers: &[i32]) -> i64 {
    numbers.iter().fold(1i64, |acc, &x| acc * x as i64)
}

/// Joins strings with a separator using fold().
///
/// # Arguments
/// * `strings` - The slice of string slices to join
/// * `separator` - The separator to place between strings
///
/// # Returns
/// A single string with all inputs joined by the separator
///
/// # Examples
/// ```
/// use fold_and_scan::concat_strings;
/// assert_eq!(concat_strings(&["a", "b", "c"], "-"), "a-b-c");
/// assert_eq!(concat_strings(&[], "-"), "");
/// ```
pub fn concat_strings(strings: &[&str], separator: &str) -> String {
    strings
        .iter()
        .enumerate()
        .fold(String::new(), |acc, (i, &s)| {
            if i == 0 {
                s.to_string()
            } else {
                format!("{}{}{}", acc, separator, s)
            }
        })
}

/// Returns cumulative sums using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the sum of all elements up to that position
///
/// # Examples
/// ```
/// use fold_and_scan::running_sum;
/// assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
/// assert_eq!(running_sum(&[]), vec![]);
/// ```
pub fn running_sum(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}

/// Returns running maximum values using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the maximum of all elements up to that position
///
/// # Examples
/// ```
/// use fold_and_scan::running_max;
/// assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);
/// assert_eq!(running_max(&[]), vec![]);
/// ```
pub fn running_max(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .scan(None, |state: &mut Option<i32>, &x| {
            *state = Some(state.map_or(x, |max| max.max(x)));
            *state
        })
        .collect()
}

/// Takes numbers while running sum stays strictly under limit.
///
/// Stops before adding a number that would make the sum >= limit.
///
/// # Arguments
/// * `numbers` - The slice of numbers
/// * `limit` - The exclusive upper limit for the running sum
///
/// # Returns
/// A Vec of numbers taken while sum stays under limit
///
/// # Examples
/// ```
/// use fold_and_scan::take_while_sum_under;
/// assert_eq!(take_while_sum_under(&[1, 2, 3, 4, 5], 10), vec![1, 2, 3]);
/// assert_eq!(take_while_sum_under(&[5, 5, 5], 10), vec![5]);
/// ```
pub fn take_while_sum_under(numbers: &[i32], limit: i32) -> Vec<i32> {
    numbers
        .iter()
        .scan(0, |state, &x| {
            let new_sum = *state + x;
            if new_sum < limit {
                *state = new_sum;
                Some(x)
            } else {
                None
            }
        })
        .collect()
}

/// Counts occurrences of target in items using fold().
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to count
///
/// # Returns
/// The number of times target appears in items
///
/// # Examples
/// ```
/// use fold_and_scan::count_occurrences;
/// assert_eq!(count_occurrences(&[1, 2, 1, 3, 1], &1), 3);
/// assert_eq!(count_occurrences(&["a", "b", "a"], &"a"), 2);
/// ```
pub fn count_occurrences<T: PartialEq>(items: &[T], target: &T) -> usize {
    items.iter().fold(0, |count, item| {
        if item == target {
            count + 1
        } else {
            count
        }
    })
}

/// Returns running average at each position using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the average of all elements up to that position
///
/// # Examples
/// ```
/// use fold_and_scan::running_average;
/// assert_eq!(running_average(&[2.0, 4.0, 6.0]), vec![2.0, 3.0, 4.0]);
/// assert_eq!(running_average(&[]), vec![]);
/// ```
pub fn running_average(numbers: &[f64]) -> Vec<f64> {
    numbers
        .iter()
        .scan((0.0, 0usize), |state, &x| {
            state.0 += x;
            state.1 += 1;
            Some(state.0 / state.1 as f64)
        })
        .collect()
}
