Some parallel algorithms run in rounds. Every worker does its part of round 1, and only when *all* of them are finished may anyone start round 2. Cellular automata, iterative solvers, and simulation ticks all have this shape.

`std::sync::Barrier` is the tool. You build it with a number of parties, and every thread that calls `wait()` blocks until that many threads have arrived. Then all of them are released together.

```rust
let barrier = Barrier::new(4);

barrier.wait(); // blocks until 4 threads are here
```

The subtle part is that one barrier per round is usually not enough. Consider workers that each read their neighbour's cell and then overwrite their own:

```rust
let mine = cells[i].load(Relaxed);
let right = cells[i + 1].load(Relaxed);
cells[i].store(mine + right, Relaxed);
barrier.wait();
```

This is broken. Worker 0 can finish its store before worker 1 has done its loads, so worker 1 reads a round-2 value while computing round 2. The fix is to separate the phases with a barrier of their own:

```rust
// read phase
barrier.wait();
// write phase
barrier.wait();
```

Two waits per round: one that says "everyone has read", one that says "everyone has written".

`wait()` also returns a `BarrierWaitResult`. Exactly one of the parties gets `is_leader() == true`, which is a cheap way to run a single piece of per-round bookkeeping without another lock.

## Your Task

Implement two functions.

### Implement `run_rounds`

```rust
pub fn run_rounds(
    cells: Vec<i64>,
    rounds: usize,
) -> Vec<i64>
```

Give every cell its own thread. In each round, a cell becomes itself plus its right neighbour, wrapping around at the end:

```
[1, 2, 3] -> [3, 5, 4] -> [8, 9, 7]
```

Every cell must read the *old* values of the round, so the read phase and the write phase have to be separated by a barrier. Return the cells after `rounds` rounds. An empty input, or zero rounds, returns the input unchanged.

### Implement `count_leaders`

```rust
pub fn count_leaders(parties: usize, rounds: usize) -> usize
```

Run `parties` threads through `rounds` barriers and count how many times a thread was told it was the leader. With at least one party the answer is always `rounds`; with zero parties it is 0.

### Notes

- `Barrier::new(n)` releases threads in groups of `n`, and can be reused round after round.
- A barrier with fewer arrivals than parties blocks forever, so the party count and the thread count must match exactly.
- `thread::scope` lets the threads borrow the shared cells without an `Arc`.
- `AtomicI64` gives you shared mutable cells that several threads may touch without a lock.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Build the cells as `Vec<AtomicI64>` and the barrier as `Barrier::new(cells.len())`, both outside the scope.
- The worker body is: load mine, load `(index + 1) % len`, `barrier.wait()`, store the sum, `barrier.wait()`.
- Return early for an empty input. `Barrier::new(0)` is not something you want to reason about.
- `AtomicI64::into_inner` turns the cells back into plain values once the threads are done.
- For `count_leaders`, `if barrier.wait().is_leader() { counter.fetch_add(1, Ordering::Relaxed); }`.

</details>
