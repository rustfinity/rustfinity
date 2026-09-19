use std::time::Duration;

/// Stands in for slow, blocking work: a password hash, an image resize,
/// a synchronous file read.
///
/// This one is written for you. It sleeps the *thread* - not the task -
/// and then folds the bytes into a number.
pub fn checksum(data: &str) -> u64 {
    std::thread::sleep(Duration::from_millis(30));

    data.bytes().fold(0u64, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u64)
    })
}

/// Runs `checksum` without stalling the async runtime.
pub async fn checksum_one(data: String) -> u64 {
    tokio::task::spawn_blocking(move || checksum(&data))
        .await
        .expect("the checksum task panicked")
}

/// Checksums every input concurrently, keeping the input order.
///
/// Every closure is handed to the blocking pool first, and only then are
/// the handles awaited. Awaiting each handle before spawning the next
/// would make the whole thing sequential again.
pub async fn checksum_all(items: Vec<String>) -> Vec<u64> {
    let handles: Vec<_> = items
        .into_iter()
        .map(|data| tokio::task::spawn_blocking(move || checksum(&data)))
        .collect();

    let mut results = Vec::with_capacity(handles.len());

    for handle in handles {
        results.push(handle.await.expect("the checksum task panicked"));
    }

    results
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{}", checksum_one("hello".to_string()).await);

    let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    println!("{:?}", checksum_all(inputs).await);
}
