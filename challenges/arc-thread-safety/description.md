# Arc and Thread Safety

While `Rc<T>` provides shared ownership for single-threaded code, it cannot be safely shared across threads because its reference count operations are not atomic. For multi-threaded scenarios, Rust provides `Arc<T>` (Atomically Reference Counted), which uses atomic operations to safely manage reference counts across thread boundaries.

`Arc<T>` works similarly to `Rc<T>` but is thread-safe. The "atomic" part means that incrementing and decrementing the reference count is done using atomic CPU operations, which are guaranteed to complete without interference from other threads. This makes `Arc<T>` slightly slower than `Rc<T>` due to the synchronization overhead, so you should only use `Arc<T>` when you actually need to share data across threads.

Common use cases for `Arc<T>` include:

- Sharing read-only configuration across worker threads
- Sharing immutable data structures in parallel computations
- Combined with `Mutex<T>` or `RwLock<T>` for shared mutable state
- Thread pools that share access to resources

## Key Differences from Rc

| Feature       | `Rc<T>`          | `Arc<T>`                |
| ------------- | ---------------- | ----------------------- |
| Thread-safe   | No               | Yes                     |
| Performance   | Faster           | Slightly slower         |
| Marker traits | `!Send`, `!Sync` | `Send + Sync` (if T is) |
| Use case      | Single-threaded  | Multi-threaded          |

## Key Operations

- `Arc::new(value)` - Create a new atomic reference-counted pointer
- `Arc::clone(&arc)` - Create a new reference (increments count atomically)
- `Arc::strong_count(&arc)` - Get the current number of strong references
- `Arc::weak_count(&arc)` - Get the current number of weak references
- `Arc::downgrade(&arc)` - Create a `Weak<T>` reference
- `weak.upgrade()` - Try to convert a `Weak<T>` back to `Arc<T>`

## Your Task

Implement the following types and functions to demonstrate `Arc<T>` patterns:

### Part 1: Basic Arc Operations

1. **`create_arc<T>(value: T) -> Arc<T>`** - Create a new atomic reference-counted value
2. **`clone_arc<T>(arc: &Arc<T>) -> Arc<T>`** - Clone a reference (increment count)
3. **`get_strong_count<T>(arc: &Arc<T>) -> usize`** - Get the strong reference count
4. **`get_value<T: Clone>(arc: &Arc<T>) -> T`** - Get a clone of the inner value

### Part 2: Thread-Safe Shared Data

5. **`SharedConfig`** - A thread-safe configuration that can be shared across threads:
   - `new(app_name: String, max_connections: usize, debug_mode: bool) -> Arc<Self>`
   - `app_name(&self) -> &str`
   - `max_connections(&self) -> usize`
   - `debug_mode(&self) -> bool`

### Part 3: Weak References

6. **`create_weak<T>(arc: &Arc<T>) -> Weak<T>`** - Create a weak reference
7. **`upgrade_weak<T>(weak: &Weak<T>) -> Option<Arc<T>>`** - Try to upgrade a weak reference
8. **`get_weak_count<T>(arc: &Arc<T>) -> usize`** - Get the weak reference count

### Part 4: Thread-Safe Counter

9. **`AtomicCounter`** - A thread-safe counter using `Arc<AtomicUsize>`:
   - `new() -> Self` - Create with initial value 0
   - `new_with_value(value: usize) -> Self` - Create with specific value
   - `get(&self) -> usize` - Get current value
   - `increment(&self) -> usize` - Increment and return previous value
   - `decrement(&self) -> usize` - Decrement and return previous value
   - `add(&self, val: usize) -> usize` - Add and return previous value
   - `clone_counter(&self) -> Self` - Create another handle to the same counter

### Part 5: Shared Data with Mutex

10. **`SharedVec<T>`** - A thread-safe vector using `Arc<Mutex<Vec<T>>>`:
    - `new() -> Self` - Create empty shared vec
    - `push(&self, value: T)` - Add an element
    - `pop(&self) -> Option<T>` - Remove and return last element
    - `len(&self) -> usize` - Get current length
    - `is_empty(&self) -> bool` - Check if empty
    - `get(&self, index: usize) -> Option<T>` where T: Clone - Get element at index
    - `clone_vec(&self) -> Self` - Create another handle to the same vec

## Examples

```rust
use arc_thread_safety::*;
use std::sync::Arc;
use std::thread;

// Basic Arc operations
let shared = create_arc(42);
assert_eq!(get_strong_count(&shared), 1);

let cloned = clone_arc(&shared);
assert_eq!(get_strong_count(&shared), 2);

// Sharing across threads
let data = create_arc(vec![1, 2, 3]);
let data_clone = clone_arc(&data);

let handle = thread::spawn(move || {
    // data_clone is accessible in this thread
    assert_eq!(data_clone.len(), 3);
});
handle.join().unwrap();

// Shared configuration
let config = SharedConfig::new("MyApp".to_string(), 100, true);
let config_clone = Arc::clone(&config);

thread::spawn(move || {
    assert_eq!(config_clone.app_name(), "MyApp");
}).join().unwrap();

// Atomic counter shared across threads
let counter = AtomicCounter::new();
let counter_clone = counter.clone_counter();

let handle = thread::spawn(move || {
    for _ in 0..1000 {
        counter_clone.increment();
    }
});
for _ in 0..1000 {
    counter.increment();
}
handle.join().unwrap();
assert_eq!(counter.get(), 2000);

// Thread-safe vector
let vec = SharedVec::new();
vec.push(1);
vec.push(2);
assert_eq!(vec.len(), 2);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `Arc::new()` to wrap values for thread-safe shared ownership
- `Arc::clone(&arc)` is cheap - it only atomically increments a counter
- For mutable shared state, combine `Arc` with `Mutex`: `Arc<Mutex<T>>`
- `AtomicUsize` provides lock-free atomic operations like `fetch_add` and `fetch_sub`
- Use `Ordering::SeqCst` for atomic operations if unsure about memory ordering
- When working with `Mutex`, use `.lock().unwrap()` to acquire the lock (panics if poisoned)
- `Weak<T>` works the same way with `Arc` as it does with `Rc`
- Remember: `Arc<T>` is `Send + Sync` only if `T` is `Send + Sync`

</details>
