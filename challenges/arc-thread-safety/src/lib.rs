use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

/// Creates a new atomic reference-counted value.
///
/// Wraps the given value in an `Arc<T>` for thread-safe shared ownership.
///
/// # Arguments
///
/// * `value` - The value to wrap in an Arc
///
/// # Examples
///
/// ```
/// use arc_thread_safety::create_arc;
/// use std::sync::Arc;
///
/// let shared = create_arc(42);
/// assert_eq!(*shared, 42);
/// ```
pub fn create_arc<T>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Clones an atomic reference-counted pointer.
///
/// This atomically increments the reference count without cloning the underlying data.
///
/// # Arguments
///
/// * `arc` - The Arc to clone
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, clone_arc, get_strong_count};
///
/// let original = create_arc(42);
/// let cloned = clone_arc(&original);
/// assert_eq!(get_strong_count(&original), 2);
/// ```
pub fn clone_arc<T>(arc: &Arc<T>) -> Arc<T> {
    Arc::clone(arc)
}

/// Returns the strong reference count of an Arc.
///
/// # Arguments
///
/// * `arc` - The Arc to inspect
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, get_strong_count};
///
/// let shared = create_arc("hello");
/// assert_eq!(get_strong_count(&shared), 1);
/// ```
pub fn get_strong_count<T>(arc: &Arc<T>) -> usize {
    Arc::strong_count(arc)
}

/// Gets a clone of the value inside an Arc.
///
/// # Arguments
///
/// * `arc` - The Arc containing the value
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, get_value};
///
/// let shared = create_arc(String::from("hello"));
/// let value = get_value(&shared);
/// assert_eq!(value, "hello");
/// ```
pub fn get_value<T: Clone>(arc: &Arc<T>) -> T {
    (**arc).clone()
}

/// A thread-safe configuration that can be shared across threads.
///
/// Uses `Arc` internally for thread-safe sharing of immutable configuration.
#[derive(Debug, Clone)]
pub struct SharedConfig {
    app_name: String,
    max_connections: usize,
    debug_mode: bool,
}

impl SharedConfig {
    /// Creates a new shared configuration wrapped in Arc.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The application name
    /// * `max_connections` - Maximum number of connections
    /// * `debug_mode` - Whether debug mode is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedConfig;
    ///
    /// let config = SharedConfig::new("MyApp".to_string(), 100, true);
    /// assert_eq!(config.app_name(), "MyApp");
    /// ```
    pub fn new(app_name: String, max_connections: usize, debug_mode: bool) -> Arc<Self> {
        Arc::new(SharedConfig {
            app_name,
            max_connections,
            debug_mode,
        })
    }

    /// Returns the application name.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedConfig;
    ///
    /// let config = SharedConfig::new("TestApp".to_string(), 50, false);
    /// assert_eq!(config.app_name(), "TestApp");
    /// ```
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Returns the maximum number of connections.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedConfig;
    ///
    /// let config = SharedConfig::new("App".to_string(), 200, false);
    /// assert_eq!(config.max_connections(), 200);
    /// ```
    pub fn max_connections(&self) -> usize {
        self.max_connections
    }

    /// Returns whether debug mode is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedConfig;
    ///
    /// let config = SharedConfig::new("App".to_string(), 100, true);
    /// assert!(config.debug_mode());
    /// ```
    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }
}

/// Creates a weak reference from an Arc.
///
/// Weak references don't prevent the value from being deallocated.
///
/// # Arguments
///
/// * `arc` - The Arc to create a weak reference from
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, create_weak, get_weak_count};
///
/// let strong = create_arc(42);
/// let weak = create_weak(&strong);
/// assert_eq!(get_weak_count(&strong), 1);
/// ```
pub fn create_weak<T>(arc: &Arc<T>) -> Weak<T> {
    Arc::downgrade(arc)
}

/// Attempts to upgrade a weak reference to a strong reference.
///
/// Returns `Some(Arc<T>)` if the value still exists, `None` if it was deallocated.
///
/// # Arguments
///
/// * `weak` - The weak reference to upgrade
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, create_weak, upgrade_weak};
///
/// let strong = create_arc(42);
/// let weak = create_weak(&strong);
///
/// // Value still exists
/// assert!(upgrade_weak(&weak).is_some());
///
/// drop(strong);
///
/// // Value was deallocated
/// assert!(upgrade_weak(&weak).is_none());
/// ```
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Arc<T>> {
    weak.upgrade()
}

/// Returns the weak reference count of an Arc.
///
/// # Arguments
///
/// * `arc` - The Arc to inspect
///
/// # Examples
///
/// ```
/// use arc_thread_safety::{create_arc, create_weak, get_weak_count};
///
/// let shared = create_arc("hello");
/// assert_eq!(get_weak_count(&shared), 0);
///
/// let _weak = create_weak(&shared);
/// assert_eq!(get_weak_count(&shared), 1);
/// ```
pub fn get_weak_count<T>(arc: &Arc<T>) -> usize {
    Arc::weak_count(arc)
}

