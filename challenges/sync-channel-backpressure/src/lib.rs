use std::sync::mpsc::{self, TrySendError};
use std::thread;

/// Moves every item from a producer thread to this thread through a
/// bounded channel, preserving order.
///
/// `sync_channel(capacity)` is the bounded sibling of `channel()`. Once
/// `capacity` items are sitting in the buffer, the next `send` parks the
/// producer until the consumer takes one out. That parking is the whole
/// point: a slow consumer slows the producer down instead of letting an
/// unbounded queue grow until memory runs out.
///
/// Only one sender exists here, so arrival order is send order and no
/// sorting is needed. The `drop(tx)` is still required, otherwise the
/// receiver would wait for a message that is never coming.
///
/// # Examples
///
/// ```
/// use sync_channel_backpressure::drain;
///
/// assert_eq!(drain(vec![1, 2, 3], 1), vec![1, 2, 3]);
/// ```
pub fn drain(items: Vec<u64>, capacity: usize) -> Vec<u64> {
    let (tx, rx) = mpsc::sync_channel(capacity);

    thread::scope(|scope| {
        scope.spawn(move || {
            for item in items {
                // Blocks here whenever the buffer is full.
                tx.send(item).unwrap();
            }
        });

        rx.iter().collect()
    })
}

/// Pushes items with `try_send` and reports how many the channel took
/// before it filled up.
///
/// `try_send` is the non blocking half of the API: instead of parking, it
/// hands the item back as `TrySendError::Full`. With nobody receiving,
/// exactly `capacity` items fit, so a capacity of 0 accepts nothing at
/// all. A rendezvous channel only moves an item when a receiver is
/// already waiting for it.
///
/// The receiver has to stay alive for the whole function. Dropping it
/// would turn every send into `TrySendError::Disconnected`, which is a
/// different failure and must not be counted as acceptance.
///
/// # Examples
///
/// ```
/// use sync_channel_backpressure::fill_without_receiving;
///
/// assert_eq!(fill_without_receiving(vec![1, 2, 3, 4], 2), 2);
/// assert_eq!(fill_without_receiving(vec![1, 2, 3, 4], 0), 0);
/// ```
pub fn fill_without_receiving(items: Vec<u64>, capacity: usize) -> usize {
    let (tx, _rx) = mpsc::sync_channel(capacity);
    let mut accepted = 0;

    for item in items {
        match tx.try_send(item) {
            Ok(()) => accepted += 1,
            Err(TrySendError::Full(_)) => break,
            Err(TrySendError::Disconnected(_)) => break,
        }
    }

    accepted
}

// Example usage
pub fn main() {
    println!("{:?}", drain(vec![1, 2, 3], 1));
    println!("{}", fill_without_receiving(vec![1, 2, 3, 4], 2));
}
