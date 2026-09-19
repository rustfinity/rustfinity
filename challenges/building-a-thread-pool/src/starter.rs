use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// A unit of work handed to the pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A fixed size pool of worker threads sharing one job queue.
pub struct ThreadPool {
    // TODO: the sending half of the job queue, and the worker handles.
    // Both need to be taken out again in `drop`.
}

impl ThreadPool {
    /// Creates a pool with `size` worker threads.
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    pub fn new(size: usize) -> ThreadPool {
        // TODO: one channel, an Arc<Mutex<Receiver<Job>>> shared by every
        // worker, and a loop in each worker that pulls jobs until the
        // channel disconnects
        unimplemented!()
    }

    /// Queues a job for the next free worker.
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO
        unimplemented!()
    }

    /// The number of worker threads in the pool.
    pub fn size(&self) -> usize {
        // TODO
        unimplemented!()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: close the queue first, then join every worker
    }
}

// Example usage
pub fn main() {
    let pool = ThreadPool::new(3);

    for id in 0..6 {
        pool.execute(move || println!("job {id}"));
    }
}
