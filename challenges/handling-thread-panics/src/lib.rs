use std::thread;

/// Runs a job on a thread and reports whether it panicked.
///
/// `join()` returns `Result<T, Box<dyn Any + Send>>`. The error side is
/// the panic payload, which has to be downcast before it is readable:
/// `panic!("literal")` stores a `&'static str`, while a formatted
/// `panic!("{}", x)` stores a `String`.
///
/// Returns `None` when the job finished normally, or `Some(message)`
/// with the panic message. A payload of neither type becomes
/// `"unknown panic"`.
///
/// # Examples
///
/// ```
/// use handling_thread_panics::describe_panic;
///
/// assert_eq!(describe_panic(Box::new(|| {})), None);
///
/// let msg = describe_panic(Box::new(|| panic!("boom")));
/// assert_eq!(msg, Some("boom".to_string()));
/// ```
pub fn describe_panic(job: Box<dyn FnOnce() + Send + 'static>) -> Option<String> {
    match thread::spawn(job).join() {
        Ok(()) => None,
        Err(payload) => {
            if let Some(s) = payload.downcast_ref::<&'static str>() {
                Some(s.to_string())
            } else if let Some(s) = payload.downcast_ref::<String>() {
                Some(s.clone())
            } else {
                Some("unknown panic".to_string())
            }
        }
    }
}

/// Divides on a worker thread, turning a panic into an `Err`.
///
/// The thread panics with `"division by zero"` when `b` is 0. The
/// calling thread survives, because a panic only unwinds the thread it
/// happened on.
///
/// # Examples
///
/// ```
/// use handling_thread_panics::divide_in_thread;
///
/// assert_eq!(divide_in_thread(10, 2), Ok(5));
/// assert_eq!(
///     divide_in_thread(1, 0),
///     Err("division by zero".to_string())
/// );
/// ```
pub fn divide_in_thread(a: i32, b: i32) -> Result<i32, String> {
    thread::spawn(move || {
        if b == 0 {
            panic!("division by zero");
        }
        a / b
    })
    .join()
    .map_err(|payload| match payload.downcast_ref::<&'static str>() {
        Some(s) => s.to_string(),
        None => "unknown panic".to_string(),
    })
}
