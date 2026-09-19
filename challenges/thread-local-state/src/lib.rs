use std::cell::{Cell, RefCell};
use std::thread;

thread_local! {
    /// One counter per thread. `const { ... }` lets the compiler use the
    /// cheap initialiser path, which is worth having on a hot accessor.
    static CALLS: Cell<u64> = const { Cell::new(0) };

    /// One log per thread. `RefCell` because a `Vec` is not `Copy`, and
    /// the borrow check happens at runtime inside a single thread.
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Increments the calling thread's counter and returns the new value.
///
/// No lock, no atomic, no `Arc`. Each thread gets its own `Cell`, created
/// the first time that thread touches `CALLS`, so there is no sharing to
/// synchronise. A counter that started life as `Arc<Mutex<u64>>` and only
/// ever needs per thread totals costs nothing at all once it moves here.
///
/// The value is destroyed when the thread ends, which is also why a fresh
/// thread always starts at 0.
///
/// # Examples
///
/// ```
/// use std::thread;
/// use thread_local_state::bump;
///
/// let count = thread::spawn(|| {
///     bump();
///     bump()
/// })
/// .join()
/// .unwrap();
///
/// assert_eq!(count, 2);
/// ```
pub fn bump() -> u64 {
    CALLS.with(|calls| {
        let next = calls.get() + 1;
        calls.set(next);
        next
    })
}

/// Appends an event to the calling thread's log.
///
/// # Examples
///
/// ```
/// use std::thread;
/// use thread_local_state::{events, record};
///
/// let log = thread::spawn(|| {
///     record("start");
///     record("stop");
///     events()
/// })
/// .join()
/// .unwrap();
///
/// assert_eq!(log, vec!["start", "stop"]);
/// ```
pub fn record(event: &str) {
    LOG.with_borrow_mut(|log| log.push(event.to_string()));
}

/// Returns a copy of the calling thread's log.
///
/// The log itself never leaves its thread, so this hands back a clone.
/// Returning a reference is not possible: the borrow only lives as long
/// as the closure `with` runs.
pub fn events() -> Vec<String> {
    LOG.with_borrow(|log| log.clone())
}

/// Runs one thread per entry, bumping the counter that many times, and
/// returns each thread's final count.
///
/// Every worker starts from 0 because thread local state is per thread,
/// not per process. The result is therefore just the input, which is the
/// whole point: nothing is shared, so nothing can be lost to a race.
///
/// # Examples
///
/// ```
/// use thread_local_state::counts_per_thread;
///
/// assert_eq!(counts_per_thread(&[3, 1, 2]), vec![3, 1, 2]);
/// ```
pub fn counts_per_thread(bumps: &[u64]) -> Vec<u64> {
    thread::scope(|scope| {
        let handles: Vec<_> = bumps
            .iter()
            .map(|&times| {
                scope.spawn(move || {
                    let mut last = 0;
                    for _ in 0..times {
                        last = bump();
                    }
                    last
                })
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    })
}

// Example usage
pub fn main() {
    println!("{:?}", counts_per_thread(&[3, 1, 2]));

    record("hello");
    println!("{:?}", events());
}
