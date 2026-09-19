A consumer with nothing to do has two bad options: spin in a loop burning CPU, or sleep for a fixed time and add latency. `Condvar` is the third option. It parks the thread until another thread says the state has changed.

A condvar is always paired with a mutex. The mutex protects the data; the condvar carries the announcement.

```rust
let mut items = self.items.lock().unwrap();

while items.is_empty() {
    items = self.ready.wait(items).unwrap();
}
```

`wait` consumes the guard, releases the lock, parks the thread, and hands the guard back re-locked once it wakes. Giving up the lock is essential: the producer needs it to add anything.

The `while` is not a stylistic choice, and swapping it for an `if` is the classic bug. `wait` is allowed to return without any matching notification at all, and even a real notification can be stolen by another consumer that reached the lock first. Always re-check the condition after waking.

On the producer side:

```rust
self.items.lock().unwrap().push_back(item);
self.ready.notify_one();
```

A notification sent while nobody is waiting is simply lost, which is the other half of why the predicate must be checked before waiting rather than after.

`notify_one` wakes a single waiter; `notify_all` wakes every one of them, which you want when the state change could satisfy several different predicates.

`wait_timeout` is the version that gives up. It returns the guard plus a `WaitTimeoutResult`, and the predicate still decides the outcome:

```rust
let (guard, result) =
    self.ready.wait_timeout(items, timeout).unwrap();
```

## Your Task

Implement a small blocking queue.

```rust
pub struct Queue<T> {
    items: Mutex<VecDeque<T>>,
    ready: Condvar,
}
```

The `items` field is given. Add the `Condvar` and implement five methods.

### Implement `new`, `len` and `is_empty`

```rust
pub fn new() -> Self
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
```

An empty queue, the current number of waiting items, and whether that count is zero.

### Implement `push`

```rust
pub fn push(&self, item: T)
```

Adds `item` to the back and wakes one waiting consumer. Note the `&self`: pushing takes no `&mut`, because the mutex provides the mutability.

### Implement `pop`

```rust
pub fn pop(&self) -> T
```

Removes and returns the oldest item, blocking until one exists. Items come out in the order they went in.

### Implement `pop_timeout`

```rust
pub fn pop_timeout(&self, timeout: Duration) -> Option<T>
```

The same, but returns `None` if nothing arrives within `timeout`.

### Notes

- `Condvar::new()` takes no arguments and needs no relationship to the mutex declared up front.
- Drop the guard before calling `notify_one`, or the thread you just woke immediately blocks again on the lock you are still holding.
- Do not hold the lock across anything slow. The whole queue stalls while you do.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `push` can be one line plus the notify: `self.items.lock().unwrap().push_back(item);` ends the temporary guard's life at the semicolon.
- `pop` is: lock, `while` the deque is empty reassign the guard from `self.ready.wait(guard).unwrap()`, then `pop_front().unwrap()`.
- For `pop_timeout`, destructure `let (guard, result) = self.ready.wait_timeout(guard, timeout).unwrap();` and return `None` when the queue is still empty and `result.timed_out()`.
- `len` can lock and read; `is_empty` can just call `len`.

</details>
