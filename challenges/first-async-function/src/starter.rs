use std::sync::atomic::AtomicUsize;

/// Builds a greeting for `name`.
pub async fn greet(name: &str) -> String {
    // TODO
    unimplemented!()
}

/// Records the call in `calls`, then greets `name`.
pub async fn greet_and_count(calls: &AtomicUsize, name: &str) -> String {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    let calls = AtomicUsize::new(0);

    println!("{}", greet("Ada").await);
    println!("{}", greet_and_count(&calls, "Grace").await);
}
