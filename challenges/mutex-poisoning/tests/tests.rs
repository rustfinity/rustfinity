use mutex_poisoning::*;
use std::thread;

/// Panics inside a thread while it holds `counter`'s lock, which poisons it.
/// The panicking thread is joined here, so nothing leaks out into the test.
fn poison(counter: &ResilientCounter) {
    let result = thread::scope(|s| {
        s.spawn(|| {
            counter.with(|value| {
                *value += 1;
                panic!("worker exploded");
            })
        })
        .join()
    });

    assert!(result.is_err(), "the worker was supposed to panic");
}

mod basics {
    use super::*;

    #[test]
    fn starts_at_the_given_value() {
        assert_eq!(ResilientCounter::new(5).get(), 5);
    }

    #[test]
    fn with_can_mutate_and_return() {
        let counter = ResilientCounter::new(0);
        counter.with(|value| *value += 3);
        let doubled = counter.with(|value| {
            *value -= 1;
            *value * 2
        });

        assert_eq!(doubled, 4);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn a_fresh_counter_is_not_poisoned() {
        assert!(!ResilientCounter::new(0).is_poisoned());
    }

    #[test]
    fn negative_start_is_fine() {
        assert_eq!(ResilientCounter::new(-40).get(), -40);
    }
}

mod poisoning {
    use super::*;

    #[test]
    fn a_panicking_holder_poisons_the_mutex() {
        let counter = ResilientCounter::new(0);
        poison(&counter);
        assert!(counter.is_poisoned());
    }

    #[test]
    fn get_still_works_after_poisoning() {
        let counter = ResilientCounter::new(10);
        poison(&counter);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn with_still_works_after_poisoning() {
        let counter = ResilientCounter::new(0);
        poison(&counter);
        counter.with(|value| *value += 41);
        assert_eq!(counter.get(), 42);
    }

    #[test]
    fn the_write_before_the_panic_is_kept() {
        let counter = ResilientCounter::new(100);
        poison(&counter);
        assert_eq!(counter.get(), 101, "the +1 happened before the panic");
    }

    #[test]
    fn poisoning_twice_is_harmless() {
        let counter = ResilientCounter::new(0);
        poison(&counter);
        poison(&counter);
        assert_eq!(counter.get(), 2);
        assert!(counter.is_poisoned());
    }
}

mod concurrency {
    use super::*;

    #[test]
    fn many_threads_add_without_losing_updates() {
        let counter = ResilientCounter::new(0);

        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..250 {
                        counter.with(|value| *value += 1);
                    }
                });
            }
        });

        assert_eq!(counter.get(), 2000);
    }

    #[test]
    fn healthy_threads_keep_going_after_one_panics() {
        let counter = ResilientCounter::new(0);
        poison(&counter);

        thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..100 {
                        counter.with(|value| *value += 1);
                    }
                });
            }
        });

        assert_eq!(counter.get(), 401);
        assert!(counter.is_poisoned());
    }
}
