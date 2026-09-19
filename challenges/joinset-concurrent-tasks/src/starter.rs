use std::time::Duration;

/// Stands in for a unit of work whose duration you do not control.
///
/// This one is written for you. It sleeps for `id` milliseconds and
/// then returns `id * 10`, so a smaller id always finishes first.
pub async fn work(id: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(id)).await;
    id * 10
}

/// Runs every id at once and returns the results in the order they
/// finished, not the order they were spawned.
pub async fn results_in_completion_order(ids: Vec<u64>) -> Vec<u64> {
    // TODO
    unimplemented!()
}

/// Runs every id at once, keeps the first `n` results, and abandons the
/// rest.
pub async fn first_n_to_finish(ids: Vec<u64>, n: usize) -> Vec<u64> {
    // TODO
    unimplemented!()
}

/// Runs the ids with at most `limit` in flight at a time, starting the
/// next one every time a task finishes. Results come back in completion
/// order.
pub async fn with_limit(ids: Vec<u64>, limit: usize) -> Vec<u64> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", results_in_completion_order(vec![30, 10, 20]).await);
    println!("{:?}", first_n_to_finish(vec![50, 10, 20], 2).await);
    println!("{:?}", with_limit(vec![50, 10, 25, 5], 2).await);
}
