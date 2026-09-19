use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// An unbounded queue that a consumer can block on until work arrives.
///
/// The pair is always the same: a `Mutex` holding the state, and a
/// `Condvar` used to announce that the state changed. The condvar owns no
/// data of its own; it only parks and wakes threads.
#[derive(Debug, Default)]
pub struct Queue<T> {
    items: Mutex<VecDeque<T>>,
    ready: Condvar,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    ///
    /// let queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub fn new() -> Self {
        Queue {
            items: Mutex::new(VecDeque::new()),
            ready: Condvar::new(),
        }
    }

    /// How many items are waiting.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push("job");
    /// assert_eq!(queue.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.items.lock().unwrap().len()
    }

    /// Whether the queue is currently empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    ///
    /// let queue: Queue<u8> = Queue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds an item and wakes one waiting consumer.
    ///
    /// The guard is dropped before `notify_one` is reached, so a woken
    /// thread does not immediately block again on a lock we still hold.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push(7);
    /// assert_eq!(queue.pop(), 7);
    /// ```
    pub fn push(&self, item: T) {
        self.items.lock().unwrap().push_back(item);
        self.ready.notify_one();
    }

    /// Removes the oldest item, blocking until one exists.
    ///
    /// The `while` loop is not optional. `wait` can return without any
    /// matching `notify` at all (a spurious wakeup), and even a real
    /// notification may be stolen by another consumer that grabbed the
    /// lock first. Re-checking the predicate after every wake is the only
    /// correct shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push(1);
    /// queue.push(2);
    /// assert_eq!(queue.pop(), 1);
    /// assert_eq!(queue.pop(), 2);
    /// ```
    pub fn pop(&self) -> T {
        let mut items = self.items.lock().unwrap();

        while items.is_empty() {
            items = self.ready.wait(items).unwrap();
        }

        items.pop_front().unwrap()
    }

    /// Like [`Queue::pop`], but gives up after `timeout` and returns
    /// `None`.
    ///
    /// `wait_timeout` hands back the guard plus a `WaitTimeoutResult`.
    /// Checking `timed_out()` alone is not enough, because the loop may
    /// go round several times; the predicate is still what decides.
    ///
    /// # Examples
    ///
    /// ```
    /// use condvar_signaling::Queue;
    /// use std::time::Duration;
    ///
    /// let queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.pop_timeout(Duration::from_millis(5)), None);
    ///
    /// queue.push(3);
    /// assert_eq!(queue.pop_timeout(Duration::from_millis(5)), Some(3));
    /// ```
    pub fn pop_timeout(&self, timeout: Duration) -> Option<T> {
        let mut items = self.items.lock().unwrap();

        while items.is_empty() {
            let (guard, result) = self.ready.wait_timeout(items, timeout).unwrap();
            items = guard;

            if items.is_empty() && result.timed_out() {
                return None;
            }
        }

        items.pop_front()
    }
}
