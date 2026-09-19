use std::thread;

/// Counts the characters in every word, on a separate thread.
///
/// `words` is moved into the closure with `move`, so the spawned thread
/// owns the vector outright and nothing is borrowed across the thread
/// boundary.
///
/// # Examples
///
/// ```
/// use move_closures_threads::count_chars_in_thread;
///
/// let words = vec!["ab".to_string(), "cde".to_string()];
/// assert_eq!(count_chars_in_thread(words), 5);
/// ```
pub fn count_chars_in_thread(words: Vec<String>) -> usize {
    thread::spawn(move || words.iter().map(|w| w.chars().count()).sum())
        .join()
        .unwrap()
}

/// Greets every name on its own thread, preserving input order.
///
/// Each thread needs its own `String` because `move` transfers
/// ownership and only one thread can own `prefix`. Cloning per thread
/// is the straightforward fix.
///
/// Handles are joined in spawn order, so the output order matches the
/// input order no matter how the threads interleave.
///
/// # Examples
///
/// ```
/// use move_closures_threads::greet_each;
///
/// let names = vec!["Ada".to_string(), "Bo".to_string()];
/// let greetings = greet_each("Hi".to_string(), names);
/// assert_eq!(greetings, vec!["Hi, Ada!", "Hi, Bo!"]);
/// ```
pub fn greet_each(prefix: String, names: Vec<String>) -> Vec<String> {
    let handles: Vec<_> = names
        .into_iter()
        .map(|name| {
            let prefix = prefix.clone();
            thread::spawn(move || format!("{prefix}, {name}!"))
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
