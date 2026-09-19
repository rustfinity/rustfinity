An `mpsc` channel is *multi producer, single consumer*. The `Sender` can be cloned as many times as you like; the `Receiver` cannot. That makes it the natural fit for fan-in: many workers producing, one thread collecting.

```rust
let (tx, rx) = mpsc::channel();

for chunk in chunks {
    let tx = tx.clone();
    thread::spawn(move || {
        for value in chunk {
            tx.send(value).unwrap();
        }
    });
}
```

Now the part that catches everybody. Iterating a receiver:

```rust
for value in rx {
    // ...
}
```

ends when the channel is *disconnected*, which means when the last `Sender` has been dropped. The workers drop their clones as they finish, but the `tx` that `channel()` handed you is still alive in this function, so the receiver keeps waiting for a message that will never come. The loop hangs.

The fix is one line:

```rust
drop(tx);
```

Drop your own sender once every clone has been handed out. Then the last worker to finish disconnects the channel and the loop ends on its own.

The other thing fan-in costs you is identity. Every message lands in one stream, so "which worker sent this" is gone unless you put it in the message:

```rust
tx.send((index, value)).unwrap();
```

Arrival order is not something you control either. A worker with less to do finishes first regardless of when it started, so any result that has to be ordered must be sorted or indexed after the fact, never assumed.

## Your Task

Implement two functions.

### Implement `collect_all`

```rust
pub fn collect_all(chunks: Vec<Vec<u64>>) -> Vec<u64>
```

Give each chunk its own thread, send every value through a single shared channel, and return everything that arrived in ascending order.

```rust
let chunks = vec![vec![3, 1], vec![2], vec![]];
assert_eq!(collect_all(chunks), vec![1, 2, 3]);
```

Duplicates are kept. An empty input, or input of only empty chunks, gives an empty vector.

### Implement `sum_by_worker`

```rust
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) -> Vec<u64>
```

The same fan-in, but the result has one entry per chunk holding that chunk's sum, in input order. Empty chunks sum to 0.

```rust
let chunks = vec![vec![1, 2, 3], vec![], vec![10]];
assert_eq!(sum_by_worker(chunks), vec![6, 0, 10]);
```

Messages arrive in an order you do not choose, so the worker index has to travel with the value.

### Notes

- `thread::scope` lets a worker borrow without an `Arc`, but the chunk itself should be moved in.
- `rx.iter()` and `for value in rx` both stop at disconnection. Neither will stop if you still hold a sender.
- `send` returns a `Result`, which is only an error if the receiver is gone.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The order is: create the channel, spawn all workers with `tx.clone()`, `drop(tx)`, *then* drain the receiver.
- If a test hangs instead of failing, you forgot the `drop`.
- `collect_all` can `let mut out: Vec<u64> = rx.iter().collect();` and then `out.sort_unstable()`.
- For `sum_by_worker`, allocate `vec![0; chunks.len()]` first, then `for (index, value) in rx { sums[index] += value; }`.
- `chunks.into_iter().enumerate()` gives you the index and the owned chunk together.

</details>
