use mpsc_fan_in::*;

mod collect_all_tests {
    use super::*;

    #[test]
    fn no_chunks_gives_nothing() {
        assert_eq!(collect_all(vec![]), Vec::<u64>::new());
    }

    #[test]
    fn only_empty_chunks_gives_nothing() {
        assert_eq!(collect_all(vec![vec![], vec![], vec![]]), Vec::<u64>::new());
    }

    #[test]
    fn a_single_chunk_comes_back_sorted() {
        assert_eq!(collect_all(vec![vec![5, 1, 3]]), vec![1, 3, 5]);
    }

    #[test]
    fn values_from_every_worker_arrive() {
        let chunks = vec![vec![3, 1], vec![2], vec![], vec![4, 6, 5]];

        assert_eq!(collect_all(chunks), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn duplicates_are_kept() {
        let chunks = vec![vec![7, 7], vec![7], vec![7]];

        assert_eq!(collect_all(chunks), vec![7, 7, 7, 7]);
    }

    #[test]
    fn nothing_is_lost_under_load() {
        let chunks: Vec<Vec<u64>> = (0..8)
            .map(|worker: u64| ((worker * 50)..(worker * 50 + 50)).collect())
            .collect();

        let collected = collect_all(chunks);

        assert_eq!(collected.len(), 400);
        assert_eq!(collected, (0..400u64).collect::<Vec<_>>());
    }
}

mod sum_by_worker_tests {
    use super::*;

    #[test]
    fn no_chunks_gives_no_sums() {
        assert_eq!(sum_by_worker(vec![]), Vec::<u64>::new());
    }

    #[test]
    fn one_sum_per_chunk_including_empty_ones() {
        let chunks = vec![vec![1, 2, 3], vec![], vec![10]];

        assert_eq!(sum_by_worker(chunks), vec![6, 0, 10]);
    }

    #[test]
    fn sums_stay_in_input_order() {
        let chunks = vec![vec![100], vec![1], vec![10], vec![1000]];

        assert_eq!(sum_by_worker(chunks), vec![100, 1, 10, 1000]);
    }

    #[test]
    fn uneven_chunk_sizes_do_not_shuffle_the_result() {
        // Descending sizes: a solution that assumes messages arrive in
        // worker order, or that indexes by arrival, gets this wrong.
        let chunks: Vec<Vec<u64>> = (0..6)
            .map(|worker: u64| vec![1; (6 - worker) as usize * 20])
            .collect();

        let expected: Vec<u64> = (0..6).map(|worker: u64| (6 - worker) * 20).collect();

        assert_eq!(sum_by_worker(chunks), expected);
    }

    #[test]
    fn totals_match_a_sequential_fold() {
        let chunks: Vec<Vec<u64>> = (0..8)
            .map(|worker: u64| ((worker * 30)..(worker * 30 + 30)).collect())
            .collect();

        let expected: Vec<u64> = chunks.iter().map(|chunk| chunk.iter().sum()).collect();

        assert_eq!(sum_by_worker(chunks), expected);
    }
}
