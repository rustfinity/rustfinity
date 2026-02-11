use std::collections::VecDeque;

/// Creates a VecDeque from a slice, preserving order.
///
/// The first element of the slice becomes the front of the queue.
pub fn create_queue<T: Clone>(items: &[T]) -> VecDeque<T> {
    // TODO: Iterate over items and collect into a VecDeque
    unimplemented!()
}

/// Adds an item to the back of the queue.
pub fn enqueue<T>(queue: &mut VecDeque<T>, item: T) {
    // TODO: Use push_back to add the item
    unimplemented!()
}

/// Removes and returns the item from the front of the queue.
///
/// Returns `Some(item)` if the queue is not empty, `None` otherwise.
pub fn dequeue<T>(queue: &mut VecDeque<T>) -> Option<T> {
    // TODO: Use pop_front to remove and return the front item
    unimplemented!()
}

/// Returns a reference to the front item without removing it.
pub fn peek_front<T>(queue: &VecDeque<T>) -> Option<&T> {
    // TODO: Use the front() method
    unimplemented!()
}

/// Returns a reference to the back item without removing it.
pub fn peek_back<T>(queue: &VecDeque<T>) -> Option<&T> {
    // TODO: Use the back() method
    unimplemented!()
}

/// Rotates the queue left so the front n elements move to the back.
///
/// After rotation, the element at index n becomes the new front.
/// Handle edge cases: empty queue, n == 0, n >= len.
pub fn rotate_left<T>(queue: &mut VecDeque<T>, n: usize) {
    // TODO: Handle edge cases, then use rotate_left method
    // Remember: n might be larger than queue length!
    unimplemented!()
}

/// Rotates the queue right so the back n elements move to the front.
///
/// After rotation, the element at index (len - n) becomes the new front.
/// Handle edge cases: empty queue, n == 0, n >= len.
pub fn rotate_right<T>(queue: &mut VecDeque<T>, n: usize) {
    // TODO: Handle edge cases, then use rotate_right method
    // Remember: n might be larger than queue length!
    unimplemented!()
}

// Example usage
pub fn main() {
    // Create a task queue
    let mut tasks = create_queue(&["task1", "task2", "task3"]);
    println!("Initial queue: {:?}", tasks);

    // Add a new task
    enqueue(&mut tasks, "task4");
    println!("After enqueue: {:?}", tasks);

    // Process tasks (FIFO order)
    while let Some(task) = dequeue(&mut tasks) {
        println!("Processing: {}", task);
    }

    // Demonstrate rotation
    let mut numbers = create_queue(&[1, 2, 3, 4, 5]);
    println!("\nBefore rotation: {:?}", numbers);

    rotate_left(&mut numbers, 2);
    println!("After rotate_left(2): {:?}", numbers);

    rotate_right(&mut numbers, 2);
    println!("After rotate_right(2): {:?}", numbers);
}
