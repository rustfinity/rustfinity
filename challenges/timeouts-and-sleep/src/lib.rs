use std::future::Future;
use std::time::Duration;

use tokio::time::{sleep, timeout};

/// Runs `work`, giving up if it takes longer than `limit`.
///
/// `tokio::time::timeout` returns `Ok(output)` when the inner future
/// finishes in time and `Err(Elapsed)` when it does not. Turning that
/// into an `Option` with `.ok()` throws away an error type that carries
/// no information anyway.
///
/// When the timeout wins, the inner future is dropped where it stood.
/// Nothing after its last `.await` ever runs.
pub async fn with_timeout<F>(limit: Duration, work: F) -> Option<F::Output>
where
    F: Future,
{
    timeout(limit, work).await.ok()
}

/// Calls `op` until it succeeds, waiting `gap` between tries.
///
/// The sleep goes *between* attempts, never before the first one and
/// never after the last, so `attempts` tries cost `gap * (attempts - 1)`
/// in the worst case.
pub async fn retry(attempts: usize, gap: Duration, mut op: impl FnMut() -> bool) -> Option<usize> {
    for attempt in 0..attempts {
        if attempt > 0 {
            sleep(gap).await;
        }

        if op() {
            return Some(attempt);
        }
    }

    None
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
