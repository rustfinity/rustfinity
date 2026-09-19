Spawning a single thread is easy. Spawning several and getting their answers back in a predictable order is where people usually slip.

The pattern is two passes. First spawn every thread and collect the handles into a `Vec<JoinHandle<T>>`. Only then join them, one after another, in that same order. Because the vector remembers which handle came from which input, the results line up with the input regardless of which thread actually finished first.

The order of the two passes matters a lot. If you join inside the loop that spawns, each thread is finished before the next one starts and you have written a sequential loop with extra steps. Spawn first, join second, and the work really does overlap.

## Your Task

Implement two functions.

### Implement `sum_chunks`

```rust
pub fn sum_chunks(chunks: Vec<Vec<u64>>) -> Vec<u64>
```

Give each chunk its own thread. Return one sum per chunk, in the same order the chunks arrived. An empty chunk sums to 0.

```rust
let chunks = vec![
    vec![1, 2],
    vec![10],
    vec![],
];
assert_eq!(sum_chunks(chunks), vec![3, 10, 0]);
```

### Implement `sum_all`

```rust
pub fn sum_all(chunks: Vec<Vec<u64>>) -> u64
```

Sum the chunks in parallel, then add the partial sums into one grand total.

```rust
let chunks = vec![vec![1, 2], vec![3]];
assert_eq!(sum_all(chunks), 6);
```

### Notes

- The output length always equals the input length. No chunk is skipped.
- The tests deliberately feed you chunks of very different sizes, so a solution that relies on threads finishing in spawn order will fail.
- Never assert on which thread finished first. Order comes from the handle vector.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Build the handles with `.map(...).collect::<Vec<_>>()`, which forces every thread to start before you touch a single result.
- Then `handles.into_iter().map(|h| h.join().unwrap()).collect()`.
- `sum_all` can just call `sum_chunks` and sum the answer. Reusing it is better than writing the spawn loop twice.
- `chunk.iter().sum::<u64>()` needs the turbofish because the closure's return type is otherwise ambiguous.

</details>
