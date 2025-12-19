One common challenge in concurrency is managing shared state across multiple threads. Rust's ownership model, along with types like `Mutex` and `Arc` (atomic reference counting), makes this task safe and efficient.

In this challenge, you will implement a generic shared state system and use it to create thread-safe counters and other shared data structures.

## Your Task

Implement three functions that demonstrate different aspects of shared state concurrency:

1. `create_shared_data`: Creates shared state with any type `T`
2. `increment_counter`: Demonstrates multiple threads modifying shared state
3. `modify_shared_data`: Generic function for thread-safe modifications

### Requirements

1. The `create_shared_data` function should:
   - Accept any type `T`
   - Wrap it in a `Mutex` and `Arc`
   - Return the thread-safe container

2. The `increment_counter` function should:
   - Accept an `Arc<Mutex<i32>>`
   - Spawn the specified number of threads
   - Return handles to all spawned threads

3. The `modify_shared_data` function should:
   - Perform modifications in a new thread
   - Return the thread handle

### Notes

- Ignore error handling for this challenge, you can use `unwrap()` to simplify the code
- Have a look at the `main` function to see how the functions are used

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use `Arc::new(Mutex::new(initial))` to create thread-safe containers
- Remember to cheap clone the `Arc` when passing it to a new thread. e.g. `let cloned = Arc::clone(&shared_data)`
- Use `lock().unwrap()` to access the data inside the Mutex
- To send data to a thread, the data must implement the `Send` trait and have a lifetime that outlives the thread `'static`

</details>
