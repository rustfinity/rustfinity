`thread::spawn` accepts any closure that is `Send + 'static`. The `'static` part is the strict one: the spawned thread might still be running long after the function that created it has returned, so the closure is not allowed to borrow anything from that function's stack.

The `move` keyword is how you satisfy that. It changes capture mode: instead of borrowing the variables it mentions, the closure takes ownership of them. The data now lives inside the closure, which lives inside the thread, so there is no dangling reference to worry about.

Ownership being ownership, only one thread can have it. If two threads each need the same `String`, one of them needs its own copy. That is not a workaround, it is the rule being consistent: `move` really does hand the value over.

## Your Task

Implement two functions.

### Implement `count_chars_in_thread`

```rust
pub fn count_chars_in_thread(
    words: Vec<String>,
) -> usize
```

Move `words` into a spawned thread, count the characters across all of them, join, and return the total.

```rust
let words = vec![
    "ab".to_string(),
    "cde".to_string(),
];
assert_eq!(count_chars_in_thread(words), 5);
```

### Implement `greet_each`

```rust
pub fn greet_each(
    prefix: String,
    names: Vec<String>,
) -> Vec<String>
```

Spawn one thread per name. Each thread builds `"{prefix}, {name}!"`. Return the greetings **in the same order as `names`**, whatever order the threads happen to finish in.

```rust
let names = vec![
    "Ada".to_string(),
    "Bo".to_string(),
];
let out = greet_each("Hi".to_string(), names);
assert_eq!(out, vec!["Hi, Ada!", "Hi, Bo!"]);
```

### Notes

- Count characters, not bytes, so `"héllo"` is 5.
- Order is part of the contract. Spawning all the threads first and then joining them in the same order is enough to guarantee it, and it also keeps the threads running concurrently.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `w.chars().count()` counts characters; `w.len()` counts bytes.
- Only one thread can own `prefix`, so give each closure its own `prefix.clone()` before the `move`.
- Collect into a `Vec` of handles first. If you join inside the same loop that spawns, each thread finishes before the next one starts and you have written a slow sequential loop.
- `handles.into_iter().map(|h| h.join().unwrap()).collect()` turns the handles back into results, in order.

</details>
