use std::thread;

/// Sums each chunk on its own thread, keeping the input order.
///
/// All threads are spawned first and collected into a
/// `Vec<JoinHandle<u64>>`, then joined in the same order. That is what
/// makes the result deterministic: the positions come from the handle
/// vector, not from whichever thread finished first.
///
/// # Examples
///
/// ```
/// use collecting_thread_results::sum_chunks;
///
/// let chunks = vec![vec![1, 2], vec![10], vec![]];
/// assert_eq!(sum_chunks(chunks), vec![3, 10, 0]);
/// ```
pub fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<u64>()))
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Sums every chunk in parallel and adds the partial sums together.
///
/// # Examples
///
/// ```
/// use collecting_thread_results::sum_all;
///
/// assert_eq!(sum_all(vec![vec![1, 2], vec![3]]), 6);
/// ```
pub fn sum_all(chunks: Vec<Vec<u64>>) -> u64 {
    sum_chunks(chunks).into_iter().sum()
}
