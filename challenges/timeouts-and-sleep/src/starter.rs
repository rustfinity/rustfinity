use std::future::Future;
use std::time::Duration;

use tokio::time::{sleep, timeout};

/// Runs `work`, giving up if it takes longer than `limit`.
pub async fn with_timeout<F>(limit: Duration, work: F) -> Option<F::Output>
where
    F: Future,
{
    // TODO
    unimplemented!()
}

/// Calls `op` until it succeeds, waiting `gap` between tries.
pub async fn retry(attempts: usize, gap: Duration, op: impl FnMut() -> bool) -> Option<usize> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    let slow = async {
        sleep(Duration::from_millis(50)).await;
        "finished"
    };

    println!("{:?}", with_timeout(Duration::from_millis(10), slow).await);

    let mut tries = 0;
    let found = retry(5, Duration::from_millis(10), || {
        tries += 1;
        tries == 3
    })
    .await;

    println!("{found:?}");
}
