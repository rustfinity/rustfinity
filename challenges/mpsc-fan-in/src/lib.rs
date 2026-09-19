use std::sync::mpsc;
use std::thread;

/// Runs one thread per chunk and gathers every value through a single
/// channel, returned in ascending order.
///
/// The shape to learn here is the drop. `rx` iteration ends when the last
/// `Sender` is gone, and the `tx` created by `channel()` is one of them.
/// Move a clone into each worker and let this function's own `tx` fall out
/// of scope, or the loop below never finishes.
///
/// Arrival order is whatever the scheduler decides, so the result is
/// sorted rather than asserted on as-is.
///
/// # Examples
///
/// ```
/// use mpsc_fan_in::collect_all;
///
/// let chunks = vec![vec![3, 1], vec![2], vec![]];
/// assert_eq!(collect_all(chunks), vec![1, 2, 3]);
/// ```
pub fn collect_all(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    let (tx, rx) = mpsc::channel();

    thread::scope(|scope| {
        for chunk in chunks {
            let tx = tx.clone();

            scope.spawn(move || {
                for value in chunk {
                    tx.send(value).unwrap();
                }
            });
        }

        // Every remaining sender lives in a worker now. Without this the
        // receiver would wait forever for a message from us.
        drop(tx);

        let mut collected: Vec<u64> = rx.iter().collect();
        collected.sort_unstable();
        collected
    })
}

/// Sums each chunk, keeping the results lined up with the input.
///
/// Fanning in loses the sender's identity, so each worker tags its
/// message with its own index. The consumer puts every value back in the
/// right slot no matter what order the messages turn up in.
///
/// # Examples
///
/// ```
/// use mpsc_fan_in::sum_by_worker;
///
/// let chunks = vec![vec![1, 2, 3], vec![], vec![10]];
/// assert_eq!(sum_by_worker(chunks), vec![6, 0, 10]);
/// ```
pub fn sum_by_worker(chunks: Vec<Vec<u64>>) -> Vec<u64> {
    let (tx, rx) = mpsc::channel();
    let mut sums = vec![0; chunks.len()];

    thread::scope(|scope| {
        for (index, chunk) in chunks.into_iter().enumerate() {
            let tx = tx.clone();

            scope.spawn(move || {
                for value in chunk {
                    tx.send((index, value)).unwrap();
                }
            });
        }

        drop(tx);

        for (index, value) in rx {
            sums[index] += value;
        }
    });

    sums
}
