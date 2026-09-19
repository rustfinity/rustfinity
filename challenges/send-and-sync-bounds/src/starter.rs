use std::sync::Arc;
use std::thread;

/// Applies `f` to every item on its own thread, preserving order.
pub fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
// TODO: add the bounds that let items, results and `f` cross a thread
// boundary. `f` is shared by every thread through an Arc.
{
    unimplemented!()
}

/// Runs every job on its own thread and returns the results in order.
pub fn run_jobs<T, F>(jobs: Vec<F>) -> Vec<T>
// TODO: a job is called once and consumed, and it is moved rather than
// shared. Pick the closure trait and the marker bounds that match.
{
    unimplemented!()
}

// Example usage
pub fn main() {
    let doubled: Vec<i32> = parallel_map(vec![1, 2, 3], |n: i32| n * 2);
    println!("{doubled:?}");

    let jobs: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![Box::new(|| 1 + 1), Box::new(|| 2 + 2)];
    let results: Vec<i32> = run_jobs(jobs);
    println!("{results:?}");
}
