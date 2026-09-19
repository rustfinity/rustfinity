`Send` and `Sync` are the two marker traits that make Rust's threading safe, and you mostly never mention them. They are auto-implemented for anything built out of parts that already have them, and the compiler checks them at every thread boundary.

- **`Send`** means a value can be *moved* to another thread.
- **`Sync`** means `&T` can be shared with another thread. Equivalently, `T: Sync` exactly when `&T: Send`.

`Rc<T>` is neither. Its reference count is a plain non-atomic integer, so two threads bumping it at once would corrupt it, and the compiler simply refuses:

```
`Rc<i32>` cannot be sent between threads safely
```

`Arc<T>` is the same type with an atomic count, so it is `Send + Sync` whenever `T` is. `Cell` and `RefCell` are `Send` but not `Sync`: moving one is fine, sharing one is not, because their borrow tracking is not atomic either.

You start writing these bounds yourself the moment you write a generic helper that spawns:

```rust
pub fn run<T, F>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

That is `thread::spawn`'s own signature. The `'static` is not "lives forever"; it is "contains no borrows of anything shorter", which is what `spawn` needs because the thread may outlive the caller. `thread::scope` is the escape hatch that drops it.

The closure trait matters too:

- `FnOnce` is consumed by the call. A closure that moves out a captured `String` is only `FnOnce`.
- `Fn` can be called repeatedly through `&self`, which is what you need if one closure is shared by many threads.

And sharing a closure across threads is a `Sync` requirement, not a `Send` one: `Arc<F>` is only `Send` when `F: Send + Sync`.

## Your Task

Write the bounds for two generic helpers. The bodies are ordinary; the `where` clauses are the exercise.

### Implement `parallel_map`

```rust
pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
```

Runs `f` on every item, each on its own thread, and returns the results in the same order as the input. One `f` is shared by all of the threads, so wrap it in an `Arc` and clone that per thread.

```rust
let doubled = parallel_map(vec![1, 2, 3], |n: i32| n * 2);
assert_eq!(doubled, vec![2, 4, 6]);
```

Your bounds must be tight enough to compile and loose enough to accept non-`Copy` items and results, such as `Vec<String>` in and `Vec<usize>` out.

### Implement `run_jobs`

```rust
pub fn run_jobs<T, F>(jobs: Vec<F>) -> Vec<T>
```

Runs each job on its own thread and returns the results in job order. Each job is called exactly once and is moved rather than shared, so its bounds are looser than `parallel_map`'s in one specific way. It must accept `Box<dyn FnOnce() -> T + Send>` and closures that consume what they captured.

### Notes

- Do not add `Clone`, `Copy`, or `Debug`. Anything you require, a caller has to satisfy, and the tests call these with types that do not have them.
- Order comes from spawning everything first and joining afterwards, not from joining as you go.
- `handle.join()` returns a `Result`; unwrapping it is fine here.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `parallel_map` needs `T: Send + 'static`, `U: Send + 'static`, and `F: Fn(T) -> U + Send + Sync + 'static`.
- The `Sync` on `F` is what `Arc<F>: Send` demands. Drop it and the error names `Arc<F>` rather than `F`.
- `run_jobs` needs `T: Send + 'static` and `F: FnOnce() -> T + Send + 'static`. No `Sync`, because each job is moved into exactly one thread.
- Collect the `JoinHandle`s into a `Vec` before joining any of them. `.map(...).map(|h| h.join())` in one chain joins each thread immediately and runs everything sequentially.
- `jobs.into_iter().map(thread::spawn)` is enough for the spawn half of `run_jobs`.

</details>
