Sharing state between threads costs something. An `Arc<Mutex<T>>` means an allocation, an atomic refcount, and a lock on every access. Sometimes that is exactly right. Often the threads never actually needed to see each other's data, and the whole apparatus is paid for nothing.

`thread_local!` gives each thread its own private copy of a static:

```rust
thread_local! {
    static CALLS: Cell<u64> = const { Cell::new(0) };
}
```

Every thread that touches `CALLS` gets a fresh `Cell` initialised on first use, dropped when that thread ends. There is no sharing, so there is nothing to lock. It is a static in the sense that it lives at the top level and outlives every function call, but it is not one value: there are as many as there are threads.

You never get a reference out. Access goes through `with`, which hands your closure a shared reference for the length of the call:

```rust
CALLS.with(|calls| calls.set(calls.get() + 1));
```

The reference cannot escape, which is precisely what makes it safe.

Because the reference is shared, the value has to provide its own interior mutability. `Cell<T>` is the cheap option for `Copy` types. For anything else, `RefCell<T>` does the borrow check at runtime, within the one thread that owns it:

```rust
thread_local! {
    static LOG: RefCell<Vec<String>> = const {
        RefCell::new(Vec::new())
    };
}

LOG.with_borrow_mut(|log| log.push("event".into()));
LOG.with_borrow(|log| log.len());
```

`with_borrow` and `with_borrow_mut` are shorthand for `with` plus `borrow`. The `const { ... }` initialiser is an optimisation: when the value can be built in a const context, the compiler skips the lazy initialisation check on every access.

## Your Task

Keep a per thread counter and a per thread log, then prove they do not leak between threads.

### Implement `bump`

```rust
pub fn bump() -> u64
```

Increment the calling thread's counter and return the new value. A thread that has never called it starts from `0`, so the first call returns `1`.

### Implement `record` and `events`

```rust
pub fn record(event: &str)
pub fn events() -> Vec<String>
```

`record` appends to the calling thread's log. `events` returns a copy of it, in the order things were recorded, and leaves the log alone.

```rust
record("open");
record("close");
assert_eq!(events(), vec!["open", "close"]);
```

### Implement `counts_per_thread`

```rust
pub fn counts_per_thread(bumps: &[u64]) -> Vec<u64>
```

Spawn one thread per entry, have it call `bump` that many times, and return each thread's final count in input order.

```rust
assert_eq!(counts_per_thread(&[3, 1, 2]), vec![3, 1, 2]);
```

The result being identical to the input is the lesson, not a coincidence. Each worker counts alone.

### Notes

- The counter and the log are separate pieces of state and do not interfere.
- `events` has to clone. The borrow only lives as long as the closure.
- A worker calling `bump` must not change the count on the thread that spawned it.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Two entries in one `thread_local!` block is fine, separated by semicolons.
- `Cell::get` and `Cell::set` are all `bump` needs. There is no `+=` on a `Cell`.
- `LOG.with_borrow(|log| log.clone())` is the whole of `events`.
- Collect the `JoinHandle`s first and join them afterwards. Joining inside the spawn loop runs the workers one at a time.
- `thread::scope` lets the closures borrow from `bumps` without any cloning.

</details>
