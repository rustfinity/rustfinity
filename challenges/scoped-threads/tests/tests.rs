use scoped_threads::*;

mod parallel_max_tests {
    use super::*;

    #[test]
    fn finds_the_largest() {
        assert_eq!(parallel_max(&[3, 9, 1, 4]), Some(9));
    }

    #[test]
    fn empty_slice_is_none() {
        assert_eq!(parallel_max(&[]), None);
    }

    #[test]
    fn single_element() {
        assert_eq!(parallel_max(&[7]), Some(7));
    }

    #[test]
    fn largest_in_the_first_half() {
        assert_eq!(parallel_max(&[100, 2, 3, 4]), Some(100));
    }

    #[test]
    fn largest_in_the_second_half() {
        assert_eq!(parallel_max(&[1, 2, 3, 100]), Some(100));
    }

    #[test]
    fn handles_negatives() {
        assert_eq!(parallel_max(&[-9, -3, -20, -1]), Some(-1));
    }

    #[test]
    fn odd_length_slice() {
        assert_eq!(parallel_max(&[5, 8, 2]), Some(8));
    }

    #[test]
    fn borrows_a_local_without_cloning_it() {
        let data: Vec<i32> = (0..500).collect();
        assert_eq!(parallel_max(&data), Some(499));
    }
}

mod split_evens_odds_tests {
    use super::*;

    #[test]
    fn splits_a_short_slice() {
        let (evens, odds) = split_evens_odds(&[1, 2, 3, 4]);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3]);
    }

    #[test]
    fn empty_slice_gives_two_empty_vectors() {
        let (evens, odds) = split_evens_odds(&[]);
        assert!(evens.is_empty());
        assert!(odds.is_empty());
    }

    #[test]
    fn all_even() {
        let (evens, odds) = split_evens_odds(&[2, 4, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert!(odds.is_empty());
    }

    #[test]
    fn all_odd() {
        let (evens, odds) = split_evens_odds(&[1, 3, 5]);
        assert!(evens.is_empty());
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn keeps_relative_order() {
        let (evens, odds) = split_evens_odds(&[9, 8, 7, 6, 5, 4]);
        assert_eq!(evens, vec![8, 6, 4]);
        assert_eq!(odds, vec![9, 7, 5]);
    }

    #[test]
    fn negative_numbers_land_correctly() {
        let (evens, odds) = split_evens_odds(&[-2, -1, 0, 1]);
        assert_eq!(evens, vec![-2, 0]);
        assert_eq!(odds, vec![-1, 1]);
    }

    #[test]
    fn every_value_appears_exactly_once() {
        let data: Vec<i32> = (0..200).collect();
        let (evens, odds) = split_evens_odds(&data);

        assert_eq!(evens.len() + odds.len(), 200);
        assert_eq!(evens.len(), 100);
    }
}
