use std::time::Duration;
use tokio::time::sleep;

/// Runs every delay concurrently and returns them in input order.
///
/// The shape that matters: spawn every task first, collecting the
/// `JoinHandle`s, and only then await them. Awaiting inside the spawning
/// loop turns the whole thing back into sequential code, because each
/// task finishes before the next one is even created.
pub async fn run_all(delays_ms: Vec<u64>) -> Vec<u64> {
    let mut handles = Vec::with_capacity(delays_ms.len());

    for ms in delays_ms {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(ms)).await;
            ms
        }));
    }

    let mut results = Vec::with_capacity(handles.len());

    for handle in handles {
        results.push(handle.await.expect("task should not panic"));
    }

    results
}

/// Divides `a` by `b` inside a spawned task.
///
/// `b == 0` makes the task panic. A panicking task does not bring the
/// runtime down: the panic is caught at the task boundary and surfaces
/// as an `Err` from the `JoinHandle`.
pub async fn checked_divide(a: i64, b: i64) -> Result<i64, String> {
    let handle = tokio::spawn(async move { a / b });

    match handle.await {
        Ok(value) => Ok(value),
        Err(_) => Err("task panicked".to_string()),
    }
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", run_all(vec![30, 20, 10]).await);
    println!("{:?}", checked_divide(10, 2).await);
    println!("{:?}", checked_divide(10, 0).await);
}
