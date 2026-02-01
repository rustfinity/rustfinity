use std::collections::VecDeque;

/// Creates a VecDeque from a slice, preserving order.
///
/// The first element of the slice becomes the front of the queue.
///
/// # Arguments
///
/// * `items` - A slice of items to create the queue from
///
/// # Returns
///
/// A VecDeque containing all items in the same order
///
/// # Examples
///
/// ```
/// use vecdeque_queue::create_queue;
///
/// let queue = create_queue(&[1, 2, 3]);
/// assert_eq!(queue.front(), Some(&1));
/// assert_eq!(queue.back(), Some(&3));
/// ```
pub fn create_queue<T: Clone>(items: &[T]) -> VecDeque<T> {
    items.iter().cloned().collect()
}

/// Adds an item to the back of the queue.
///
/// # Arguments
///
/// * `queue` - The VecDeque to add to
/// * `item` - The item to add
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, enqueue};
///
/// let mut queue = create_queue(&[1, 2]);
/// enqueue(&mut queue, 3);
/// assert_eq!(queue.back(), Some(&3));
/// ```
pub fn enqueue<T>(queue: &mut VecDeque<T>, item: T) {
    queue.push_back(item);
}

/// Removes and returns the item from the front of the queue.
///
/// # Arguments
///
/// * `queue` - The VecDeque to remove from
///
/// # Returns
///
/// `Some(item)` if the queue is not empty, `None` otherwise
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, dequeue};
///
/// let mut queue = create_queue(&["first", "second"]);
/// assert_eq!(dequeue(&mut queue), Some("first"));
/// assert_eq!(dequeue(&mut queue), Some("second"));
/// assert_eq!(dequeue(&mut queue), None);
/// ```
pub fn dequeue<T>(queue: &mut VecDeque<T>) -> Option<T> {
    queue.pop_front()
}

/// Returns a reference to the front item without removing it.
///
/// # Arguments
///
/// * `queue` - The VecDeque to peek
///
/// # Returns
///
/// `Some(&item)` if the queue is not empty, `None` otherwise
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, peek_front};
///
/// let queue = create_queue(&[10, 20, 30]);
/// assert_eq!(peek_front(&queue), Some(&10));
/// assert_eq!(queue.len(), 3); // Still has 3 elements
/// ```
pub fn peek_front<T>(queue: &VecDeque<T>) -> Option<&T> {
    queue.front()
}

/// Returns a reference to the back item without removing it.
///
/// # Arguments
///
/// * `queue` - The VecDeque to peek
///
/// # Returns
///
/// `Some(&item)` if the queue is not empty, `None` otherwise
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, peek_back};
///
/// let queue = create_queue(&[10, 20, 30]);
/// assert_eq!(peek_back(&queue), Some(&30));
/// assert_eq!(queue.len(), 3); // Still has 3 elements
/// ```
pub fn peek_back<T>(queue: &VecDeque<T>) -> Option<&T> {
    queue.back()
}

/// Rotates the queue left so the front n elements move to the back.
///
/// After rotation, the element at index n becomes the new front.
/// If n is 0 or the queue is empty, this is a no-op.
/// If n >= len, it wraps around (equivalent to n % len rotations).
///
/// # Arguments
///
/// * `queue` - The VecDeque to rotate
/// * `n` - Number of positions to rotate left
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, rotate_left};
///
/// let mut queue = create_queue(&[1, 2, 3, 4, 5]);
/// rotate_left(&mut queue, 2);
/// // [1, 2, 3, 4, 5] -> [3, 4, 5, 1, 2]
/// assert_eq!(queue.front(), Some(&3));
/// ```
pub fn rotate_left<T>(queue: &mut VecDeque<T>, n: usize) {
    if queue.is_empty() || n == 0 {
        return;
    }
    let len = queue.len();
    let effective_n = n % len;
    if effective_n > 0 {
        queue.rotate_left(effective_n);
    }
}

/// Rotates the queue right so the back n elements move to the front.
///
/// After rotation, the element at index (len - n) becomes the new front.
/// If n is 0 or the queue is empty, this is a no-op.
/// If n >= len, it wraps around (equivalent to n % len rotations).
///
/// # Arguments
///
/// * `queue` - The VecDeque to rotate
/// * `n` - Number of positions to rotate right
///
/// # Examples
///
/// ```
/// use vecdeque_queue::{create_queue, rotate_right};
///
/// let mut queue = create_queue(&[1, 2, 3, 4, 5]);
/// rotate_right(&mut queue, 2);
/// // [1, 2, 3, 4, 5] -> [4, 5, 1, 2, 3]
/// assert_eq!(queue.front(), Some(&4));
/// ```
pub fn rotate_right<T>(queue: &mut VecDeque<T>, n: usize) {
    if queue.is_empty() || n == 0 {
        return;
    }
    let len = queue.len();
    let effective_n = n % len;
    if effective_n > 0 {
        queue.rotate_right(effective_n);
    }
}
