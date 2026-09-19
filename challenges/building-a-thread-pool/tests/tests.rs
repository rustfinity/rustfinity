use building_a_thread_pool::ThreadPool;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[test]
fn a_pool_reports_its_size() {
    assert_eq!(ThreadPool::new(1).size(), 1);
    assert_eq!(ThreadPool::new(7).size(), 7);
}

#[test]
#[should_panic]
fn a_pool_with_no_workers_is_rejected() {
    ThreadPool::new(0);
}

#[test]
fn a_single_job_runs() {
    let flag = Arc::new(AtomicUsize::new(0));

    {
        let pool = ThreadPool::new(2);
        let flag = Arc::clone(&flag);
        pool.execute(move || {
            flag.store(1, Ordering::SeqCst);
        });
    }

    assert_eq!(flag.load(Ordering::SeqCst), 1);
}

#[test]
fn an_unused_pool_shuts_down_cleanly() {
    let pool = ThreadPool::new(4);
    drop(pool);
}

#[test]
fn dropping_the_pool_waits_for_every_queued_job() {
    let done = Arc::new(AtomicUsize::new(0));

    {
        let pool = ThreadPool::new(4);

        for _ in 0..200 {
            let done = Arc::clone(&done);
            pool.execute(move || {
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
    } // drop joins the workers

    assert_eq!(
        done.load(Ordering::SeqCst),
        200,
        "drop must finish the queue before returning"
    );
}

#[test]
fn jobs_can_move_owned_values_in() {
    let results = Arc::new(Mutex::new(Vec::new()));

    {
        let pool = ThreadPool::new(3);

        for id in 0..10u64 {
            let results = Arc::clone(&results);
            let label = format!("job-{id}");

            pool.execute(move || {
                results.lock().unwrap().push((id, label));
            });
        }
    }

    let mut collected = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    collected.sort();

    let expected: Vec<(u64, String)> = (0..10u64).map(|id| (id, format!("job-{id}"))).collect();

    assert_eq!(collected, expected);
}

#[test]
fn work_is_spread_over_at_most_size_threads() {
    let seen: Arc<Mutex<HashSet<std::thread::ThreadId>>> = Arc::new(Mutex::new(HashSet::new()));

    {
        let pool = ThreadPool::new(3);

        for _ in 0..60 {
            let seen = Arc::clone(&seen);
            pool.execute(move || {
                seen.lock().unwrap().insert(std::thread::current().id());
            });
        }
    }

    let count = seen.lock().unwrap().len();

    assert!(
        count <= 3,
        "a pool of 3 must not run jobs on {count} different threads"
    );
    assert!(count >= 1, "the jobs never ran");
}

#[test]
fn every_worker_can_run_at_the_same_time() {
    // Four jobs on a pool of four. Each one waits for the other three to
    // arrive, so this only completes if the workers really do run
    // concurrently. The timeout keeps a broken pool from hanging.
    let size = 4;
    let gate = Arc::new((Mutex::new(0usize), Condvar::new()));
    let all_arrived = Arc::new(AtomicUsize::new(0));

    {
        let pool = ThreadPool::new(size);

        for _ in 0..size {
            let gate = Arc::clone(&gate);
            let all_arrived = Arc::clone(&all_arrived);

            pool.execute(move || {
                let (lock, condvar) = &*gate;
                let mut arrived = lock.lock().unwrap();

                *arrived += 1;
                condvar.notify_all();

                let deadline = Duration::from_secs(2);
                while *arrived < size {
                    let (guard, timeout) = condvar.wait_timeout(arrived, deadline).unwrap();
                    arrived = guard;

                    if timeout.timed_out() {
                        return;
                    }
                }

                all_arrived.fetch_add(1, Ordering::SeqCst);
            });
        }
    }

    assert_eq!(
        all_arrived.load(Ordering::SeqCst),
        size,
        "all {size} workers should have been running together"
    );
}

#[test]
fn a_pool_of_one_still_runs_everything() {
    let total = Arc::new(AtomicUsize::new(0));

    {
        let pool = ThreadPool::new(1);

        for value in 1..=50 {
            let total = Arc::clone(&total);
            pool.execute(move || {
                total.fetch_add(value, Ordering::SeqCst);
            });
        }
    }

    assert_eq!(total.load(Ordering::SeqCst), (1..=50).sum::<usize>());
}

#[test]
fn a_pool_can_be_reused_for_several_batches() {
    let total = Arc::new(AtomicUsize::new(0));
    let pool = ThreadPool::new(3);

    for _ in 0..5 {
        for _ in 0..20 {
            let total = Arc::clone(&total);
            pool.execute(move || {
                total.fetch_add(1, Ordering::SeqCst);
            });
        }
    }

    drop(pool);

    assert_eq!(total.load(Ordering::SeqCst), 100);
}
