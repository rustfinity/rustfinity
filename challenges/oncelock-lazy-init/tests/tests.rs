use oncelock_lazy_init::*;
use std::thread;

// These tests share one process-wide `OnceLock`, so they are written to
// hold no matter which one runs first.

mod settings_tests {
    use super::*;

    #[test]
    fn has_the_expected_values() {
        assert_eq!(settings().name, "rustfinity");
        assert_eq!(settings().retries, 3);
    }

    #[test]
    fn repeated_calls_return_the_same_value() {
        assert_eq!(settings(), settings());
    }

    #[test]
    fn every_caller_gets_the_same_allocation() {
        let first: *const Settings = settings();
        let second: *const Settings = settings();

        assert!(
            std::ptr::eq(first, second),
            "settings() must hand out the one stored value, not a fresh copy"
        );
    }

    #[test]
    fn the_initializer_runs_at_most_once() {
        for _ in 0..100 {
            settings();
        }

        assert_eq!(build_count(), 1);
    }

    #[test]
    fn racing_threads_still_build_it_once() {
        let pointers = thread::scope(|s| {
            let handles: Vec<_> = (0..8)
                .map(|_| s.spawn(|| settings() as *const Settings as usize))
                .collect();

            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        });

        assert_eq!(build_count(), 1);
        assert!(
            pointers.windows(2).all(|pair| pair[0] == pair[1]),
            "all threads must observe the same value"
        );
    }
}

mod triangular_tests {
    use super::*;

    #[test]
    fn has_32_entries() {
        assert_eq!(TRIANGULAR.len(), 32);
    }

    #[test]
    fn starts_with_the_known_values() {
        assert_eq!(TRIANGULAR[..6], [0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn last_entry_is_correct() {
        assert_eq!(TRIANGULAR[31], 31 * 32 / 2);
    }

    #[test]
    fn each_entry_is_the_running_sum() {
        for n in 1..32usize {
            assert_eq!(TRIANGULAR[n], TRIANGULAR[n - 1] + n as u64, "at {n}");
        }
    }

    #[test]
    fn readable_from_many_threads_at_once() {
        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    assert_eq!(TRIANGULAR[10], 55);
                });
            }
        });
    }
}
