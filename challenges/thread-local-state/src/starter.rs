use std::cell::{Cell, RefCell};
use std::thread;

thread_local! {
    // TODO: one Cell<u64> counter and one RefCell<Vec<String>> log,
    // both per thread
}

/// Increments the calling thread's counter and returns the new value.
pub fn bump() -> u64 {
    // TODO
    unimplemented!()
}

/// Appends an event to the calling thread's log.
pub fn record(event: &str) {
    // TODO
    unimplemented!()
}

/// Returns a copy of the calling thread's log.
pub fn events() -> Vec<String> {
    // TODO
    unimplemented!()
}

/// Runs one thread per entry, bumping the counter that many times, and
/// returns each thread's final count.
pub fn counts_per_thread(bumps: &[u64]) -> Vec<u64> {
    // TODO: spawn a thread per entry, and remember that each one starts
    // its own counter at 0
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", counts_per_thread(&[3, 1, 2]));

    record("hello");
    println!("{:?}", events());
}
