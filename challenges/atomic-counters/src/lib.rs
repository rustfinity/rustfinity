use std::sync::atomic::{AtomicUsize, Ordering};

/// Cache statistics kept in atomics instead of behind a lock.
///
/// Every operation is a single atomic instruction, so there is no guard,
/// no blocking, and nothing to poison.
#[derive(Default)]
pub struct CacheStats {
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl CacheStats {
    /// Creates a fresh set of counters, both at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_counters::CacheStats;
    ///
    /// assert_eq!(CacheStats::new().snapshot(), (0, 0));
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one lookup: a hit if `hit` is true, otherwise a miss.
    ///
    /// `Relaxed` is enough here. The counters guard no other data, so the
    /// only guarantee needed is that no increment is lost.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_counters::CacheStats;
    ///
    /// let stats = CacheStats::new();
    /// stats.record(true);
    /// stats.record(false);
    /// assert_eq!(stats.snapshot(), (1, 1));
    /// ```
    pub fn record(&self, hit: bool) {
        let counter = if hit { &self.hits } else { &self.misses };
        counter.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns `(hits, misses)` as they stand right now.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_counters::CacheStats;
    ///
    /// let stats = CacheStats::new();
    /// stats.record(true);
    /// assert_eq!(stats.snapshot(), (1, 0));
    /// ```
    pub fn snapshot(&self) -> (usize, usize) {
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
        )
    }

    /// Returns `(hits, misses)` and sets both counters back to zero.
    ///
    /// Each counter is swapped in one atomic step, so no lookup recorded
    /// before the swap can be dropped on the floor.
    ///
    /// # Examples
    ///
    /// ```
    /// use atomic_counters::CacheStats;
    ///
    /// let stats = CacheStats::new();
    /// stats.record(true);
    /// stats.record(true);
    ///
    /// assert_eq!(stats.drain(), (2, 0));
    /// assert_eq!(stats.snapshot(), (0, 0));
    /// ```
    pub fn drain(&self) -> (usize, usize) {
        (
            self.hits.swap(0, Ordering::Relaxed),
            self.misses.swap(0, Ordering::Relaxed),
        )
    }
}
