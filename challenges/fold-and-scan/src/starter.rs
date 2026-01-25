/// Sums all numbers using fold().
///
/// # Arguments
/// * `numbers` - The slice of numbers to sum
///
/// # Returns
/// The sum of all numbers (0 for empty slice)
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    // TODO: Use iter().fold() to sum all numbers
    // fold() takes an initial value and a closure that combines the accumulator with each element
    unimplemented!()
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
pub fn product_with_fold(numbers: &[i32]) -> i64 {
    // TODO: Use iter().fold() to multiply all numbers
    // Start with 1i64 as the initial value
    unimplemented!()
}

/// Joins strings with a separator using fold().
///
/// # Arguments
/// * `strings` - The slice of string slices to join
/// * `separator` - The separator to place between strings
///
/// # Returns
/// A single string with all inputs joined by the separator
pub fn concat_strings(strings: &[&str], separator: &str) -> String {
    // TODO: Use enumerate().fold() to concatenate strings with a separator
    // Hint: Use enumerate to track the index, and only add separator when i > 0
    unimplemented!()
}

/// Returns cumulative sums using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the sum of all elements up to that position
pub fn running_sum(numbers: &[i32]) -> Vec<i32> {
    // TODO: Use iter().scan() to compute running sums
    // scan() takes an initial state and a closure that receives &mut state and each element
    // The closure returns Option<output> - return Some(value) to yield values
    unimplemented!()
}

/// Returns running maximum values using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the maximum of all elements up to that position
pub fn running_max(numbers: &[i32]) -> Vec<i32> {
    // TODO: Use scan() to track the running maximum
    // Hint: You can use Option<i32> as state to handle the first element
    unimplemented!()
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
pub fn take_while_sum_under(numbers: &[i32], limit: i32) -> Vec<i32> {
    // TODO: Use scan() and return None to stop iteration early
    // Check if adding the current number would exceed the limit
    unimplemented!()
}

/// Counts occurrences of target in items using fold().
///
/// # Arguments
/// * `items` - The slice to search
/// * `target` - The value to count
///
/// # Returns
/// The number of times target appears in items
pub fn count_occurrences<T: PartialEq>(items: &[T], target: &T) -> usize {
    // TODO: Use fold() to count how many times target appears
    unimplemented!()
}

/// Returns running average at each position using scan().
///
/// # Arguments
/// * `numbers` - The slice of numbers
///
/// # Returns
/// A Vec where each element is the average of all elements up to that position
pub fn running_average(numbers: &[f64]) -> Vec<f64> {
    // TODO: Use scan() with a tuple state (sum, count) to track running average
    // Divide sum by count at each step
    unimplemented!()
}

// Example usage
pub fn main() {
    // sum_with_fold
    let sum = sum_with_fold(&[1, 2, 3, 4, 5]);
    println!("Sum: {}", sum); // Expected: 15

    // product_with_fold
    let product = product_with_fold(&[1, 2, 3, 4]);
    println!("Product: {}", product); // Expected: 24

    // concat_strings
    let joined = concat_strings(&["Hello", "World"], " ");
    println!("Joined: {}", joined); // Expected: "Hello World"

    // running_sum
    let sums = running_sum(&[1, 2, 3, 4]);
    println!("Running sums: {:?}", sums); // Expected: [1, 3, 6, 10]

    // running_max
    let maxes = running_max(&[3, 1, 4, 1, 5]);
    println!("Running max: {:?}", maxes); // Expected: [3, 3, 4, 4, 5]

    // take_while_sum_under
    let taken = take_while_sum_under(&[1, 2, 3, 4, 5], 10);
    println!("Taken while sum < 10: {:?}", taken); // Expected: [1, 2, 3]

    // count_occurrences
    let count = count_occurrences(&[1, 2, 1, 3, 1], &1);
    println!("Count of 1: {}", count); // Expected: 3

    // running_average
    let averages = running_average(&[2.0, 4.0, 6.0]);
    println!("Running averages: {:?}", averages); // Expected: [2.0, 3.0, 4.0]
}
