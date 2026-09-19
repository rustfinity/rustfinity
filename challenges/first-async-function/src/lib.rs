use std::sync::atomic::{AtomicUsize, Ordering};

/// Builds a greeting for `name`.
///
/// Marking a function `async` changes its return type: the body is not
/// `String` any more, it is a future that produces a `String` when it is
/// awaited. Nothing in here runs at call time.
pub async fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Records the call in `calls`, then greets `name`.
///
/// The counter exists to make laziness observable. Calling this function
/// only builds a future, so `calls` is untouched until the future is
/// awaited. That is the whole difference between a Rust future and, say,
/// a JavaScript promise, which starts running the moment it is created.
pub async fn greet_and_count(calls: &AtomicUsize, name: &str) -> String {
    calls.fetch_add(1, Ordering::SeqCst);
    greet(name).await
}

// Example usage
#[tokio::main]
pub async fn main() {
    let calls = AtomicUsize::new(0);

    println!("{}", greet("Ada").await);
    println!("{}", greet_and_count(&calls, "Grace").await);
    println!("calls: {}", calls.load(Ordering::SeqCst));
}
