use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// A unit of work handed to the pool.
///
/// `FnOnce` because a job runs exactly once, `Send` because it crosses a
/// thread boundary, `'static` because the pool cannot prove the job
/// finishes before any borrow it holds expires.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A fixed size pool of worker threads sharing one job queue.
///
/// The threads are created once, in `new`, and reused for every job.
/// That is the entire reason a pool exists: spawning an OS thread is not
/// free, so a program with many short tasks should pay for the threads
/// once rather than once per task.
///
/// Both fields are `Option` so that `drop` can take them out. Dropping
/// the sender is how the workers are told to stop, and it has to happen
/// before the joins or the pool would wait for threads that are still
/// blocked on `recv`.
pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Option<JoinHandle<()>>>,
}

impl ThreadPool {
    /// Creates a pool with `size` worker threads.
    ///
    /// The receiver is shared as `Arc<Mutex<Receiver<Job>>>`. A
    /// `Receiver` cannot be cloned, so the workers take turns holding the
    /// lock long enough to pull one job out. Notice the guard is dropped
    /// before the job runs: holding it across the call would serialise
    /// the whole pool down to one active worker.
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0. A pool with no workers accepts jobs and
    /// never runs them, which is a silent hang rather than an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use building_a_thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(2);
    /// pool.execute(|| println!("running"));
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a thread pool needs at least one worker");

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_| {
                let receiver: Arc<Mutex<Receiver<Job>>> = Arc::clone(&receiver);

                Some(thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv();

                    match job {
                        // The lock is already released here, so the other
                        // workers can pick up their own jobs while this
                        // one runs.
                        Ok(job) => job(),
                        // Every sender is gone: no more work can arrive.
                        Err(_) => break,
                    }
                }))
            })
            .collect();

        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }

    /// Queues a job for the next free worker.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use std::sync::Arc;
    /// use building_a_thread_pool::ThreadPool;
    ///
    /// let counter = Arc::new(AtomicUsize::new(0));
    ///
    /// {
    ///     let pool = ThreadPool::new(4);
    ///
    ///     for _ in 0..20 {
    ///         let counter = Arc::clone(&counter);
    ///         pool.execute(move || {
    ///             counter.fetch_add(1, Ordering::Relaxed);
    ///         });
    ///     }
    /// } // dropping the pool waits for every job to finish
    ///
    /// assert_eq!(counter.load(Ordering::Relaxed), 20);
    /// ```
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = self.sender.as_ref() {
            // Only fails once the workers are gone, which cannot happen
            // while `&self` is alive.
            let _ = sender.send(Box::new(job));
        }
    }

    /// The number of worker threads in the pool.
    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    /// Shuts the pool down: closes the queue, then waits for every worker.
    ///
    /// Order matters. Dropping the sender disconnects the channel, so
    /// each worker's `recv` returns `Err` once the queue is empty and the
    /// loop ends. Only then can the joins finish. Joining first would
    /// block forever on a worker still waiting for work.
    ///
    /// Because the joins happen here, jobs already queued are guaranteed
    /// to have run by the time the pool is dropped.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(handle) = worker.take() {
                let _ = handle.join();
            }
        }
    }
}

// Example usage
pub fn main() {
    let pool = ThreadPool::new(3);

    for id in 0..6 {
        pool.execute(move || println!("job {id}"));
    }
}
