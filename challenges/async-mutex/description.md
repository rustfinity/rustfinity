`std::sync::Mutex` and `tokio::sync::Mutex` look almost the same. The difference is what happens when the lock is already taken.

`std::sync::Mutex::lock` blocks the thread. On an async runtime that thread is a worker shared by many tasks, so blocking it stalls all of them. Worse, an async runtime may have far fewer threads than tasks, and enough blocked workers is a deadlock.

`tokio::sync::Mutex::lock` is `async`. It yields, the runtime runs something else, and your task wakes when the lock is free.

```rust
use tokio::sync::Mutex;

let guard = shared.lock().await;
```

## The rule about guards and awaits

A `std::sync::MutexGuard` is not `Send`. Hold one across an `.await` and the whole future stops being `Send`, so `tokio::spawn` refuses it. The compiler catches the mistake, and the error is confusing the first time you hit it.

So: if the critical section contains an `.await`, use `tokio::sync::Mutex`. If it does not, keep `std::sync::Mutex`, which is faster and simpler.

Holding a lock across an await is still a real cost. Every other caller is stopped for the duration. Sometimes that is the entire point.

## Your Task

Build a cache whose misses are filled by an async loader, where a key that several tasks ask for at once is loaded exactly once.

`load_value` is written for you. It sleeps, then returns a string.

```rust
pub struct Cache {
    entries: Mutex<HashMap<String, String>>,
    loads: AtomicUsize,
}
```

Implement four methods:

- `new() -> Self`, an empty cache
- `async get_or_load(&self, key: &str) -> String`, the value for `key`, calling `load_value` only on a miss
- `loads(&self) -> usize`, how many times the loader has run
- `async snapshot(&self) -> Vec<(String, String)>`, every cached pair sorted by key

Eight tasks calling `get_or_load("config")` at the same time must leave `loads()` at `1`, and all eight must get the same value.

### Notes

- The dedup falls out of holding the guard across the load. Do not drop it and take it again.
- Sorting a `Vec<(String, String)>` sorts by key first, which is what you want.
- `Ordering::SeqCst` is a fine default for the counter here.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `#[derive(Default)]` is already on the struct, so `new` can be `Self::default()`.
- In `get_or_load`, take `let mut entries = self.entries.lock().await;` first, before anything else.
- Return early with a clone if `entries.get(key)` is `Some`.
- On a miss: await the load, bump the counter, insert a clone, return the value.
- For `snapshot`, collect cloned pairs out of the map, then `sort()`.

</details>
