use std::thread;

/// Runs a job on a thread and reports whether it panicked.
///
/// Returns `None` when the job finished normally, or `Some(message)`
/// with the panic message. Payloads that are neither `&'static str` nor
/// `String` become `"unknown panic"`.
pub fn describe_panic(job: Box<dyn FnOnce() + Send + 'static>) -> Option<String> {
    // TODO: spawn the job, join it, and inspect the error payload
    unimplemented!()
}

/// Divides on a worker thread, turning a panic into an `Err`.
pub fn divide_in_thread(a: i32, b: i32) -> Result<i32, String> {
    // TODO: panic with "division by zero" inside the thread when b is 0,
    // then map the joined panic payload into Err
    unimplemented!()
}

// Example usage
pub fn main() {
    println!("{:?}", describe_panic(Box::new(|| {})));
    println!("{:?}", describe_panic(Box::new(|| panic!("boom"))));

    println!("{:?}", divide_in_thread(10, 2));
    println!("{:?}", divide_in_thread(1, 0));
}
