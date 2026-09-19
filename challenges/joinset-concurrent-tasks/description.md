`tokio::join!` needs to know its futures at compile time. `Vec<JoinHandle<T>>` handles a dynamic count, but awaiting it in order means a slow first task hides five fast ones that already finished.

`JoinSet` is the collection for the general case: spawn as many tasks as you like, whenever you like, and pull results out in the order they complete.

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
set.spawn(work(1));
set.spawn(work(2));

while let Some(res) = set.join_next().await {
    println!("{}", res.unwrap());
}
```

`join_next` returns `None` when the set is empty, so the `while let` ends on its own. Each item is a `Result`, because a task can panic or be aborted.

Two more things it gives you:

- `abort_all()` cancels every task still running.
- Dropping the set aborts them too, so a `?` on an early return does not leak tasks.

## Bounded concurrency

Because you can spawn into a set that is already running, `JoinSet` is how you cap how much runs at once: prime it with `limit` tasks, then start one more each time `join_next` hands you a result.

## Your Task

`work` is written for you. It sleeps for `id` milliseconds and returns `id * 10`, so a smaller id always finishes first.

Implement three functions:

- `async results_in_completion_order(ids: Vec<u64>) -> Vec<u64>`, everything at once, results in finishing order
- `async first_n_to_finish(ids: Vec<u64>, n: usize) -> Vec<u64>`, the first `n` results, with the rest abandoned rather than awaited
- `async with_limit(ids: Vec<u64>, limit: usize) -> Vec<u64>`, at most `limit` tasks in flight, results in finishing order

So `results_in_completion_order(vec![30, 10, 20])` is `[100, 200, 300]`, and `with_limit(vec![50, 10, 25, 5], 2)` is `[100, 250, 50, 500]`.

### Notes

- `first_n_to_finish` must return as soon as it has `n` results. Waiting for the slow ones first and truncating afterwards is not the same thing.
- Handle `n` larger than the number of ids, and `n` of zero.
- Assume `limit` is at least one.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `set.spawn(work(id))` takes the future itself, not a closure.
- `while let Some(result) = set.join_next().await` is the whole drain loop.
- For `first_n_to_finish`, loop while `results.len() < n` and break when `join_next` gives `None`.
- For `with_limit`, keep the remaining ids in an iterator: `ids.into_iter()`. Prime the set with `by_ref().take(limit)`, then call `next()` on it after each completed result.

</details>
