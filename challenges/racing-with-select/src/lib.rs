use std::future::Future;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::sleep;

/// Returns whichever of the two futures produces a value first.
///
/// `select!` polls every branch, and the first one to be ready wins.
/// The losing branch is dropped right there, mid-flight, which is why
/// only cancel-safe futures belong in a `select!` you run in a loop.
pub async fn first_of<A, B>(a: A, b: B) -> String
where
    A: Future<Output = String>,
    B: Future<Output = String>,
{
    tokio::select! {
        value = a => value,
        value = b => value,
    }
}

/// Collects messages until the channel closes or `limit` elapses.
///
/// The deadline is created **once**, before the loop, and pinned so the
/// same sleep can be polled again on every pass. Creating it inside the
/// loop would restart the clock after every message, and a steady
/// producer would keep the deadline alive forever.
pub async fn collect_until_deadline(rx: &mut mpsc::Receiver<i32>, limit: Duration) -> Vec<i32> {
    let mut collected = Vec::new();
    let deadline = sleep(limit);
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            message = rx.recv() => match message {
                Some(value) => collected.push(value),
                None => break,
            },
            _ = &mut deadline => break,
        }
    }

    collected
}

// Example usage
#[tokio::main]
pub async fn main() {
    let slow = async {
        sleep(Duration::from_millis(50)).await;
        "slow".to_string()
    };
    let quick = async {
        sleep(Duration::from_millis(5)).await;
        "quick".to_string()
    };

    println!("{}", first_of(slow, quick).await);

    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(async move {
        for value in 0..3 {
            tx.send(value).await.unwrap();
        }
    });

    let collected = collect_until_deadline(&mut rx, Duration::from_millis(50)).await;
    println!("{collected:?}");
}
