`Mutex::lock` does not return a guard. It returns a `Result<MutexGuard<T>, PoisonError<...>>`, and most code papers over that with `.unwrap()`. That `unwrap` is not there for the usual "this can't fail" reason: it is there because a mutex can be **poisoned**.

A mutex becomes poisoned when a thread panics while holding its guard. Rust assumes the panicking thread was halfway through a change and left the data in a state the rest of the program was never meant to see, so every later `lock()` call returns `Err` as a warning.

The data itself is untouched and still reachable. `PoisonError` owns the guard, and `into_inner` gives it to you:

```rust
let guard = mutex
    .lock()
    .unwrap_or_else(|e| e.into_inner());
```

Whether that is the right call depends on the invariant. For a plain counter, a half-finished increment is not dangerous, so recovering is reasonable. For a data structure with an invariant spanning several fields, it may not be.

## Your Task

Implement `ResilientCounter`, a counter that never panics on a poisoned lock.

```rust
pub struct ResilientCounter {
    value: Mutex<i64>,
}
```

### Implement `new`

```rust
pub fn new(start: i64) -> Self
```

Creates a counter holding `start`.

### Implement `with`

```rust
pub fn with<R>(
    &self,
    f: impl FnOnce(&mut i64) -> R,
) -> R
```

Takes the lock, calls `f` with a mutable reference to the value, and returns whatever `f` returned. If the mutex is poisoned, recover the guard and run `f` anyway.

```rust
let counter = ResilientCounter::new(0);
counter.with(|value| *value += 21);
assert_eq!(counter.get(), 21);
```

### Implement `get`

```rust
pub fn get(&self) -> i64
```

Returns the current value, poisoned or not.

### Implement `is_poisoned`

```rust
pub fn is_poisoned(&self) -> bool
```

Reports whether a thread has ever panicked while holding the lock. Recovering the guard does **not** clear the flag, so this stays `true` afterwards.

### Notes

- The lock is only poisoned if the panic happens while the guard is alive. A thread that locks, releases, and then panics leaves the mutex healthy.
- `with` must not hold the lock after it returns. Locking inside `with` and releasing at the end of `with` is what makes nesting easy to reason about.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `unwrap_or_else` on a `Result` gives you the error value to work with: `self.value.lock().unwrap_or_else(|e| e.into_inner())`.
- `into_inner` consumes the `PoisonError` and returns the `MutexGuard` that was inside it.
- A `MutexGuard<i64>` derefs to `i64`, so `f(&mut guard)` coerces to the `&mut i64` that `f` expects.
- `get` does not need its own locking code. Write it as `self.with(|value| *value)`.
- `Mutex` has an `is_poisoned` method already, so the last one is a single line.

</details>
