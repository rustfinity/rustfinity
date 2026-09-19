use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Barrier;
use std::thread;

/// Runs one "read everybody, then write everybody" step per round.
///
/// Every cell is replaced by itself plus its right neighbour, wrapping at
/// the end. One thread owns one cell for the whole run.
///
/// The whole point is the phase split. A round has a read phase and a
/// write phase, and no thread may start writing while another is still
/// reading, or the fast thread's new value leaks into the slow thread's
/// input. Two `Barrier::wait` calls per round enforce that: one after the
/// reads, one after the writes.
///
/// The barrier is built with `cells.len()` parties, so every `wait`
/// blocks until all of them have arrived and then releases them together.
///
/// Returns the cells after `rounds` rounds. With no cells, or zero
/// rounds, the input comes straight back.
///
/// # Examples
///
/// ```
/// use barrier_synchronization::run_rounds;
///
/// assert_eq!(run_rounds(vec![1, 2, 3], 1), vec![3, 5, 4]);
/// assert_eq!(run_rounds(vec![1, 2, 3], 2), vec![8, 9, 7]);
/// ```
pub fn run_rounds(cells: Vec<i64>, rounds: usize) -> Vec<i64> {
    if cells.is_empty() || rounds == 0 {
        return cells;
    }

    let len = cells.len();
    let shared: Vec<AtomicI64> = cells.into_iter().map(AtomicI64::new).collect();
    let barrier = Barrier::new(len);

    thread::scope(|scope| {
        for index in 0..len {
            let shared = &shared;
            let barrier = &barrier;

            scope.spawn(move || {
                for _ in 0..rounds {
                    let mine = shared[index].load(Ordering::Relaxed);
                    let neighbour = shared[(index + 1) % len].load(Ordering::Relaxed);

                    barrier.wait();

                    shared[index].store(mine + neighbour, Ordering::Relaxed);

                    barrier.wait();
                }
            });
        }
    });

    shared.into_iter().map(|cell| cell.into_inner()).collect()
}

/// Counts how many threads were told they were the leader of a barrier.
///
/// `Barrier::wait` returns a `BarrierWaitResult`, and exactly one of the
/// waiting threads gets `is_leader() == true`. That is the standard way
/// to run a single piece of cleanup between rounds without a second lock.
///
/// Runs `parties` threads through `rounds` barriers, so the answer is
/// always `rounds`.
///
/// # Examples
///
/// ```
/// use barrier_synchronization::count_leaders;
///
/// assert_eq!(count_leaders(4, 3), 3);
/// assert_eq!(count_leaders(0, 5), 0);
/// ```
pub fn count_leaders(parties: usize, rounds: usize) -> usize {
    if parties == 0 {
        return 0;
    }

    let leaders = AtomicUsize::new(0);
    let barrier = Barrier::new(parties);

    thread::scope(|scope| {
        for _ in 0..parties {
            let leaders = &leaders;
            let barrier = &barrier;

            scope.spawn(move || {
                for _ in 0..rounds {
                    if barrier.wait().is_leader() {
                        leaders.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });

    leaders.into_inner()
}
