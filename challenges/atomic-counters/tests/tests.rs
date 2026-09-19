use atomic_counters::*;
use std::thread;

mod basics {
    use super::*;

    #[test]
    fn starts_at_zero() {
        assert_eq!(CacheStats::new().snapshot(), (0, 0));
    }

    #[test]
    fn records_hits_and_misses_separately() {
        let stats = CacheStats::new();
        stats.record(true);
        stats.record(true);
        stats.record(false);

        assert_eq!(stats.snapshot(), (2, 1));
    }

    #[test]
    fn snapshot_does_not_reset() {
        let stats = CacheStats::new();
        stats.record(true);

        assert_eq!(stats.snapshot(), (1, 0));
        assert_eq!(stats.snapshot(), (1, 0));
    }

    #[test]
    fn only_misses() {
        let stats = CacheStats::new();
        for _ in 0..5 {
            stats.record(false);
        }

        assert_eq!(stats.snapshot(), (0, 5));
    }
}

mod drain_tests {
    use super::*;

    #[test]
    fn returns_the_values_it_clears() {
        let stats = CacheStats::new();
        stats.record(true);
        stats.record(true);
        stats.record(false);

        assert_eq!(stats.drain(), (2, 1));
        assert_eq!(stats.snapshot(), (0, 0));
    }

    #[test]
    fn draining_twice_gives_zeroes_the_second_time() {
        let stats = CacheStats::new();
        stats.record(true);

        assert_eq!(stats.drain(), (1, 0));
        assert_eq!(stats.drain(), (0, 0));
    }

    #[test]
    fn counting_resumes_after_a_drain() {
        let stats = CacheStats::new();
        stats.record(true);
        stats.drain();
        stats.record(false);

        assert_eq!(stats.snapshot(), (0, 1));
    }

    #[test]
    fn draining_an_untouched_counter_is_zero() {
        assert_eq!(CacheStats::new().drain(), (0, 0));
    }
}

mod concurrency {
    use super::*;

    #[test]
    fn no_increment_is_lost_under_contention() {
        let stats = CacheStats::new();

        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for i in 0..500 {
                        stats.record(i % 4 != 0);
                    }
                });
            }
        });

        // 500 lookups per thread, one in four a miss.
        assert_eq!(stats.snapshot(), (8 * 375, 8 * 125));
    }

    #[test]
    fn concurrent_drains_partition_the_counts() {
        let stats = CacheStats::new();
        let stats = &stats;

        thread::scope(|s| {
            let writer = s.spawn(move || {
                for _ in 0..2000 {
                    stats.record(true);
                }
            });

            let drainer = s.spawn(move || {
                let mut collected = 0;
                for _ in 0..500 {
                    collected += stats.drain().0;
                }
                collected
            });

            writer.join().unwrap();
            let drained = drainer.join().unwrap();
            let leftover = stats.drain().0;

            // Every recorded hit is counted exactly once: swap hands the
            // old value out and puts zero in, atomically.
            assert_eq!(drained + leftover, 2000);
        });
    }
}
