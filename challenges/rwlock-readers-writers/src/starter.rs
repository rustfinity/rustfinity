use std::collections::HashMap;
use std::sync::RwLock;

/// A read-heavy word tally guarded by an `RwLock`.
#[derive(Default)]
pub struct WordCounts {
    counts: RwLock<HashMap<String, usize>>,
}

impl WordCounts {
    /// Creates an empty tally.
    pub fn new() -> Self {
        // TODO
        unimplemented!()
    }

    /// Records one sighting of `word`. This is the only writer.
    pub fn record(&self, word: &str) {
        // TODO: take a write lock and bump the entry
        unimplemented!()
    }

    /// How many times `word` has been recorded.
    pub fn count(&self, word: &str) -> usize {
        // TODO: take a read lock
        unimplemented!()
    }

    /// The total number of sightings across every word.
    pub fn total(&self) -> usize {
        // TODO: take a read lock
        unimplemented!()
    }

    /// The word with the most sightings, and its count.
    ///
    /// Ties are broken by taking the alphabetically smallest word.
    pub fn most_common(&self) -> Option<(String, usize)> {
        // TODO: take a read lock
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    let counts = WordCounts::new();
    counts.record("ferris");
    counts.record("ferris");
    counts.record("crab");
    println!("{:?} {}", counts.most_common(), counts.total());
}
