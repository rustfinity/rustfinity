use std::thread;
use thread_local_state::*;

// Every exact assertion runs inside a thread this test spawned, so it
// never depends on what the harness thread did earlier.
fn in_fresh_thread<T, F>(body: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    thread::spawn(body).join().unwrap()
}

mod bump_tests {
    use super::*;

    #[test]
    fn a_fresh_thread_starts_at_zero() {
        assert_eq!(in_fresh_thread(bump), 1);
    }

    #[test]
    fn it_returns_the_value_after_incrementing() {
        let counts = in_fresh_thread(|| vec![bump(), bump(), bump()]);

        assert_eq!(counts, vec![1, 2, 3]);
    }

    #[test]
    fn each_call_moves_the_count_by_exactly_one() {
        let first = bump();
        let second = bump();

        assert_eq!(second, first + 1);
    }

    #[test]
    fn one_thread_never_sees_another_thread_count() {
        let a = in_fresh_thread(|| {
            for _ in 0..49 {
                bump();
            }
            bump()
        });
        let b = in_fresh_thread(bump);

        assert_eq!(a, 50);
        assert_eq!(b, 1, "a new thread must not inherit an old thread's count");
    }
}

mod log_tests {
    use super::*;

    #[test]
    fn a_fresh_thread_has_an_empty_log() {
        assert_eq!(in_fresh_thread(events), Vec::<String>::new());
    }

    #[test]
    fn events_come_back_in_the_order_they_were_recorded() {
        let log = in_fresh_thread(|| {
            record("open");
            record("read");
            record("close");
            events()
        });

        assert_eq!(log, vec!["open", "read", "close"]);
    }

    #[test]
    fn duplicates_are_kept() {
        let log = in_fresh_thread(|| {
            record("tick");
            record("tick");
            events()
        });

        assert_eq!(log, vec!["tick", "tick"]);
    }

    #[test]
    fn reading_the_log_does_not_empty_it() {
        let (first, second) = in_fresh_thread(|| {
            record("once");
            (events(), events())
        });

        assert_eq!(first, vec!["once"]);
        assert_eq!(second, vec!["once"]);
    }

    #[test]
    fn logs_do_not_leak_between_threads() {
        in_fresh_thread(|| record("private"));

        assert_eq!(in_fresh_thread(events), Vec::<String>::new());
    }

    #[test]
    fn the_counter_and_the_log_are_independent() {
        let (count, log) = in_fresh_thread(|| {
            record("a");
            let count = bump();
            record("b");
            (count, events())
        });

        assert_eq!(count, 1);
        assert_eq!(log, vec!["a", "b"]);
    }
}

mod counts_per_thread_tests {
    use super::*;

    #[test]
    fn no_workers_gives_no_counts() {
        assert_eq!(counts_per_thread(&[]), Vec::<u64>::new());
    }

    #[test]
    fn each_worker_reports_its_own_total() {
        assert_eq!(counts_per_thread(&[3, 1, 2]), vec![3, 1, 2]);
    }

    #[test]
    fn a_worker_with_no_bumps_reports_zero() {
        assert_eq!(counts_per_thread(&[0, 2, 0]), vec![0, 2, 0]);
    }

    #[test]
    fn results_stay_in_input_order() {
        // Descending work: a solution that collects by completion order
        // rather than by index gets this wrong.
        let bumps: Vec<u64> = (0..8).map(|worker| (8 - worker) * 40).collect();
        let expected = bumps.clone();

        assert_eq!(counts_per_thread(&bumps), expected);
    }

    #[test]
    fn nothing_is_shared_between_the_workers() {
        let counts = counts_per_thread(&[100; 12]);

        assert_eq!(
            counts,
            vec![100; 12],
            "shared state would make these totals overlap"
        );
    }

    #[test]
    fn the_calling_thread_is_unaffected_by_its_workers() {
        let before = bump();
        counts_per_thread(&[500, 500]);
        let after = bump();

        assert_eq!(after, before + 1);
    }
}
