use collecting_thread_results::*;

mod sum_chunks_tests {
    use super::*;

    #[test]
    fn sums_three_chunks() {
        let chunks = vec![vec![1, 2], vec![10], vec![]];
        assert_eq!(sum_chunks(chunks), vec![3, 10, 0]);
    }

    #[test]
    fn no_chunks_gives_no_sums() {
        assert_eq!(sum_chunks(vec![]), Vec::<u64>::new());
    }

    #[test]
    fn one_result_per_chunk() {
        let chunks: Vec<Vec<u64>> = (0..8).map(|i| vec![i; 3]).collect();
        assert_eq!(sum_chunks(chunks).len(), 8);
    }

    #[test]
    fn order_follows_the_input_not_the_finish_time() {
        // Descending sizes: the first chunk is the slowest, yet it must
        // still come first in the output.
        let chunks: Vec<Vec<u64>> = (1..=6).rev().map(|n| vec![1; n * 200]).collect();

        let expected: Vec<u64> = (1..=6).rev().map(|n| (n * 200) as u64).collect();
        assert_eq!(sum_chunks(chunks), expected);
    }

    #[test]
    fn empty_chunks_are_kept_as_zero() {
        let chunks = vec![vec![], vec![5], vec![], vec![7]];
        assert_eq!(sum_chunks(chunks), vec![0, 5, 0, 7]);
    }

    #[test]
    fn handles_large_values() {
        let big = u32::MAX as u64;
        assert_eq!(sum_chunks(vec![vec![big, big]]), vec![big * 2]);
    }
}

mod sum_all_tests {
    use super::*;

    #[test]
    fn adds_the_partial_sums() {
        assert_eq!(sum_all(vec![vec![1, 2], vec![3]]), 6);
    }

    #[test]
    fn no_chunks_is_zero() {
        assert_eq!(sum_all(vec![]), 0);
    }

    #[test]
    fn only_empty_chunks_is_zero() {
        assert_eq!(sum_all(vec![vec![], vec![]]), 0);
    }

    #[test]
    fn matches_a_flat_sum() {
        let chunks: Vec<Vec<u64>> = (0..10)
            .map(|i| (0..10).map(|j| i * 10 + j).collect())
            .collect();

        let flat: u64 = (0..100).sum();
        assert_eq!(sum_all(chunks), flat);
    }
}
