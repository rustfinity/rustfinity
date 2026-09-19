/// Runs every delay concurrently and returns them in input order.
///
/// Each task should sleep for its own number of milliseconds and then
/// return that number.
pub async fn run_all(delays_ms: Vec<u64>) -> Vec<u64> {
    // TODO
    unimplemented!()
}

/// Divides `a` by `b` inside a spawned task.
pub async fn checked_divide(a: i64, b: i64) -> Result<i64, String> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", run_all(vec![30, 20, 10]).await);
    println!("{:?}", checked_divide(10, 0).await);
}
