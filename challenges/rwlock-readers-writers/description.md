A `Mutex` gives exactly one thread access at a time, whether that thread is changing the data or only looking at it. For data that is read far more often than it is written, that is a waste: ten threads reading the same map cannot interfere with each other, yet a mutex makes nine of them wait.

`RwLock<T>` splits the lock in two:

- `read()` returns a shared guard. **Many** readers can hold one at the same time.
- `write()` returns an exclusive guard. While it is held, nothing else gets in, reader or writer.

```rust
let lock = RwLock::new(vec![1, 2, 3]);

let a = lock.read().unwrap();
let b = lock.read().unwrap(); // fine, both alive
assert_eq!(a.len(), b.len());
```

Both methods return a `Result` for the same reason `Mutex::lock` does: the lock is poisoned if a writer panics while holding it.

`RwLock` is the right pick when reads dominate and each one is long enough that the extra bookkeeping pays for itself. For a lock held for a couple of instructions, a plain `Mutex` is usually faster.

## Your Task

Implement `WordCounts`, a tally where one method writes and three read.

```rust
pub struct WordCounts {
    counts: RwLock<HashMap<String, usize>>,
}
```

### Implement `new`

```rust
pub fn new() -> Self
```

An empty tally.

### Implement `record`

```rust
pub fn record(&self, word: &str)
```

Adds one sighting of `word`. This is the only method that takes a write lock.

### Implement `count` and `total`

```rust
pub fn count(&self, word: &str) -> usize
pub fn total(&self) -> usize
```

`count` returns how many times `word` was recorded, or 0 if it was never seen. `total` returns the sum over every word. Both take read locks.

### Implement `most_common`

```rust
pub fn most_common(&self)
    -> Option<(String, usize)>
```

The word with the highest count, and that count. Returns `None` for an empty tally. **Ties are broken by the alphabetically smallest word**, so the result never depends on hash order or thread timing.

```rust
let counts = WordCounts::new();
counts.record("zebra");
counts.record("ant");
assert_eq!(
    counts.most_common(),
    Some(("ant".to_string(), 1))
);
```

### Notes

- Words are compared exactly. `"Rust"` and `"rust"` are two different words.
- Take the read lock in the three readers. Using `write()` everywhere passes the tests but throws away the whole point of `RwLock`.
- Never hold a read guard and then ask for a write guard on the same lock in the same thread. That deadlocks.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `*guard.entry(word.to_string()).or_insert(0) += 1;` inserts a zero on first sight and bumps it.
- `HashMap::get` returns `Option<&usize>`; `.copied().unwrap_or(0)` turns it into the `usize` you want.
- `guard.values().sum()` gives the total.
- For `most_common`, iterate and compare with `min_by`: sort descending on the count first, then ascending on the word with `.then_with(...)`.
- Both `read()` and `write()` return a `Result`, so `.unwrap()` after each is expected here.

</details>
