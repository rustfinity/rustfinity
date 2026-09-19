use std::thread::{self, JoinHandle};

/// Sums a list of numbers on a separate thread and returns the total.
///
/// The vector is moved into the spawned thread, the thread computes the
/// sum, and `join()` hands the value back to the calling thread.
///
/// # Examples
///
/// ```
/// use spawning_threads::sum_in_thread;
///
/// assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
/// ```
pub fn sum_in_thread(values: Vec<u64>) -> u64 {
    let handle = thread::spawn(move || values.iter().sum::<u64>());

    handle.join().unwrap()
}

/// Starts a thread that counts and returns the handle immediately.
///
/// The returned `JoinHandle<u64>` is a value like any other: the caller
/// can keep working and only block on `join()` when the result is
/// actually needed.
///
/// The thread adds up `start + (start + 1) + ... + (start + steps - 1)`.
/// With `steps` of 0 the sum is 0.
///
/// # Examples
///
/// ```
/// use spawning_threads::spawn_counter;
///
/// let handle = spawn_counter(10, 3);
/// assert_eq!(handle.join().unwrap(), 33);
/// ```
pub fn spawn_counter(start: u64, steps: u64) -> JoinHandle<u64> {
    thread::spawn(move || (0..steps).map(|i| start + i).sum())
}
