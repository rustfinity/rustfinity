A thread is a second line of execution inside the same process. In Rust you start one with `std::thread::spawn`, which takes a closure and immediately returns a `JoinHandle`. The closure starts running right away, in parallel with the code that spawned it.

The handle is the only way back. Calling `join()` on it blocks the current thread until the spawned one finishes, and hands you whatever the closure returned. That return value is how work crosses the thread boundary: no channels, no shared memory, just a value coming back out of `join()`.

Because the closure may outlive the function that created it, `spawn` requires the closure to own everything it touches. That is what the `move` keyword is for, and it is why `values` below is moved into the thread rather than borrowed.

## Your Task

Implement two functions.

### Implement `sum_in_thread`

```rust
pub fn sum_in_thread(values: Vec<u64>) -> u64
```

Spawn a thread that adds up `values`, join it, and return the total.

```rust
assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
assert_eq!(sum_in_thread(vec![]), 0);
```

### Implement `spawn_counter`

```rust
pub fn spawn_counter(
    start: u64,
    steps: u64,
) -> JoinHandle<u64>
```

Spawn a thread that adds up `steps` consecutive numbers beginning at `start`, and return the handle **without** joining it. The caller decides when to block.

```rust
// 10 + 11 + 12
let h = spawn_counter(10, 3);
assert_eq!(h.join().unwrap(), 33);

// no steps means nothing to add
assert_eq!(spawn_counter(7, 0).join().unwrap(), 0);
```

### Notes

- `join()` returns a `Result`. Panics inside a thread are a topic of their own, so `unwrap()` is fine here.
- Returning the handle is the interesting part of the second function. It lets the caller start work now and collect it later.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `thread::spawn(move || { ... })` returns a `JoinHandle<T>`, where `T` is the closure's return type.
- The last expression of the closure is its return value, exactly like a function body.
- For the counter, `(0..steps).map(|i| start + i).sum()` avoids a manual loop.
- If the compiler complains that a captured variable does not live long enough, you are missing `move`.

</details>
