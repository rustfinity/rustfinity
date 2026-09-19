use spawning_threads::*;

mod sum_in_thread_tests {
    use super::*;

    #[test]
    fn sums_a_short_list() {
        assert_eq!(sum_in_thread(vec![1, 2, 3]), 6);
    }

    #[test]
    fn sums_an_empty_list() {
        assert_eq!(sum_in_thread(vec![]), 0);
    }

    #[test]
    fn sums_a_single_element() {
        assert_eq!(sum_in_thread(vec![42]), 42);
    }

    #[test]
    fn sums_a_longer_list() {
        let values: Vec<u64> = (1..=100).collect();
        assert_eq!(sum_in_thread(values), 5050);
    }

    #[test]
    fn sums_large_values_without_overflow() {
        let big = u32::MAX as u64;
        assert_eq!(sum_in_thread(vec![big, big]), big * 2);
    }
}

mod spawn_counter_tests {
    use super::*;

    #[test]
    fn counts_three_steps() {
        assert_eq!(spawn_counter(10, 3).join().unwrap(), 33);
    }

    #[test]
    fn zero_steps_is_zero() {
        assert_eq!(spawn_counter(7, 0).join().unwrap(), 0);
    }

    #[test]
    fn one_step_is_the_start_value() {
        assert_eq!(spawn_counter(5, 1).join().unwrap(), 5);
    }

    #[test]
    fn counts_from_zero() {
        assert_eq!(spawn_counter(0, 5).join().unwrap(), 10);
    }

    #[test]
    fn several_handles_can_run_at_once() {
        let handles: Vec<_> = (0..4).map(|i| spawn_counter(i, 4)).collect();

        let results: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results, vec![6, 10, 14, 18]);
    }

    #[test]
    fn work_happens_before_join_is_called() {
        // The handle is created first and joined last, so the caller is
        // free to do other work in between.
        let handle = spawn_counter(100, 3);
        let local = 1 + 1;

        assert_eq!(local, 2);
        assert_eq!(handle.join().unwrap(), 303);
    }
}
