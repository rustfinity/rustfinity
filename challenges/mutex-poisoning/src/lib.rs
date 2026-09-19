use std::sync::Mutex;

/// A counter that keeps working after a thread panics while holding the lock.
///
/// `Mutex::lock` returns a `Result` because a mutex becomes *poisoned* when a
/// thread panics while holding its guard. The data is still there, and
/// `PoisonError::into_inner` hands the guard over anyway.
pub struct ResilientCounter {
    value: Mutex<i64>,
}

impl ResilientCounter {
    /// Creates a counter starting at `start`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutex_poisoning::ResilientCounter;
    ///
    /// let counter = ResilientCounter::new(5);
    /// assert_eq!(counter.get(), 5);
    /// ```
    pub fn new(start: i64) -> Self {
        Self {
            value: Mutex::new(start),
        }
    }

    /// Runs `f` on the counter while holding the lock, and returns its result.
    ///
    /// The lock is taken even when the mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutex_poisoning::ResilientCounter;
    ///
    /// let counter = ResilientCounter::new(0);
    /// let doubled = counter.with(|value| {
    ///     *value += 21;
    ///     *value * 2
    /// });
    ///
    /// assert_eq!(doubled, 42);
    /// assert_eq!(counter.get(), 21);
    /// ```
    pub fn with<R>(&self, f: impl FnOnce(&mut i64) -> R) -> R {
        let mut guard = self.value.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut guard)
    }

    /// Reads the counter, even if the mutex is poisoned.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutex_poisoning::ResilientCounter;
    ///
    /// assert_eq!(ResilientCounter::new(9).get(), 9);
    /// ```
    pub fn get(&self) -> i64 {
        self.with(|value| *value)
    }

    /// Reports whether some thread has panicked while holding the lock.
    ///
    /// # Examples
    ///
    /// ```
    /// use mutex_poisoning::ResilientCounter;
    ///
    /// assert!(!ResilientCounter::new(0).is_poisoned());
    /// ```
    pub fn is_poisoned(&self) -> bool {
        self.value.is_poisoned()
    }
}
