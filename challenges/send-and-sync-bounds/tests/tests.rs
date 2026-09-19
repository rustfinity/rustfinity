use send_and_sync_bounds::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

mod parallel_map_tests {
    use super::*;

    #[test]
    fn empty_input_gives_empty_output() {
        assert_eq!(
            parallel_map(Vec::<u32>::new(), |n| n + 1),
            Vec::<u32>::new()
        );
    }

    #[test]
    fn maps_every_item() {
        assert_eq!(parallel_map(vec![1, 2, 3, 4], |n| n * 2), vec![2, 4, 6, 8]);
    }

    #[test]
    fn output_order_matches_input_order() {
        // Descending work per item, so a result order that depends on
        // completion time comes out backwards.
        let items: Vec<u64> = (0..12).collect();

        let result = parallel_map(items, |n| {
            let mut acc = 0u64;
            for step in 0..(12 - n) * 2_000 {
                acc = acc.wrapping_add(step);
            }
            let _ = acc;
            n
        });

        assert_eq!(result, (0..12u64).collect::<Vec<_>>());
    }

    #[test]
    fn accepts_items_that_are_not_copy() {
        let items = vec![String::from("a"), String::from("bb"), String::from("ccc")];

        assert_eq!(parallel_map(items, |s| s.len()), vec![1, 2, 3]);
    }

    #[test]
    fn produces_results_that_are_not_copy() {
        let result = parallel_map(vec![1u32, 2], |n| n.to_string());

        assert_eq!(result, vec!["1", "2"]);
    }

    #[test]
    fn the_same_closure_is_shared_by_every_thread() {
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);

        let result = parallel_map(vec![1, 2, 3, 4, 5], move |n: u32| {
            counter.fetch_add(1, Ordering::Relaxed);
            n
        });

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        assert_eq!(calls.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn a_closure_holding_shared_state_still_fits_the_bounds() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let recorder = Arc::clone(&seen);

        parallel_map(vec![10u32, 20, 30], move |n| {
            recorder.lock().unwrap().push(n);
        });

        let mut recorded = seen.lock().unwrap().clone();
        recorded.sort_unstable();

        assert_eq!(recorded, vec![10, 20, 30]);
    }
}

mod run_jobs_tests {
    use super::*;

    #[test]
    fn no_jobs_gives_no_results() {
        let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = vec![];

        assert_eq!(run_jobs(jobs), Vec::<u32>::new());
    }

    #[test]
    fn results_come_back_in_job_order() {
        let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = vec![
            Box::new(|| 1),
            Box::new(|| 2),
            Box::new(|| 3),
            Box::new(|| 4),
        ];

        assert_eq!(run_jobs(jobs), vec![1, 2, 3, 4]);
    }

    #[test]
    fn a_job_may_consume_what_it_captured() {
        // `FnOnce`, not `Fn`: this closure moves its captured String out.
        let owned = String::from("rustfinity");
        let jobs: Vec<Box<dyn FnOnce() -> String + Send>> =
            vec![Box::new(move || owned), Box::new(|| String::from("!"))];

        assert_eq!(run_jobs(jobs), vec!["rustfinity", "!"]);
    }

    #[test]
    fn plain_function_pointers_work_too() {
        fn double(n: u32) -> u32 {
            n * 2
        }

        let jobs: Vec<Box<dyn FnOnce() -> u32 + Send>> = (1..=5u32)
            .map(|n| Box::new(move || double(n)) as _)
            .collect();

        assert_eq!(run_jobs(jobs), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn every_job_really_runs_on_its_own_thread() {
        let ids = Arc::new(Mutex::new(Vec::new()));

        let jobs: Vec<Box<dyn FnOnce() + Send>> = (0..6)
            .map(|_| {
                let ids = Arc::clone(&ids);
                Box::new(move || {
                    ids.lock().unwrap().push(std::thread::current().id());
                }) as _
            })
            .collect();

        run_jobs(jobs);

        let ids = ids.lock().unwrap();
        let unique: std::collections::HashSet<_> = ids.iter().collect();

        assert_eq!(ids.len(), 6);
        assert_eq!(unique.len(), 6, "each job must get its own thread");
    }
}