/// A thread-safe counter using atomic operations.
///
/// Multiple threads can safely increment, decrement, and read the counter
/// without requiring locks.
#[derive(Debug)]
pub struct AtomicCounter {
    value: Arc<AtomicUsize>,
}

impl AtomicCounter {
    /// Creates a new counter with initial value 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new();
    /// assert_eq!(counter.get(), 0);
    /// ```
    pub fn new() -> Self {
        AtomicCounter {
            value: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Creates a new counter with a specific initial value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new_with_value(100);
    /// assert_eq!(counter.get(), 100);
    /// ```
    pub fn new_with_value(value: usize) -> Self {
        AtomicCounter {
            value: Arc::new(AtomicUsize::new(value)),
        }
    }

    /// Gets the current value of the counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new_with_value(42);
    /// assert_eq!(counter.get(), 42);
    /// ```
    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    /// Increments the counter by 1 and returns the previous value.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new();
    /// assert_eq!(counter.increment(), 0);
    /// assert_eq!(counter.get(), 1);
    /// ```
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    /// Decrements the counter by 1 and returns the previous value.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new_with_value(5);
    /// assert_eq!(counter.decrement(), 5);
    /// assert_eq!(counter.get(), 4);
    /// ```
    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    /// Adds a value to the counter and returns the previous value.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter = AtomicCounter::new();
    /// assert_eq!(counter.add(10), 0);
    /// assert_eq!(counter.get(), 10);
    /// ```
    pub fn add(&self, val: usize) -> usize {
        self.value.fetch_add(val, Ordering::SeqCst)
    }

    /// Creates another handle to the same counter.
    ///
    /// Both handles will share the same underlying atomic value.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::AtomicCounter;
    ///
    /// let counter1 = AtomicCounter::new();
    /// let counter2 = counter1.clone_counter();
    ///
    /// counter1.increment();
    /// assert_eq!(counter2.get(), 1);
    /// ```
    pub fn clone_counter(&self) -> Self {
        AtomicCounter {
            value: Arc::clone(&self.value),
        }
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// A thread-safe vector using Arc and Mutex.
///
/// Multiple threads can safely push, pop, and read from this vector.
#[derive(Debug)]
pub struct SharedVec<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec: SharedVec<i32> = SharedVec::new();
    /// assert!(vec.is_empty());
    /// ```
    pub fn new() -> Self {
        SharedVec {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds an element to the end of the vector.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.len(), 2);
    /// ```
    pub fn push(&self, value: T) {
        self.data.lock().unwrap().push(value);
    }

    /// Removes and returns the last element, or None if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.pop(), Some(2));
    /// assert_eq!(vec.pop(), Some(1));
    /// assert_eq!(vec.pop(), None);
    /// ```
    pub fn pop(&self) -> Option<T> {
        self.data.lock().unwrap().pop()
    }

    /// Returns the current number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// assert_eq!(vec.len(), 0);
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    /// Returns true if the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec: SharedVec<i32> = SharedVec::new();
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    /// Creates another handle to the same vector.
    ///
    /// Both handles will share the same underlying vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec1 = SharedVec::new();
    /// let vec2 = vec1.clone_vec();
    ///
    /// vec1.push(42);
    /// assert_eq!(vec2.len(), 1);
    /// ```
    pub fn clone_vec(&self) -> Self {
        SharedVec {
            data: Arc::clone(&self.data),
        }
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to get
    ///
    /// # Examples
    ///
    /// ```
    /// use arc_thread_safety::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(10);
    /// vec.push(20);
    /// vec.push(30);
    /// assert_eq!(vec.get(1), Some(20));
    /// assert_eq!(vec.get(10), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<T> {
        self.data.lock().unwrap().get(index).cloned()
    }
}

impl<T> Default for SharedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arc_basic() {
        let shared = create_arc(42);
        assert_eq!(*shared, 42);
    }

    #[test]
    fn test_clone_arc_increments_count() {
        let original = create_arc(42);
        assert_eq!(get_strong_count(&original), 1);

        let cloned = clone_arc(&original);
        assert_eq!(get_strong_count(&original), 2);
        assert_eq!(get_strong_count(&cloned), 2);
    }

    #[test]
    fn test_get_value() {
        let shared = create_arc(String::from("hello"));
        let value = get_value(&shared);
        assert_eq!(value, "hello");
    }

    #[test]
    fn test_shared_config_basic() {
        let config = SharedConfig::new("TestApp".to_string(), 100, true);
        assert_eq!(config.app_name(), "TestApp");
        assert_eq!(config.max_connections(), 100);
        assert!(config.debug_mode());
    }

    #[test]
    fn test_atomic_counter_basic() {
        let counter = AtomicCounter::new();
        assert_eq!(counter.get(), 0);

        counter.increment();
        assert_eq!(counter.get(), 1);

        counter.add(10);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_shared_vec_basic() {
        let vec = SharedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(1), Some(2));
        assert_eq!(vec.pop(), Some(3));
    }
}
