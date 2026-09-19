use barrier_synchronization::*;

/// Reference implementation of one round, done sequentially.
fn step(cells: &[i64]) -> Vec<i64> {
    (0..cells.len())
        .map(|i| cells[i] + cells[(i + 1) % cells.len()])
        .collect()
}

mod run_rounds_tests {
    use super::*;

    #[test]
    fn zero_rounds_returns_the_input() {
        assert_eq!(run_rounds(vec![1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(run_rounds(vec![], 4), Vec::<i64>::new());
    }

    #[test]
    fn one_cell_doubles_every_round() {
        assert_eq!(run_rounds(vec![5], 3), vec![40]);
    }

    #[test]
    fn one_round_adds_the_right_neighbour() {
        assert_eq!(run_rounds(vec![1, 2, 3], 1), vec![3, 5, 4]);
    }

    #[test]
    fn two_rounds_build_on_the_first() {
        assert_eq!(run_rounds(vec![1, 2, 3], 2), vec![8, 9, 7]);
    }

    #[test]
    fn matches_the_sequential_reference() {
        let start = vec![3, -1, 4, 1, -5, 9, 2, 6];

        for rounds in 0..6 {
            let mut expected = start.clone();
            for _ in 0..rounds {
                expected = step(&expected);
            }

            assert_eq!(
                run_rounds(start.clone(), rounds),
                expected,
                "mismatch after {rounds} rounds"
            );
        }
    }

    #[test]
    fn phases_are_not_allowed_to_overlap() {
        // Without a barrier between the read and the write phase, a fast
        // thread's fresh value leaks into a slow thread's input and the
        // totals drift. Every round exactly doubles the sum, so a single
        // leak is visible here.
        let start: Vec<i64> = (1..=12).collect();
        let sum: i64 = start.iter().sum();

        let result = run_rounds(start, 5);

        assert_eq!(result.iter().sum::<i64>(), sum * 32);
    }
}

mod count_leaders_tests {
    use super::*;

    #[test]
    fn one_leader_per_round() {
        assert_eq!(count_leaders(4, 3), 3);
    }

    #[test]
    fn a_single_party_leads_every_time() {
        assert_eq!(count_leaders(1, 7), 7);
    }

    #[test]
    fn no_parties_means_no_leaders() {
        assert_eq!(count_leaders(0, 5), 0);
    }

    #[test]
    fn no_rounds_means_no_leaders() {
        assert_eq!(count_leaders(6, 0), 0);
    }

    #[test]
    fn holds_for_many_parties_and_rounds() {
        assert_eq!(count_leaders(8, 20), 20);
    }
}
