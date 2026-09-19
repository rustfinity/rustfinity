use tokio::sync::mpsc;

/// Receives every remaining message, in order, until the channel closes.
///
/// `recv().await` yields `Some(value)` while messages are available or a
/// sender is still alive, and `None` once the last `Sender` has been
/// dropped and the buffer is empty. That `None` is the only clean way to
/// know a stream of messages is finished.
pub async fn drain(mut rx: mpsc::Receiver<i32>) -> Vec<i32> {
    let mut received = Vec::new();

    while let Some(value) = rx.recv().await {
        received.push(value);
    }

    received
}

/// Sends `0..n` over a bounded channel and sums what comes back.
///
/// The sender is moved into the spawned task, so it is dropped the moment
/// that task returns. Without that drop the loop below would wait forever
/// for a message that is never coming.
pub async fn produce_and_sum(n: i32, capacity: usize) -> i64 {
    let (tx, mut rx) = mpsc::channel(capacity);

    tokio::spawn(async move {
        for value in 0..n {
            if tx.send(value).await.is_err() {
                break;
            }
        }
    });

    let mut total = 0i64;

    while let Some(value) = rx.recv().await {
        total += value as i64;
    }

    total
}

// Example usage
#[tokio::main]
pub async fn main() {
    let (tx, rx) = mpsc::channel(4);

    tokio::spawn(async move {
        for value in [1, 2, 3] {
            tx.send(value).await.unwrap();
        }
    });

    println!("{:?}", drain(rx).await);
    println!("{}", produce_and_sum(100, 1).await);
}
