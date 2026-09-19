`std::sync::mpsc` blocks the thread when you call `recv()`. Inside an async runtime that is a bug: the worker thread stops polling every other task on it while it waits. Async code needs a channel whose "wait" is an `.await`.

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(32);
```

`tx.send(value).await` and `rx.recv().await` both yield instead of blocking, so the runtime can run other tasks while this one waits.

## Bounded or unbounded

`mpsc::channel(capacity)` is bounded. Once `capacity` messages are queued, `send` waits for the consumer to take one. That wait is **backpressure**: a producer that outruns its consumer is slowed down instead of filling memory.

`mpsc::unbounded_channel()` never makes the producer wait, and its `send` is not even `async`. It is the right tool when the producer physically cannot outrun the consumer, and a memory leak waiting to happen otherwise. Prefer bounded, and pick a capacity.

## `None` means finished

```rust
while let Some(value) = rx.recv().await {
    // ...
}
```

`recv()` returns `None` when the channel is empty **and** every `Sender` has been dropped. Until then it waits, however long that takes.

This is the part that catches people out. The channel counts senders, and clones count. If the task that owns the original `tx` finishes but a clone is still alive somewhere, the loop above never ends. Move the sender into the producing task so it is dropped when that task returns, or `drop(tx)` explicitly once you are done sending.

`send` has the mirror-image behaviour: it returns `Err(SendError(value))` when the receiver is gone, handing your value back to you.

## Your Task

### Implement `drain`

```rust
pub async fn drain(rx: mpsc::Receiver<i32>) -> Vec<i32>
```

Receive everything until the channel closes, and return the values in the order they arrived.

### Implement `produce_and_sum`

```rust
pub async fn produce_and_sum(n: i32, capacity: usize) -> i64
```

Build a bounded channel of the given capacity. Spawn a task that sends `0`, `1`, ... up to but not including `n`. Receive them all and return the sum.

```rust
assert_eq!(produce_and_sum(5, 4).await, 10);
assert_eq!(produce_and_sum(500, 1).await, 124_750);
```

A capacity of `1` means the producer spends most of its life waiting. It must still finish, and the totals must match.

### Notes

- `rx` needs to be mutable to receive from, and `drain` takes it by value.
- Sum into an `i64`. The totals overflow `i32` less easily that way.
- Do not join the producer task. Waiting for `None` is what tells you it is done.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `pub async fn drain(mut rx: ...)` puts the `mut` in the pattern, no signature change needed.
- `while let Some(value) = rx.recv().await { ... }` is the whole loop.
- `let (tx, mut rx) = mpsc::channel(capacity);` then `tokio::spawn(async move { ... })` captures `tx` by move.
- Inside the producer, `tx.send(value).await` returns a `Result`; `break` on `Err` or unwrap it.
- If a test hangs, a `Sender` is still alive somewhere outside the producer task.

</details>
