`std::thread::sleep` parks the whole thread. Inside an async runtime that is a disaster: one task naps and every other task sharing that worker thread stops making progress.

`tokio::time::sleep` is the async version. It yields, the runtime goes off and polls everything else, and your task is woken when the timer fires.

```rust
use tokio::time::sleep;
use std::time::Duration;

sleep(Duration::from_millis(50)).await;
```

## Putting a limit on a future

`tokio::time::timeout` wraps any future and races it against a timer.

```rust
use tokio::time::timeout;

let limit = Duration::from_millis(100);

match timeout(limit, slow_work()).await {
    Ok(value) => println!("got {value}"),
    Err(_) => println!("gave up"),
}
```

The error type, `Elapsed`, carries nothing beyond "the time was up", so `.ok()` is usually all you want from it.

The important part is what happens to the inner future when the timer wins: it is **dropped**. Not signalled, not asked to stop. Dropped, part-finished, at whatever `.await` it was suspended on. Anything after that point never runs. This is what people mean when they say async Rust has cancellation built in.

## Your Task

### Implement `with_timeout`

```rust
pub async fn with_timeout<F>(
    limit: Duration,
    work: F,
) -> Option<F::Output>
where
    F: Future,
```

Run `work` with a deadline. Return `Some(output)` if it finished in time, `None` if the limit ran out first.

### Implement `retry`

```rust
pub async fn retry(
    attempts: usize,
    gap: Duration,
    op: impl FnMut() -> bool,
) -> Option<usize>
```

Call `op` until it returns `true`, at most `attempts` times, waiting `gap` between one call and the next. Return the zero-based number of the attempt that succeeded, or `None` if none did.

The gap goes **between** attempts only. One attempt sleeps not at all, four failing attempts sleep three times.

```rust
let mut tries = 0;
let found = retry(5, gap, || {
    tries += 1;
    tries == 3
})
.await;

assert_eq!(found, Some(2));
```

### Notes

- `op` needs to be `mut` to be called, since it is `FnMut`.
- `attempts` of `0` means `op` is never called at all.
- Keep every duration small. The test runner has a hard timeout.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `timeout(limit, work).await.ok()` is the whole body of `with_timeout`.
- Take the closure as `mut op` in the parameter list.
- Loop over `0..attempts` and sleep at the top of the body whenever the index is not `0`. That is easier to get right than sleeping at the bottom.
- Return from inside the loop the moment `op()` is true, so no further attempt happens.
- Fall through to `None` after the loop.

</details>
