use condvar_signaling::*;
use std::thread;
use std::time::{Duration, Instant};

mod basic_tests {
    use super::*;

    #[test]
    fn a_new_queue_is_empty() {
        let queue: Queue<u32> = Queue::new();

        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn push_then_pop_is_fifo() {
        let queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
        assert!(queue.is_empty());
    }

    #[test]
    fn works_with_non_copy_items() {
        let queue = Queue::new();
        queue.push(String::from("alpha"));
        queue.push(String::from("beta"));

        assert_eq!(queue.pop(), "alpha");
        assert_eq!(queue.pop(), "beta");
    }
}

mod pop_timeout_tests {
    use super::*;

    #[test]
    fn returns_none_on_an_empty_queue() {
        let queue: Queue<u32> = Queue::new();

        assert_eq!(queue.pop_timeout(Duration::from_millis(10)), None);
    }

    #[test]
    fn actually_waits_before_giving_up() {
        let queue: Queue<u32> = Queue::new();
        let started = Instant::now();

        assert_eq!(queue.pop_timeout(Duration::from_millis(40)), None);

        assert!(
            started.elapsed() >= Duration::from_millis(30),
            "pop_timeout returned after {:?}, it must wait for the timeout",
            started.elapsed()
        );
    }

    #[test]
    fn returns_an_item_that_is_already_there() {
        let queue = Queue::new();
        queue.push(9);

        assert_eq!(queue.pop_timeout(Duration::from_millis(10)), Some(9));
    }

    #[test]
    fn wakes_up_when_a_producer_pushes() {
        let queue: Queue<u32> = Queue::new();

        thread::scope(|scope| {
            scope.spawn(|| {
                thread::sleep(Duration::from_millis(20));
                queue.push(42);
            });

            assert_eq!(queue.pop_timeout(Duration::from_secs(2)), Some(42));
        });
    }

    #[test]
    fn does_not_lose_a_notification_sent_before_the_wait() {
        // A `notify` with nobody waiting is dropped on the floor. Only a
        // predicate loop that checks the queue *before* waiting survives
        // this ordering.
        let queue: Queue<u32> = Queue::new();
        queue.push(5);
        thread::sleep(Duration::from_millis(10));

        assert_eq!(queue.pop_timeout(Duration::from_millis(50)), Some(5));
    }
}

mod blocking_tests {
    use super::*;

    #[test]
    fn pop_blocks_until_an_item_arrives() {
        let queue: Queue<u32> = Queue::new();

        thread::scope(|scope| {
            scope.spawn(|| {
                thread::sleep(Duration::from_millis(20));
                queue.push(7);
            });

            assert_eq!(queue.pop(), 7);
        });
    }

    #[test]
    fn four_consumers_split_the_work_with_nothing_lost() {
        let queue: Queue<u64> = Queue::new();

        let totals = thread::scope(|scope| {
            let consumers: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(|| {
                        let mut sum = 0;
                        for _ in 0..25 {
                            sum += queue.pop();
                        }
                        sum
                    })
                })
                .collect();

            for value in 1..=100u64 {
                queue.push(value);
            }

            consumers
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect::<Vec<_>>()
        });

        assert_eq!(totals.iter().sum::<u64>(), 5050);
        assert!(queue.is_empty());
    }

    #[test]
    fn many_producers_and_one_consumer() {
        let queue: Queue<u64> = Queue::new();

        let queue = &queue;

        let total = thread::scope(|scope| {
            for producer in 0..5u64 {
                scope.spawn(move || {
                    for step in 0..10u64 {
                        queue.push(producer * 10 + step);
                    }
                });
            }

            scope
                .spawn(move || (0..50).map(|_| queue.pop()).sum::<u64>())
                .join()
                .unwrap()
        });

        assert_eq!(total, (0..50u64).sum::<u64>());
    }
}
