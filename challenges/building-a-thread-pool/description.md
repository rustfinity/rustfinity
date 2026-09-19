Spawning a thread per task works until there are a lot of tasks. Each `thread::spawn` asks the OS for a real thread with its own stack, and for work measured in microseconds the spawn costs more than the job. A **thread pool** pays that cost once: a fixed number of workers start up, then take jobs off a shared queue for as long as the pool lives.

A job is a boxed closure:

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

`FnOnce` because it runs once, `Send` because it crosses into another thread, `'static` because the pool cannot prove the job finishes before any borrow it captured expires.

The queue is an `mpsc` channel. Many threads send, but a `Receiver` cannot be cloned, so the workers share one behind a lock:

```rust
let receiver = Arc::new(Mutex::new(receiver));
```

Each worker loops: take the lock, pull one job, release the lock, run the job.

```rust
loop {
    let job = receiver.lock().unwrap().recv();

    match job {
        Ok(job) => job(),
        Err(_) => break,
    }
}
```

The line to stare at is `let job = ...recv();`. The temporary guard is dropped at the end of that statement, before `job()` runs, so the other workers can pull their own jobs while this one is busy. Write it as `while let Ok(job) = receiver.lock().unwrap().recv()` and the guard lives for the whole body: your pool has as many threads as you asked for and exactly one of them ever works.

Shutdown is the other half. `recv` returns `Err` when every `Sender` is gone, which is the signal for a worker to break out of its loop. So `Drop` has to drop the pool's sender *before* joining:

```rust
fn drop(&mut self) {
    drop(self.sender.take());

    for worker in &mut self.workers {
        if let Some(handle) = worker.take() {
            handle.join().unwrap();
        }
    }
}
```

Both fields are `Option` because `drop` only gets `&mut self` and cannot move out of it. `take()` swaps in `None` and hands you the value. Join first and drop second and you wait forever on workers that are still blocked on `recv`.

## Your Task

Build `ThreadPool`.

### Implement `new`

```rust
pub fn new(size: usize) -> ThreadPool
```

Start `size` worker threads sharing one job queue. Panic if `size` is `0`: a pool with no workers would accept jobs and silently never run them.

### Implement `execute`

```rust
pub fn execute<F>(&self, job: F)
where
    F: FnOnce() + Send + 'static,
```

Box the job and put it on the queue. Note the `&self`: submitting work needs no exclusive access.

### Implement `size`

```rust
pub fn size(&self) -> usize
```

How many workers the pool has.

### Implement `Drop` for the pool

Close the queue, then join every worker. When the pool is dropped, every job already queued must have finished.

```rust
let counter = Arc::new(AtomicUsize::new(0));

{
    let pool = ThreadPool::new(4);

    for _ in 0..20 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            counter.fetch_add(1, Ordering::Relaxed);
        });
    }
}

assert_eq!(counter.load(Ordering::Relaxed), 20);
```

### Notes

- Jobs run on `size` threads at most, and all `size` of them must be able to run at the same time.
- A pool of one worker still has to run everything, just one at a time.
- The pool can be reused for as many batches as you like.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `struct ThreadPool { sender: Option<Sender<Job>>, workers: Vec<Option<JoinHandle<()>>> }`.
- `Arc::clone(&receiver)` once per worker, before the `move` closure.
- Bind the received job to a `let` on its own line so the mutex guard is released before the job runs.
- `assert!(size > 0, "...")` at the top of `new`.
- In `Drop`, `self.sender.take()` returns the `Option`; dropping it disconnects the channel.
- `for worker in &mut self.workers` plus `worker.take()` gives you an owned `JoinHandle` to join.

</details>
