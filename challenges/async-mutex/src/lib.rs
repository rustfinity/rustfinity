use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
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
        Self::default()
    }

    /// Returns the value for `key`, loading it exactly once.
    ///
    /// The guard is held across the `.await` on purpose. That is the
    /// whole point of `tokio::sync::Mutex`: callers that arrive during
    /// a load wait on the lock, and by the time they get it the entry
    /// is already there. Dropping the guard before the load and taking
    /// it again afterwards would let every caller start its own load.
    pub async fn get_or_load(&self, key: &str) -> String {
        let mut entries = self.entries.lock().await;

        if let Some(value) = entries.get(key) {
            return value.clone();
        }

        let value = load_value(key).await;
        self.loads.fetch_add(1, Ordering::SeqCst);
        entries.insert(key.to_string(), value.clone());

        value
    }

    /// How many times the loader has actually run.
    pub fn loads(&self) -> usize {
        self.loads.load(Ordering::SeqCst)
    }

    /// Every cached pair, sorted by key.
    pub async fn snapshot(&self) -> Vec<(String, String)> {
        let entries = self.entries.lock().await;
        let mut pairs: Vec<_> = entries
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        pairs.sort();
        pairs
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
