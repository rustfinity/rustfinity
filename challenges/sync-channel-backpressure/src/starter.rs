use std::sync::mpsc;
use std::thread;

/// Moves every item from a producer thread to this thread through a
/// bounded channel, preserving order.
pub fn drain(items: Vec<u64>, capacity: usize) -> Vec<u64> {
    // TODO: build a bounded channel, send from a worker thread, and
    // collect on this one
    unimplemented!()
}

/// Pushes items with `try_send` and reports how many the channel took
/// before it filled up.
pub fn fill_without_receiving(items: Vec<u64>, capacity: usize) -> usize {
    // TODO: keep the receiver alive, and stop counting at the first
    // TrySendError::Full
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", drain(vec![1, 2, 3], 1));
    println!("{}", fill_without_receiving(vec![1, 2, 3, 4], 2));
}
