use std::sync::mpsc;
use std::thread;

/// Runs one thread per chunk and gathers every value through a single
/// channel, returned in ascending order.
pub fn collect_all(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: clone a sender into every worker, drop the original, then
    // drain the receiver
    unimplemented!()
}

/// Sums each chunk, keeping the results lined up with the input.
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    // TODO: tag every message with the worker index so the consumer can
    // put it back in the right slot
    unimplemented!()
}

// Example usage
pub fn main() {
    let chunks = vec![vec![3, 1], vec![2], vec![]];

    println!("{:?}", collect_all(chunks.clone()));
    println!("{:?}", sum_by_worker(chunks));
}
