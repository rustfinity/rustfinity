/// Filters a slice to return only even numbers.
///
/// Use the `.filter()` method with a predicate that checks for evenness.
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Filters a slice using a custom predicate function.
///
/// The predicate takes a reference to an i32 and returns true for elements to keep.
pub fn filter_by_predicate<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    // TODO: Implement this function
    unimplemented!()
}

/// Parses a slice of strings to integers, keeping only valid parses.
///
/// Use filter_map to combine parsing and filtering in one step.
pub fn parse_valid_numbers(strings: &[&str]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Generic filter_map operation that applies a function returning Option<U>.
///
/// Items where f returns None are filtered out; Some values are collected.
pub fn filter_map_with<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    T: Clone,
    F: Fn(T) -> Option<U>,
{
    // TODO: Implement this function
    unimplemented!()
}

/// Takes elements from the beginning while they are positive (> 0).
///
/// Stops at the first non-positive element.
pub fn take_while_positive(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Skips elements from the beginning while they are negative (< 0).
///
/// After the first non-negative element, takes all remaining elements.
pub fn skip_while_negative(numbers: &[i32]) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Filters numbers to keep only those within a range [min, max] inclusive.
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    // TODO: Implement this function
    unimplemented!()
}

/// Finds the first element that matches a predicate.
///
/// Returns Some(element) if found, None otherwise.
pub fn first_matching<T: Clone, F>(items: &[T], predicate: F) -> Option<T>
where
    F: Fn(&T) -> bool,
{
    // TODO: Implement this function
    unimplemented!()
}

// Example usage
pub fn main() {
    // filter_even
    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("Even numbers: {:?}", filter_even(&numbers));

    // filter_by_predicate
    println!("Numbers > 3: {:?}", filter_by_predicate(&numbers, |&x| x > 3));

    // parse_valid_numbers
    let strings = vec!["1", "hello", "3", "world"];
    println!("Valid numbers: {:?}", parse_valid_numbers(&strings));

    // filter_map_with
    let doubled: Vec<i32> = filter_map_with(&[1, 2, 3, 4], |x| {
        if x % 2 == 0 { Some(x * 2) } else { None }
    });
    println!("Doubled evens: {:?}", doubled);

    // take_while_positive
    let mixed = vec![3, 5, -1, 2, 4];
    println!("Take while positive: {:?}", take_while_positive(&mixed));

    // skip_while_negative
    let negative_start = vec![-3, -1, 2, -4, 5];
    println!("Skip while negative: {:?}", skip_while_negative(&negative_start));

    // filter_in_range
    let range_nums = vec![1, 5, 10, 15, 20];
    println!("In range [5, 15]: {:?}", filter_in_range(&range_nums, 5, 15));

    // first_matching
    println!("First > 3: {:?}", first_matching(&numbers, |&x| x > 3));
    println!("First > 100: {:?}", first_matching(&numbers, |&x| x > 100));
}
