use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// An unbounded queue that a consumer can block on until work arrives.
#[derive(Debug, Default)]
pub struct Queue<T> {
    // TODO: hold the items behind a Mutex, and add a Condvar to signal on
    items: Mutex<VecDeque<T>>,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// How many items are waiting.
    pub fn len(&self) -> usize {
        // TODO
        unimplemented!()
    }

    /// Whether the queue is currently empty.
    pub fn is_empty(&self) -> bool {
        // TODO
        unimplemented!()
    }

    /// Adds an item and wakes one waiting consumer.
    pub fn push(&self, item: T) {
        // TODO
        unimplemented!()
    }

    /// Removes the oldest item, blocking until one exists.
    pub fn pop(&self) -> T {
        // TODO: lock, then wait in a loop while the queue is empty
        unimplemented!()
    }

    /// Like `pop`, but gives up after `timeout` and returns `None`.
    pub fn pop_timeout(&self, timeout: Duration) -> Option<T> {
        // TODO: same shape as `pop`, but with `wait_timeout`
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let queue = Queue::new();
    queue.push("first");
    println!("{}", queue.pop());
    println!("{:?}", queue.pop_timeout(Duration::from_millis(5)));
}
