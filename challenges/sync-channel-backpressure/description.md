`mpsc::channel()` gives you an *unbounded* queue. `send` never blocks, never fails while the receiver lives, and never says "slow down". That sounds convenient until the producer is faster than the consumer, which it usually is. The queue then grows for as long as the program runs, and the only thing that eventually stops it is the allocator.

`mpsc::sync_channel(capacity)` is the bounded version:

```rust
let (tx, rx) = mpsc::sync_channel(4);
```

At most four items sit in the buffer. The fifth `send` parks the producer thread until the consumer takes one out. That pause is **backpressure**: the consumer's speed becomes the producer's speed limit, and memory use stays flat no matter how long the pipeline runs.

The extreme case is capacity `0`, called a *rendezvous* channel. Nothing is buffered at all. A `send` waits for a matching `recv`, and the two threads meet in the middle:

```rust
let (tx, rx) = mpsc::sync_channel(0);
```

When you do not want to wait, `try_send` returns instead of blocking:

```rust
match tx.try_send(item) {
    Ok(()) => { /* it fitted */ }
    Err(TrySendError::Full(item)) => { /* no room */ }
    Err(TrySendError::Disconnected(item)) => { /* gone */ }
}
```

Notice the item comes back to you in the error. Nothing is lost, and you get to decide whether to drop it, retry, or give up.

## Your Task

Implement two functions.

### Implement `drain`

```rust
pub fn drain(items: Vec<u64>, capacity: usize) -> Vec<u64>
```

Create a bounded channel with the given capacity, send every item from a worker thread, and collect them on the calling thread in the order they were sent.

```rust
assert_eq!(drain(vec![1, 2, 3], 1), vec![1, 2, 3]);
```

There is one producer, so arrival order is send order and no sorting is needed. This must work for a capacity of `0` and for inputs far larger than the capacity.

### Implement `fill_without_receiving`

```rust
pub fn fill_without_receiving(
    items: Vec<u64>,
    capacity: usize,
) -> usize
```

Create a bounded channel, push items with `try_send` while nobody is receiving, and return how many were accepted before the channel reported itself full.

```rust
assert_eq!(fill_without_receiving(vec![1, 2, 3, 4], 2), 2);
assert_eq!(fill_without_receiving(vec![1, 2, 3, 4], 0), 0);
```

Stop at the first refusal. If the input runs out first, the answer is simply how many items there were.

### Notes

- `try_send` is on `SyncSender`, not on the plain `Sender`.
- The receiver has to stay alive for the whole of `fill_without_receiving`, otherwise the errors become `Disconnected` rather than `Full`.
- A blocked `send` is not a deadlock as long as somebody is still receiving.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `drain` is the same shape as any other `mpsc` producer or consumer pair: spawn, send, drop the sender, then `rx.iter().collect()`.
- `thread::scope` avoids needing `'static` here, and moving `items` into the worker means the closure owns them.
- Bind the receiver to `_rx` in `fill_without_receiving` so it lives to the end of the function. A bare `_` would drop it immediately.
- Match on `Err(TrySendError::Full(_))` and `break`.
- With no receiver waiting, a rendezvous channel has nowhere to put anything, so the answer for capacity `0` is `0`.

</details>
