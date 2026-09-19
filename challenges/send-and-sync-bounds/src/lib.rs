use std::sync::Arc;
use std::thread;

/// Applies `f` to every item on its own thread, preserving order.
///
/// The bounds are the exercise:
///
/// - `T: Send` because each item is *moved* into another thread.
/// - `U: Send` because each result is moved back out of one.
/// - `F: Fn(T) -> U` so one `f` can be called many times.
/// - `F: Send + Sync` because a single `f` is shared by every thread at
///   once through an `Arc`. `Sync` is exactly the "`&F` may cross a
///   thread boundary" permission.
/// - `'static` on all three because `thread::spawn` needs a closure that
///   could outlive this call. `thread::scope` is what lifts that
///   requirement, at the cost of blocking here.
///
/// # Examples
///
/// ```
/// use send_and_sync_bounds::parallel_map;
///
/// let doubled = parallel_map(vec![1, 2, 3], |n: i32| n * 2);
/// assert_eq!(doubled, vec![2, 4, 6]);
///
/// let lengths = parallel_map(
///     vec![String::from("ab"), String::from("cde")],
///     |s| s.len(),
/// );
/// assert_eq!(lengths, vec![2, 3]);
/// ```
pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let f = Arc::new(f);

    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let f = Arc::clone(&f);
            thread::spawn(move || f(item))
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

/// Runs every job on its own thread and returns the results in order.
///
/// A job is called once and consumed, so `FnOnce` is the right trait
/// here. It is moved rather than shared, so it needs `Send` but not
/// `Sync`. Asking for `Fn` instead would reject any closure that moves a
/// captured `String` into its body.
///
/// # Examples
///
/// ```
/// use send_and_sync_bounds::run_jobs;
///
/// let owned = String::from("hello");
/// let jobs: Vec<Box<dyn FnOnce() -> usize + Send>> = vec![
///     Box::new(|| 1 + 1),
///     Box::new(move || owned.len()),
/// ];
///
/// assert_eq!(run_jobs(jobs), vec![2, 5]);
/// ```
pub fn run_jobs<T, F>(jobs: Vec<F>) -> Vec<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let handles: Vec<_> = jobs.into_iter().map(thread::spawn).collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}
