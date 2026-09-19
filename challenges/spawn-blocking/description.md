An async runtime runs your tasks on a small pool of worker threads. Tokio's default is one per CPU core. Every task on a worker shares that thread, and a task keeps it until it hits an `.await` and yields.

Blocking code never yields. A `std::thread::sleep`, a synchronous file read, a password hash, a tight numeric loop: while any of those runs, every other task queued on that worker is frozen. With four workers, four blocking calls stop the entire runtime.

`spawn_blocking` moves the work to a separate pool that exists for exactly this, and hands you a `JoinHandle` to await.

```rust
use tokio::task::spawn_blocking;

let handle = spawn_blocking(|| expensive(input));
let value = handle.await.unwrap();
```

The closure must be `Send + 'static`, because it runs on another thread. That usually means taking ownership with `move`.

## Async or blocking?

- Waiting on the network, a channel, a timer: async, `.await` it.
- Burning CPU, or calling a synchronous library: blocking, `spawn_blocking` it.

Roughly: if it does not have an `.await` in it and it is not instant, it belongs on the blocking pool.

## Your Task

`checksum` is written for you. It sleeps its thread for 30ms, then folds the bytes of a string into a `u64`. It is deliberately, honestly blocking.

Implement two async wrappers around it:

- `async checksum_one(data: String) -> u64`, one checksum, off the runtime's worker threads
- `async checksum_all(items: Vec<String>) -> Vec<u64>`, every input checksummed concurrently, results in input order

Eight payloads must not take eight times as long as one, and a timer running alongside `checksum_all` must keep firing while the checksums are in flight.

### Notes

- Spawn every closure first, collect the handles, and await them afterwards. Awaiting inside the loop makes the work sequential again.
- `handle.await` gives a `Result`. The task only fails if the closure panicked, so unwrapping it is fine here.
- `spawn_blocking` cannot be cancelled once it starts. Dropping the handle just discards the result.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `checksum` takes `&str`, so a `move` closure that owns a `String` can call `checksum(&data)`.
- For `checksum_all`, `items.into_iter().map(...).collect::<Vec<_>>()` gives you the handles in one line.
- Then a plain `for handle in handles` loop pushes each awaited value onto the results.
- The order falls out for free: you await the handles in the order you spawned them.

</details>
