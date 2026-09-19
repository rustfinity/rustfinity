Three lookups, awaited one after the other:

```rust
let name = load_name(id).await;
let email = load_email(id).await;
let posts = load_posts(id).await;
```

If each takes 40ms this takes 120ms, and for 80 of those 120 milliseconds the program is sitting on its hands. The three lookups do not depend on each other, so they should be waiting at the same time.

`tokio::join!` does exactly that:

```rust
let (name, email, posts) = tokio::join!(
    load_name(id),
    load_email(id),
    load_posts(id),
);
```

Now the call takes 40ms. `join!` takes the futures, polls each one whenever it can make progress, and returns a tuple of their outputs in the order you listed them, once all of them are done.

## `join!` is not `spawn`

`tokio::spawn` hands a future to the runtime as an independent task. `join!` keeps everything on the current task and just interleaves the polling. That has real consequences:

- Nothing is moved to another thread, so the futures do not have to be `Send` or `'static`. They can borrow local variables.
- There is no `JoinHandle` and no `Result` to unwrap.
- It is concurrency, not parallelism. Two CPU-bound futures under `join!` will not use two cores.

Reach for `join!` when you have a fixed, known set of things to wait on. Reach for `spawn` when the work should outlive the current scope or genuinely needs its own thread.

## Failing fast with `try_join!`

`join!` waits for everything, even after one branch has already returned an `Err`. When every branch returns a `Result` and one failure makes the rest pointless, use `try_join!`:

```rust
let (a, b) = tokio::try_join!(load_a(), load_b())?;
```

It returns `Ok((a, b))` when both succeeded, or the first `Err` it sees, dropping the other future mid-flight. Both branches must have the same error type.

## Your Task

Five lookup functions are already written for you: `load_name`, `load_email`, `load_posts`, `load_quota` and `load_rate_limit`. Each one sleeps for 40ms. Do not change them.

### Implement `load_profile`

```rust
pub async fn load_profile(id: u32) -> Profile
```

Run `load_name`, `load_email` and `load_posts` concurrently and put the three results into a `Profile`. The call must take about 40ms, not 120ms.

### Implement `load_limits`

```rust
pub async fn load_limits(
    id: u32,
) -> Result<(u32, u32), String>
```

Run `load_quota` and `load_rate_limit` concurrently. Return `Ok((quota, rate_limit))` if both succeed, or the error if either fails.

```rust
assert_eq!(load_limits(3).await, Ok((300, 60)));
assert_eq!(
    load_limits(0).await,
    Err("no quota for user 0".to_string())
);
```

### Notes

- Do not `spawn` anything. Both functions are a single macro call plus a little plumbing.
- Build the future values inside the macro, not before it. `join!` needs to own them.
- The tuple comes back in the order you listed the futures, not the order they finished.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `let (name, email, posts) = tokio::join!(...)` destructures straight into the three bindings.
- `Profile { name, email, posts }` then uses field init shorthand.
- `tokio::try_join!(load_quota(id), load_rate_limit(id))` already has the exact return type `load_limits` needs, so it can be the last expression in the body.
- If the timing test fails, look for a stray `.await` before the macro.

</details>
