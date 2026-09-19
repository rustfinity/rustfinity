Awaiting a future runs it, one at a time. To get two futures making progress at once you have to hand them to the runtime as separate **tasks**:

```rust
let handle = tokio::spawn(async {
    expensive().await
});
```

`tokio::spawn` returns immediately with a `JoinHandle`. The task is now the runtime's problem, and it keeps running whether or not you ever await the handle.

A task is not a thread. A thread costs the OS a stack, usually a couple of hundred kilobytes; a task is a state machine on the heap that often fits in a few hundred bytes. Spawning a hundred thousand tasks is normal. Spawning a hundred thousand threads is not.

## Spawn first, await second

This is the mistake to avoid:

```rust
for ms in delays {
    let value = tokio::spawn(work(ms)).await.unwrap();
    results.push(value);
}
```

That spawns a task and immediately blocks on it, so the next task is not created until the previous one is done. It is sequential code with extra steps. Collect the handles first:

```rust
let handles: Vec<_> = delays
    .into_iter()
    .map(|ms| tokio::spawn(work(ms)))
    .collect();

for handle in handles {
    results.push(handle.await.unwrap());
}
```

Now every task is running while you walk the list, and awaiting in order gives you results in input order even though they finish out of order.

## Spawned tasks need to own their data

The closure body moves to another task, which may outlive the caller, so the future must be `'static`. Use `async move` and clone anything you need to keep.

## A `JoinHandle` gives you a `Result`

```rust
match handle.await {
    Ok(value) => ...,
    Err(join_error) => ...,
}
```

The `Err` is a `JoinError`: the task panicked, or it was aborted. A panic inside a task does not unwind into your code and does not stop the runtime, it just turns that one handle into an `Err`.

## Your Task

### Implement `run_all`

```rust
pub async fn run_all(delays_ms: Vec<u64>) -> Vec<u64>
```

Spawn one task per entry. Each task sleeps for its own number of milliseconds with `tokio::time::sleep`, then returns that number. Return the values in input order.

The whole call must take about as long as the single slowest delay, not the sum of them.

### Implement `checked_divide`

```rust
pub async fn checked_divide(
    a: i64,
    b: i64,
) -> Result<i64, String>
```

Spawn a task that computes `a / b` and await its handle. Integer division by zero panics, so return `Ok(quotient)` when the task finished and `Err("task panicked".to_string())` when it did not.

```rust
assert_eq!(checked_divide(10, 2).await, Ok(5));
assert_eq!(
    checked_divide(1, 0).await,
    Err("task panicked".to_string())
);
```

### Notes

- Do not add your own check for `b == 0`. The point is to let the task panic and read that off the handle.
- The panic message printed to stderr by the panicking task is expected.
- `Duration::from_millis(ms)` builds the sleep duration.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Build a `Vec<JoinHandle<u64>>` in one loop, then drain it in a second loop.
- `tokio::spawn(async move { sleep(Duration::from_millis(ms)).await; ms })`.
- `for ms in delays_ms` moves each `u64` out, so `async move` captures a copy.
- `handle.await` is `Result<u64, JoinError>`; use `.expect(...)` in `run_all` where no task can panic.
- In `checked_divide`, `match handle.await { Ok(v) => Ok(v), Err(_) => Err(...) }`.

</details>
