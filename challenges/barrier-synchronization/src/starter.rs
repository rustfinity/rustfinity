use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Barrier;
use std::thread;

/// Runs one "read everybody, then write everybody" step per round.
///
/// Every cell is replaced by itself plus its right neighbour, wrapping at
/// the end. One thread owns one cell for the whole run.
pub fn run_rounds(cells: Vec<i64>, rounds: usize) -> Vec<i64> {
    // TODO: give every cell a thread, and use two barrier waits per round
    // so that no thread writes while another is still reading
    unimplemented!()
}

/// Counts how many threads were told they were the leader of a barrier.
///
/// Runs `parties` threads through `rounds` barriers.
pub fn count_leaders(parties: usize, rounds: usize) -> usize {
    // TODO: count the `wait()` results whose `is_leader()` is true
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", run_rounds(vec![1, 2, 3], 2));
    println!("{}", count_leaders(4, 3));
}
