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
    // TODO
    unimplemented!()
}

/// Checksums every input concurrently, keeping the input order.
pub async fn checksum_all(items: Vec<String>) -> Vec<u64> {
    // TODO
    unimplemented!()
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{}", checksum_one("hello".to_string()).await);

    let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    println!("{:?}", checksum_all(inputs).await);
}
