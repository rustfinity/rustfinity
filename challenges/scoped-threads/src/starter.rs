use std::thread;

/// Finds the largest value by scanning both halves in parallel.
pub fn parallel_max(data: &[i32]) -> Option<i32> {
    // TODO: split `data` in half and compare both halves inside a
    // thread::scope, without cloning or using Arc
    unimplemented!()
}

/// Splits values into evens and odds using two scoped threads.
pub fn split_evens_odds(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    // TODO: fill both vectors from inside a thread::scope

    (evens, odds)
}

// Example usage
pub fn main() {
    println!("{:?}", parallel_max(&[3, 9, 1, 4]));
    println!("{:?}", split_evens_odds(&[1, 2, 3, 4]));
}
