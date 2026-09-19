use tokio::sync::mpsc;

/// Receives every remaining message, in order, until the channel closes.
pub async fn drain(rx: mpsc::Receiver<i32>) -> Vec<i32> {
    // TODO
    unimplemented!()
}

/// Sends `0..n` over a bounded channel and sums what comes back.
pub async fn produce_and_sum(n: i32, capacity: usize) -> i64 {
    // TODO
    unimplemented!()
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
