use std::collections::HashMap;
use std::sync::RwLock;

/// A read-heavy word tally guarded by an `RwLock`.
///
/// Reads take `read()`, which many threads can hold at once. Only
/// `record` takes `write()`, which is exclusive.
#[derive(Default)]
pub struct WordCounts {
    counts: RwLock<HashMap<String, usize>>,
}

impl WordCounts {
    /// Creates an empty tally.
    ///
    /// # Examples
    ///
    /// ```
    /// use rwlock_readers_writers::WordCounts;
    ///
    /// assert_eq!(WordCounts::new().total(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one sighting of `word`. This is the only writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rwlock_readers_writers::WordCounts;
    ///
    /// let counts = WordCounts::new();
    /// counts.record("ferris");
    /// counts.record("ferris");
    /// assert_eq!(counts.count("ferris"), 2);
    /// ```
    pub fn record(&self, word: &str) {
        let mut guard = self.counts.write().unwrap();
        *guard.entry(word.to_string()).or_insert(0) += 1;
    }

    /// How many times `word` has been recorded. Takes a read lock.
    ///
    /// # Examples
    ///
    /// ```
    /// use rwlock_readers_writers::WordCounts;
    ///
    /// assert_eq!(WordCounts::new().count("nope"), 0);
    /// ```
    pub fn count(&self, word: &str) -> usize {
        let guard = self.counts.read().unwrap();
        guard.get(word).copied().unwrap_or(0)
    }

    /// The total number of sightings across every word. Takes a read lock.
    ///
    /// # Examples
    ///
    /// ```
    /// use rwlock_readers_writers::WordCounts;
    ///
    /// let counts = WordCounts::new();
    /// counts.record("a");
    /// counts.record("b");
    /// assert_eq!(counts.total(), 2);
    /// ```
    pub fn total(&self) -> usize {
        let guard = self.counts.read().unwrap();
        guard.values().sum()
    }

    /// The word with the most sightings, and its count.
    ///
    /// Ties are broken by taking the alphabetically smallest word, so the
    /// answer never depends on thread timing or hash order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rwlock_readers_writers::WordCounts;
    ///
    /// let counts = WordCounts::new();
    /// counts.record("zebra");
    /// counts.record("ant");
    /// assert_eq!(counts.most_common(), Some(("ant".to_string(), 1)));
    /// ```
    pub fn most_common(&self) -> Option<(String, usize)> {
        let guard = self.counts.read().unwrap();
        guard
            .iter()
            .min_by(|(a_word, a_count), (b_word, b_count)| {
                b_count.cmp(a_count).then_with(|| a_word.cmp(b_word))
            })
            .map(|(word, count)| (word.clone(), *count))
    }
}
