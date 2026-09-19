A panic unwinds only the thread it happens on. The rest of the program keeps running, which is exactly why `JoinHandle::join` returns a `Result` rather than the value directly. If the worker panicked, the calling thread finds out by getting an `Err` back instead of being taken down with it.

The error side is `Box<dyn Any + Send>`, the **panic payload**. It is deliberately untyped, because a panic can carry anything. In practice it carries one of two things: `panic!("literal")` stores a `&'static str`, and a formatted `panic!("{}", value)` stores a `String`. Reading the message means downcasting to each of those in turn.

This is also why blindly writing `handle.join().unwrap()` is worth thinking about. It works, but it converts a worker's panic into a panic in your own thread. Sometimes that is what you want. Often you would rather turn it into a value and decide what to do.

## Your Task

Implement two functions.

### Implement `describe_panic`

```rust
pub fn describe_panic(
    job: Box<dyn FnOnce() + Send + 'static>,
) -> Option<String>
```

Run `job` on a spawned thread and join it.

- Finished normally: return `None`.
- Panicked: return `Some(message)` with the panic message.
- Payload that is neither a `&'static str` nor a `String`: return `Some("unknown panic")`.

```rust
let ok = describe_panic(Box::new(|| {}));
assert_eq!(ok, None);

let bad = describe_panic(Box::new(|| panic!("boom")));
assert_eq!(bad, Some("boom".to_string()));
```

### Implement `divide_in_thread`

```rust
pub fn divide_in_thread(
    a: i32,
    b: i32,
) -> Result<i32, String>
```

Compute `a / b` on a spawned thread. When `b` is 0 the thread must panic with the message `"division by zero"`. Join it and turn a panic into `Err(message)`.

```rust
assert_eq!(divide_in_thread(10, 2), Ok(5));
assert_eq!(
    divide_in_thread(1, 0),
    Err("division by zero".to_string()),
);
```

### Notes

- Your job is to panic and then recover from it, not to avoid the panic. Do not check for zero and return early.
- The test output will show panic messages on stderr. That is expected and does not mean a test failed.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `match handle.join() { Ok(value) => ..., Err(payload) => ... }`.
- `payload.downcast_ref::<&'static str>()` covers `panic!("literal")`.
- `payload.downcast_ref::<String>()` covers `panic!("{}", x)`. Try both before giving up.
- `downcast_ref` hands you a reference, so finish with `.to_string()` or `.clone()`.
- For the second function, `join()` gives a `Result` already, so `map_err` is a tidy way to convert only the error side.

</details>
