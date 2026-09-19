use std::sync::atomic::{AtomicUsize, Ordering};

/// Cache statistics kept in atomics instead of behind a lock.
#[derive(Default)]
pub struct CacheStats {
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl CacheStats {
    /// Creates a fresh set of counters, both at zero.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Records one lookup: a hit if `hit` is true, otherwise a miss.
    pub fn record(&self, hit: bool) {
        // TODO: bump the right counter atomically
        unimplemented!()
    }

    /// Returns `(hits, misses)` as they stand right now.
    pub fn snapshot(&self) -> (usize, usize) {
        // TODO
        unimplemented!()
    }

    /// Returns `(hits, misses)` and sets both counters back to zero.
    pub fn drain(&self) -> (usize, usize) {
        // TODO: read and reset each counter in a single atomic step
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let stats = CacheStats::new();
    stats.record(true);
    stats.record(false);
    println!("{:?} {:?}", stats.snapshot(), stats.drain());
}
