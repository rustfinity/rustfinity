use sync_channel_backpressure::*;

mod drain_tests {
    use super::*;

    #[test]
    fn nothing_to_move_is_fine() {
        assert_eq!(drain(vec![], 4), Vec::<u64>::new());
    }

    #[test]
    fn order_is_preserved() {
        assert_eq!(drain(vec![5, 1, 4, 1, 3], 8), vec![5, 1, 4, 1, 3]);
    }

    #[test]
    fn a_rendezvous_channel_still_delivers() {
        // Capacity 0 means every send waits for a matching receive.
        assert_eq!(drain(vec![1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn capacity_of_one_does_not_deadlock() {
        assert_eq!(drain(vec![9, 8, 7], 1), vec![9, 8, 7]);
    }

    #[test]
    fn far_more_items_than_capacity() {
        let items: Vec<u64> = (0..500).collect();

        assert_eq!(drain(items.clone(), 2), items);
    }

    #[test]
    fn capacity_larger_than_the_input() {
        assert_eq!(drain(vec![1, 2], 64), vec![1, 2]);
    }
}

mod fill_without_receiving_tests {
    use super::*;

    #[test]
    fn a_rendezvous_channel_accepts_nothing() {
        assert_eq!(fill_without_receiving(vec![1, 2, 3], 0), 0);
    }

    #[test]
    fn it_stops_at_the_capacity() {
        assert_eq!(fill_without_receiving(vec![1, 2, 3, 4, 5], 3), 3);
    }

    #[test]
    fn a_short_input_runs_out_first() {
        assert_eq!(fill_without_receiving(vec![1, 2], 10), 2);
    }

    #[test]
    fn no_items_means_no_acceptances() {
        assert_eq!(fill_without_receiving(vec![], 4), 0);
    }

    #[test]
    fn exactly_capacity_many_items() {
        assert_eq!(fill_without_receiving(vec![1, 2, 3, 4], 4), 4);
    }

    #[test]
    fn the_bound_holds_for_several_capacities() {
        let items: Vec<u64> = (0..100).collect();

        for capacity in 0..12 {
            assert_eq!(
                fill_without_receiving(items.clone(), capacity),
                capacity,
                "capacity {capacity} should accept exactly {capacity} items"
            );
        }
    }
}
