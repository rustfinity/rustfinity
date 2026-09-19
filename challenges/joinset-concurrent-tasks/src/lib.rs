use std::time::Duration;

use tokio::task::JoinSet;

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
    let mut set = JoinSet::new();

    for id in ids {
        set.spawn(work(id));
    }

    let mut results = Vec::new();

    while let Some(result) = set.join_next().await {
        results.push(result.expect("a work task panicked"));
    }

    results
}

/// Runs every id at once, keeps the first `n` results, and abandons the
/// rest.
///
/// Dropping the `JoinSet` would abort the stragglers too. `abort_all`
/// says so out loud.
pub async fn first_n_to_finish(ids: Vec<u64>, n: usize) -> Vec<u64> {
    let mut set = JoinSet::new();

    for id in ids {
        set.spawn(work(id));
    }

    let mut results = Vec::new();

    while results.len() < n {
        match set.join_next().await {
            Some(result) => results.push(result.expect("a work task panicked")),
            None => break,
        }
    }

    set.abort_all();
    results
}

/// Runs the ids with at most `limit` in flight at a time, starting the
/// next one every time a task finishes. Results come back in completion
/// order.
pub async fn with_limit(ids: Vec<u64>, limit: usize) -> Vec<u64> {
    let limit = limit.max(1);
    let mut pending = ids.into_iter();
    let mut set = JoinSet::new();

    for id in pending.by_ref().take(limit) {
        set.spawn(work(id));
    }

    let mut results = Vec::new();

    while let Some(result) = set.join_next().await {
        results.push(result.expect("a work task panicked"));

        if let Some(id) = pending.next() {
            set.spawn(work(id));
        }
    }

    results
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", results_in_completion_order(vec![30, 10, 20]).await);
    println!("{:?}", first_n_to_finish(vec![50, 10, 20], 2).await);
    println!("{:?}", with_limit(vec![50, 10, 25, 5], 2).await);
}
