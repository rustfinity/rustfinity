use std::thread::{self, JoinHandle};

/// Sums a list of numbers on a separate thread and returns the total.
pub fn sum_in_thread(values: Vec<u64>) -> u64 {
    // TODO: spawn a thread that sums `values`, then join it
    unimplemented!()
}

/// Starts a thread that counts and returns the handle immediately.
///
/// The thread adds up `start + (start + 1) + ... + (start + steps - 1)`.
pub fn spawn_counter(start: u64, steps: u64) -> JoinHandle<u64> {
    // TODO: spawn the thread and return its handle without joining
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("sum: {}", sum_in_thread(vec![1, 2, 3, 4]));

    let handle = spawn_counter(10, 3);
    println!("counter: {}", handle.join().unwrap());
}
