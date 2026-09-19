use rwlock_readers_writers::*;
use std::thread;

mod basics {
    use super::*;

    #[test]
    fn a_new_tally_is_empty() {
        let counts = WordCounts::new();
        assert_eq!(counts.total(), 0);
        assert_eq!(counts.count("anything"), 0);
        assert_eq!(counts.most_common(), None);
    }

    #[test]
    fn record_counts_repeats() {
        let counts = WordCounts::new();
        counts.record("ferris");
        counts.record("ferris");
        counts.record("crab");

        assert_eq!(counts.count("ferris"), 2);
        assert_eq!(counts.count("crab"), 1);
    }

    #[test]
    fn unknown_words_count_zero() {
        let counts = WordCounts::new();
        counts.record("ferris");
        assert_eq!(counts.count("gopher"), 0);
    }

    #[test]
    fn total_sums_every_word() {
        let counts = WordCounts::new();
        for word in ["a", "b", "b", "c", "c", "c"] {
            counts.record(word);
        }

        assert_eq!(counts.total(), 6);
    }

    #[test]
    fn words_are_case_sensitive() {
        let counts = WordCounts::new();
        counts.record("Rust");
        counts.record("rust");

        assert_eq!(counts.count("Rust"), 1);
        assert_eq!(counts.count("rust"), 1);
    }
}

mod most_common_tests {
    use super::*;

    #[test]
    fn picks_the_highest_count() {
        let counts = WordCounts::new();
        for word in ["a", "b", "b", "b", "c"] {
            counts.record(word);
        }

        assert_eq!(counts.most_common(), Some(("b".to_string(), 3)));
    }

    #[test]
    fn ties_go_to_the_alphabetically_smallest() {
        let counts = WordCounts::new();
        for word in ["zebra", "ant", "moose"] {
            counts.record(word);
        }

        assert_eq!(counts.most_common(), Some(("ant".to_string(), 1)));
    }

    #[test]
    fn a_single_word_wins() {
        let counts = WordCounts::new();
        counts.record("solo");
        assert_eq!(counts.most_common(), Some(("solo".to_string(), 1)));
    }

    #[test]
    fn tie_breaking_is_stable_across_repeated_calls() {
        let counts = WordCounts::new();
        for word in ["delta", "alpha", "charlie", "bravo"] {
            counts.record(word);
        }

        let first = counts.most_common();
        for _ in 0..50 {
            assert_eq!(counts.most_common(), first);
        }
        assert_eq!(first, Some(("alpha".to_string(), 1)));
    }
}

mod concurrency {
    use super::*;

    #[test]
    fn concurrent_writers_lose_nothing() {
        let counts = WordCounts::new();

        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..100 {
                        counts.record("shared");
                    }
                });
            }
        });

        assert_eq!(counts.count("shared"), 800);
        assert_eq!(counts.total(), 800);
    }

    #[test]
    fn writers_on_different_words_do_not_interfere() {
        let counts = WordCounts::new();
        let counts = &counts;
        let words = ["alpha", "bravo", "charlie", "delta"];

        thread::scope(|s| {
            for (index, word) in words.iter().enumerate() {
                s.spawn(move || {
                    for _ in 0..(index + 1) * 10 {
                        counts.record(word);
                    }
                });
            }
        });

        assert_eq!(counts.count("alpha"), 10);
        assert_eq!(counts.count("bravo"), 20);
        assert_eq!(counts.count("charlie"), 30);
        assert_eq!(counts.count("delta"), 40);
        assert_eq!(counts.most_common(), Some(("delta".to_string(), 40)));
    }

    #[test]
    fn readers_run_alongside_a_writer_without_deadlocking() {
        let counts = WordCounts::new();
        counts.record("seed");

        thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..200 {
                    counts.record("seed");
                }
            });

            for _ in 0..6 {
                s.spawn(|| {
                    for _ in 0..200 {
                        // Every read must observe a value in range: readers
                        // never see a half-applied increment.
                        let seen = counts.count("seed");
                        assert!((1..=201).contains(&seen), "saw {seen}");
                    }
                });
            }
        });

        assert_eq!(counts.count("seed"), 201);
    }
}
