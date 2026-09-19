Threads let you run work on more than one CPU. Async lets you keep thousands of half-finished operations in flight on very few threads. Both are concurrency, but they solve different problems: threads are for CPU work, async is for waiting.

Marking a function `async` changes what it returns:

```rust
async fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

The signature says `-> String`, but the compiler rewrites it to return `impl Future<Output = String>`. You get the `String` back by awaiting it:

```rust
let greeting = greet("Ada").await;
```

## Futures are lazy

This is the part that surprises people coming from other languages. A JavaScript promise starts running the moment you create it. A Rust future does nothing at all until something polls it:

```rust
let fut = greet("Ada"); // no work has happened
let s = fut.await;      // now the body runs
```

Drop the future without awaiting it and the body never runs. There is no background thread, no scheduler entry, nothing. A future is a plain value describing work that has not started.

## Something has to poll it

Because futures are inert, `.await` only works inside another `async` context, and at the top there has to be a runtime that does the polling. In tests that runtime comes from a macro:

```rust
#[tokio::test]
async fn it_greets() {
    assert_eq!(greet("Ada").await, "Hello, Ada!");
}
```

`#[tokio::test]` replaces `#[test]`, builds a small runtime, and drives your async test body to completion. For a binary the equivalent is `#[tokio::main]` on `main`.

## Your Task

Write two async functions.

### Implement `greet`

```rust
pub async fn greet(name: &str) -> String
```

Return `"Hello, {name}!"`.

### Implement `greet_and_count`

```rust
pub async fn greet_and_count(
    calls: &AtomicUsize,
    name: &str,
) -> String
```

Add one to `calls`, then return the result of awaiting `greet(name)`.

The counter is there so the tests can prove laziness. Building the future must leave `calls` at zero; only awaiting it may bump the count.

```rust
let calls = AtomicUsize::new(0);

let fut = greet_and_count(&calls, "Ada");
assert_eq!(calls.load(Ordering::SeqCst), 0);

assert_eq!(fut.await, "Hello, Ada!");
assert_eq!(calls.load(Ordering::SeqCst), 1);
```

### Notes

- You do not need to write `impl Future` anywhere. `async fn` does it for you.
- `greet_and_count` should call `greet`, not rebuild the string itself.
- Use `Ordering::SeqCst` for the counter unless you have a reason not to.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `format!("Hello, {name}!")` is the whole body of `greet`.
- `calls.fetch_add(1, Ordering::SeqCst);` adds one and returns the old value, which you can ignore.
- Do not forget the `.await` on `greet(name)`, or you will try to return a future where a `String` is expected.
- Laziness is free here: it comes from `async fn`, so if you write the body normally the tests pass.

</details>
