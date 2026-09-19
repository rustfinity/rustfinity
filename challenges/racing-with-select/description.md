`join!` waits for everything. Sometimes you want the opposite: whichever of several things happens first, and none of the rest.

`tokio::select!` polls every branch and completes as soon as one of them does.

```rust
tokio::select! {
    value = fetch_from_cache() => use_it(value),
    value = fetch_from_disk()  => use_it(value),
}
```

Each arm is `pattern = future => expression`. Every arm must produce the same type, exactly like a `match`.

## Every loser is cancelled

When one branch wins, the others are dropped mid-flight. Whatever they had done so far is thrown away.

That makes `select!` in a loop a sharp tool. A future that is safe to drop half-finished and poll again from scratch is called **cancel safe**. `Receiver::recv` is: a message is either taken or still in the channel, never lost in between. Reading half a line from a socket into a local buffer is not.

If several branches are ready at once, `select!` picks between them at random, so never write code that depends on branch order.

## Reusing a future across iterations

A future created inside a loop body starts over on every pass. For a deadline that is exactly wrong: the timer resets after each message and never fires. Create it once, pin it, then poll the same one with `&mut`.

```rust
let deadline = sleep(limit);
tokio::pin!(deadline);

loop {
    tokio::select! {
        v = rx.recv() => { /* ... */ }
        _ = &mut deadline => break,
    }
}
```

`tokio::pin!` pins the future in place on the stack, which is what lets you take `&mut` of it.

## Your Task

### Implement `first_of`

```rust
pub async fn first_of<A, B>(a: A, b: B) -> String
where
    A: Future<Output = String>,
    B: Future<Output = String>,
```

Return whichever future produces a value first, and drop the other.

### Implement `collect_until_deadline`

```rust
pub async fn collect_until_deadline(
    rx: &mut mpsc::Receiver<i32>,
    limit: Duration,
) -> Vec<i32>
```

Collect messages in arrival order and stop at whichever comes first: the channel closing, or `limit` elapsing since the call started.

A producer sending steadily forever must not keep the collection running past `limit`.

### Notes

- The deadline is measured from the start of the call, not from the last message.
- `rx.recv()` yields `None` when every sender is gone. That ends the loop too.
- Both branches of a `select!` arm must return the same type, so do the work in a block.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `first_of` is one `select!` with two arms, each binding a value and returning it.
- In `collect_until_deadline`, build the sleep and `tokio::pin!` it before the loop.
- Match on the `Option` the receive arm produces: push on `Some`, `break` on `None`.
- The deadline arm ignores its output and just breaks.
- If a test hangs, the deadline is being created inside the loop instead of outside it.

</details>
