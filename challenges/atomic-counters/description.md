A `Mutex<usize>` around a counter works, but it is heavy for the job. Taking a lock means a guard, a possible block, and a poisoning story, all so one thread at a time can run `+= 1`.

The atomic types in `std::sync::atomic` do that increment directly. `AtomicUsize::fetch_add` is one instruction that reads, adds, and writes back without any other thread being able to slip in between. There is no guard, nothing to unwrap, and nothing to poison.

```rust
let hits = AtomicUsize::new(0);
hits.fetch_add(1, Ordering::Relaxed);
assert_eq!(hits.load(Ordering::Relaxed), 1);
```

Every atomic method takes an `Ordering`, which says how strongly this operation is ordered against the *other* memory your thread touches:

- `Relaxed` guarantees the atomic operation itself is not torn or lost, and nothing more. This is what a standalone statistics counter wants.
- `Acquire` / `Release` pair up to publish other writes: a `Release` store makes everything the thread wrote beforehand visible to a thread that later does an `Acquire` load.
- `SeqCst` adds a single global order over all `SeqCst` operations. It is the safest default and the slowest.

If the counter is only ever read for its own sake, `Relaxed` is correct. If reading it is a signal that some *other* data is ready, you need `Acquire` / `Release`.

`swap` is the other operation worth knowing: it stores a new value and returns the old one in the same atomic step. That makes "read and reset" safe even while other threads are still incrementing.

## Your Task

Implement `CacheStats`, a pair of lock-free counters.

```rust
pub struct CacheStats {
    hits: AtomicUsize,
    misses: AtomicUsize,
}
```

### Implement `new`

```rust
pub fn new() -> Self
```

Both counters at zero.

### Implement `record`

```rust
pub fn record(&self, hit: bool)
```

Adds one to `hits` when `hit` is true, otherwise one to `misses`. Note it takes `&self`, not `&mut self`: that is the whole appeal.

### Implement `snapshot`

```rust
pub fn snapshot(&self) -> (usize, usize)
```

Returns `(hits, misses)` without changing anything.

### Implement `drain`

```rust
pub fn drain(&self) -> (usize, usize)
```

Returns `(hits, misses)` **and** resets both counters to zero. A lookup recorded before the reset must never be lost, so read and reset each counter in one atomic step rather than loading and then storing.

```rust
let stats = CacheStats::new();
stats.record(true);
stats.record(true);

assert_eq!(stats.drain(), (2, 0));
assert_eq!(stats.snapshot(), (0, 0));
```

### Notes

- `Relaxed` is the right ordering throughout this challenge, and the tests do not check which one you used. Understanding *why* it is enough matters more than the keyword.
- `drain` reads two separate counters, so it is not atomic as a whole. That is fine for statistics; it would not be for an invariant spanning both.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `AtomicUsize` implements `Default`, so `#[derive(Default)]` on the struct gives you `new` as `Self::default()`.
- Pick the counter first and then bump it: `let counter = if hit { &self.hits } else { &self.misses };`.
- Reading is `load(Ordering::Relaxed)`.
- `swap(0, Ordering::Relaxed)` returns the previous value and leaves zero behind. That is what `drain` needs on each counter.

</details>
