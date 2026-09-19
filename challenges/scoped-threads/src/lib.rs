use std::thread;

/// Finds the largest value by scanning both halves in parallel.
///
/// `thread::scope` guarantees every thread it spawns is joined before
/// the scope returns, so the borrow checker lets the threads hold plain
/// `&[i32]` references into `data`. No `'static`, no `Arc`, no clone.
///
/// # Examples
///
/// ```
/// use scoped_threads::parallel_max;
///
/// assert_eq!(parallel_max(&[3, 9, 1, 4]), Some(9));
/// assert_eq!(parallel_max(&[]), None);
/// ```
pub fn parallel_max(data: &[i32]) -> Option<i32> {
    let (left, right) = data.split_at(data.len() / 2);

    thread::scope(|s| {
        let left_handle = s.spawn(|| left.iter().copied().max());
        let right_max = right.iter().copied().max();

        match (left_handle.join().unwrap(), right_max) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (a, b) => a.or(b),
        }
    })
}

/// Splits values into evens and odds using two scoped threads.
///
/// Each thread borrows `data` immutably and one of the output vectors
/// mutably. The borrows are to different locals, so they can be held at
/// the same time.
///
/// # Examples
///
/// ```
/// use scoped_threads::split_evens_odds;
///
/// let (evens, odds) = split_evens_odds(&[1, 2, 3, 4]);
/// assert_eq!(evens, vec![2, 4]);
/// assert_eq!(odds, vec![1, 3]);
/// ```
pub fn split_evens_odds(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    thread::scope(|s| {
        s.spawn(|| evens.extend(data.iter().copied().filter(|n| n % 2 == 0)));
        s.spawn(|| odds.extend(data.iter().copied().filter(|n| n % 2 != 0)));
    });

    (evens, odds)
}
