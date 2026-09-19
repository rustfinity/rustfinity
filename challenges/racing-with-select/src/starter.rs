use std::future::Future;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::sleep;

/// Returns whichever of the two futures produces a value first.
pub async fn first_of<A, B>(a: A, b: B) -> String
where
    A: Future<Output = String>,
    B: Future<Output = String>,
{
    // TODO
    unimplemented!()
}

/// Collects messages until the channel closes or `limit` elapses.
pub async fn collect_until_deadline(rx: &mut mpsc::Receiver<i32>, limit: Duration) -> Vec<i32> {
    // TODO
    unimplemented!()
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
