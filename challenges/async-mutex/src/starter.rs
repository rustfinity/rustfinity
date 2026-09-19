use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::sleep;

/// Stands in for an expensive lookup: a network call, a query, a file read.
///
/// This one is written for you. It only sleeps and builds a string.
pub async fn load_value(key: &str) -> String {
    sleep(Duration::from_millis(10)).await;
    format!("value-for-{key}")
}

/// A cache whose misses are filled by an async loader.
#[derive(Default)]
pub struct Cache {
    entries: Mutex<HashMap<String, String>>,
    loads: AtomicUsize,
}

impl Cache {
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Returns the value for `key`, loading it exactly once.
    pub async fn get_or_load(&self, key: &str) -> String {
        // TODO
        unimplemented!()
    }

    /// How many times the loader has actually run.
    pub fn loads(&self) -> usize {
        // TODO
        unimplemented!()
    }

    /// Every cached pair, sorted by key.
    pub async fn snapshot(&self) -> Vec<(String, String)> {
        // TODO
        unimplemented!()
    }
}

// Example usage
#[tokio::main]
pub async fn main() {
    use std::sync::Arc;

    let cache = Arc::new(Cache::new());
    let mut handles = Vec::new();

    for _ in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(
            async move { cache.get_or_load("config").await },
        ));
    }

    for handle in handles {
        println!("{}", handle.await.unwrap());
    }

    println!("loads: {}", cache.loads());
    println!("{:?}", cache.snapshot().await);
}
