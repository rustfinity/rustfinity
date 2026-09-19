use std::sync::Mutex;

/// A counter that keeps working after a thread panics while holding the lock.
pub struct ResilientCounter {
    value: Mutex<i64>,
}

impl ResilientCounter {
    /// Creates a counter starting at `start`.
    pub fn new(start: i64) -> Self {
        // TODO: build the counter
        unimplemented!()
    }

    /// Runs `f` on the counter while holding the lock, and returns its result.
    ///
    /// The lock is taken even when the mutex is poisoned.
    pub fn with<R>(&self, f: impl FnOnce(&mut i64) -> R) -> R {
        // TODO: take the lock, recovering it if it is poisoned, then call `f`
        unimplemented!()
    }

    /// Reads the counter, even if the mutex is poisoned.
    pub fn get(&self) -> i64 {
        // TODO
        unimplemented!()
    }

    /// Reports whether some thread has panicked while holding the lock.
    pub fn is_poisoned(&self) -> bool {
        // TODO
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let counter = ResilientCounter::new(0);
    counter.with(|value| *value += 3);
    println!("{} {}", counter.get(), counter.is_poisoned());
}
