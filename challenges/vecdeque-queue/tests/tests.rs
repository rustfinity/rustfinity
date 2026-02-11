use std::collections::VecDeque;
use vecdeque_queue::*;

// ============================================================================
// Tests for create_queue
// ============================================================================

#[test]
fn test_create_queue_basic() {
    let queue = create_queue(&[1, 2, 3]);
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.front(), Some(&1));
    assert_eq!(queue.back(), Some(&3));
}

#[test]
fn test_create_queue_empty() {
    let queue: VecDeque<i32> = create_queue(&[]);
    assert!(queue.is_empty());
}

#[test]
fn test_create_queue_single() {
    let queue = create_queue(&["only"]);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front(), Some(&"only"));
    assert_eq!(queue.back(), Some(&"only"));
}

#[test]
fn test_create_queue_strings() {
    let queue = create_queue(&["first".to_string(), "second".to_string()]);
    assert_eq!(queue.front(), Some(&"first".to_string()));
    assert_eq!(queue.back(), Some(&"second".to_string()));
}

#[test]
fn test_create_queue_preserves_order() {
    let items: Vec<i32> = (0..10).collect();
    let queue = create_queue(&items);
    for (i, item) in queue.iter().enumerate() {
        assert_eq!(*item, i as i32);
    }
}

// ============================================================================
// Tests for enqueue
// ============================================================================

#[test]
fn test_enqueue_basic() {
    let mut queue = create_queue(&[1, 2]);
    enqueue(&mut queue, 3);
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.back(), Some(&3));
}

#[test]
fn test_enqueue_to_empty() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    enqueue(&mut queue, 42);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front(), Some(&42));
    assert_eq!(queue.back(), Some(&42));
}

#[test]
fn test_enqueue_multiple() {
    let mut queue: VecDeque<char> = VecDeque::new();
    enqueue(&mut queue, 'a');
    enqueue(&mut queue, 'b');
    enqueue(&mut queue, 'c');
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.front(), Some(&'a'));
    assert_eq!(queue.back(), Some(&'c'));
}

#[test]
fn test_enqueue_strings() {
    let mut queue = create_queue(&["first".to_string()]);
    enqueue(&mut queue, "second".to_string());
    assert_eq!(queue.back(), Some(&"second".to_string()));
}

// ============================================================================
// Tests for dequeue
// ============================================================================

#[test]
fn test_dequeue_basic() {
    let mut queue = create_queue(&[1, 2, 3]);
    assert_eq!(dequeue(&mut queue), Some(1));
    assert_eq!(dequeue(&mut queue), Some(2));
    assert_eq!(dequeue(&mut queue), Some(3));
    assert_eq!(dequeue(&mut queue), None);
}

#[test]
fn test_dequeue_empty() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    assert_eq!(dequeue(&mut queue), None);
}

#[test]
fn test_dequeue_single() {
    let mut queue = create_queue(&["only"]);
    assert_eq!(dequeue(&mut queue), Some("only"));
    assert!(queue.is_empty());
}

#[test]
fn test_dequeue_fifo_order() {
    let mut queue = create_queue(&["first", "second", "third"]);
    enqueue(&mut queue, "fourth");

    assert_eq!(dequeue(&mut queue), Some("first"));
    assert_eq!(dequeue(&mut queue), Some("second"));
    assert_eq!(dequeue(&mut queue), Some("third"));
    assert_eq!(dequeue(&mut queue), Some("fourth"));
}

#[test]
fn test_dequeue_interleaved() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    enqueue(&mut queue, 1);
    enqueue(&mut queue, 2);
    assert_eq!(dequeue(&mut queue), Some(1));
    enqueue(&mut queue, 3);
    assert_eq!(dequeue(&mut queue), Some(2));
    assert_eq!(dequeue(&mut queue), Some(3));
}

// ============================================================================
// Tests for peek_front
// ============================================================================

#[test]
fn test_peek_front_basic() {
    let queue = create_queue(&[10, 20, 30]);
    assert_eq!(peek_front(&queue), Some(&10));
    assert_eq!(queue.len(), 3); // Still has all elements
}

#[test]
fn test_peek_front_empty() {
    let queue: VecDeque<i32> = VecDeque::new();
    assert_eq!(peek_front(&queue), None);
}

#[test]
fn test_peek_front_single() {
    let queue = create_queue(&[42]);
    assert_eq!(peek_front(&queue), Some(&42));
}

#[test]
fn test_peek_front_after_dequeue() {
    let mut queue = create_queue(&[1, 2, 3]);
    dequeue(&mut queue);
    assert_eq!(peek_front(&queue), Some(&2));
}

#[test]
fn test_peek_front_does_not_modify() {
    let queue = create_queue(&["first", "second"]);
    let _ = peek_front(&queue);
    let _ = peek_front(&queue);
    let _ = peek_front(&queue);
    assert_eq!(queue.len(), 2);
    assert_eq!(peek_front(&queue), Some(&"first"));
}

// ============================================================================
// Tests for peek_back
// ============================================================================

#[test]
fn test_peek_back_basic() {
    let queue = create_queue(&[10, 20, 30]);
    assert_eq!(peek_back(&queue), Some(&30));
    assert_eq!(queue.len(), 3);
}

#[test]
fn test_peek_back_empty() {
    let queue: VecDeque<i32> = VecDeque::new();
    assert_eq!(peek_back(&queue), None);
}

#[test]
fn test_peek_back_single() {
    let queue = create_queue(&[42]);
    assert_eq!(peek_back(&queue), Some(&42));
}

#[test]
fn test_peek_back_after_enqueue() {
    let mut queue = create_queue(&[1, 2]);
    enqueue(&mut queue, 3);
    assert_eq!(peek_back(&queue), Some(&3));
}

#[test]
fn test_peek_front_and_back_same_single() {
    let queue = create_queue(&[99]);
    assert_eq!(peek_front(&queue), peek_back(&queue));
    assert_eq!(peek_front(&queue), Some(&99));
}

// ============================================================================
// Tests for rotate_left
// ============================================================================

#[test]
fn test_rotate_left_basic() {
    let mut queue = create_queue(&[1, 2, 3, 4, 5]);
    rotate_left(&mut queue, 2);
    // [1, 2, 3, 4, 5] -> [3, 4, 5, 1, 2]
    let expected: VecDeque<i32> = VecDeque::from([3, 4, 5, 1, 2]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_left_zero() {
    let mut queue = create_queue(&[1, 2, 3]);
    rotate_left(&mut queue, 0);
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_left_empty() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    rotate_left(&mut queue, 5); // Should not panic
    assert!(queue.is_empty());
}

#[test]
fn test_rotate_left_single() {
    let mut queue = create_queue(&[42]);
    rotate_left(&mut queue, 1);
    assert_eq!(queue.front(), Some(&42));
}

#[test]
fn test_rotate_left_full_rotation() {
    let mut queue = create_queue(&[1, 2, 3]);
    rotate_left(&mut queue, 3); // Full rotation = no change
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_left_more_than_len() {
    let mut queue = create_queue(&[1, 2, 3, 4]);
    rotate_left(&mut queue, 6); // 6 % 4 = 2
    // [1, 2, 3, 4] -> [3, 4, 1, 2]
    let expected: VecDeque<i32> = VecDeque::from([3, 4, 1, 2]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_left_one() {
    let mut queue = create_queue(&[1, 2, 3, 4]);
    rotate_left(&mut queue, 1);
    let expected: VecDeque<i32> = VecDeque::from([2, 3, 4, 1]);
    assert_eq!(queue, expected);
}

// ============================================================================
// Tests for rotate_right
// ============================================================================

#[test]
fn test_rotate_right_basic() {
    let mut queue = create_queue(&[1, 2, 3, 4, 5]);
    rotate_right(&mut queue, 2);
    // [1, 2, 3, 4, 5] -> [4, 5, 1, 2, 3]
    let expected: VecDeque<i32> = VecDeque::from([4, 5, 1, 2, 3]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_right_zero() {
    let mut queue = create_queue(&[1, 2, 3]);
    rotate_right(&mut queue, 0);
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_right_empty() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    rotate_right(&mut queue, 5); // Should not panic
    assert!(queue.is_empty());
}

#[test]
fn test_rotate_right_single() {
    let mut queue = create_queue(&[42]);
    rotate_right(&mut queue, 1);
    assert_eq!(queue.front(), Some(&42));
}

#[test]
fn test_rotate_right_full_rotation() {
    let mut queue = create_queue(&[1, 2, 3]);
    rotate_right(&mut queue, 3); // Full rotation = no change
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_right_more_than_len() {
    let mut queue = create_queue(&[1, 2, 3, 4]);
    rotate_right(&mut queue, 6); // 6 % 4 = 2
    // [1, 2, 3, 4] -> [3, 4, 1, 2]
    let expected: VecDeque<i32> = VecDeque::from([3, 4, 1, 2]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_right_one() {
    let mut queue = create_queue(&[1, 2, 3, 4]);
    rotate_right(&mut queue, 1);
    let expected: VecDeque<i32> = VecDeque::from([4, 1, 2, 3]);
    assert_eq!(queue, expected);
}

// ============================================================================
// Tests for rotation inverses
// ============================================================================

#[test]
fn test_rotate_left_then_right_inverse() {
    let mut queue = create_queue(&[1, 2, 3, 4, 5]);
    rotate_left(&mut queue, 2);
    rotate_right(&mut queue, 2);
    // Should be back to original
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
    assert_eq!(queue, expected);
}

#[test]
fn test_rotate_right_then_left_inverse() {
    let mut queue = create_queue(&[1, 2, 3, 4, 5]);
    rotate_right(&mut queue, 3);
    rotate_left(&mut queue, 3);
    // Should be back to original
    let expected: VecDeque<i32> = VecDeque::from([1, 2, 3, 4, 5]);
    assert_eq!(queue, expected);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_task_queue_workflow() {
    let mut tasks: VecDeque<&str> = VecDeque::new();

    // Add tasks
    enqueue(&mut tasks, "download");
    enqueue(&mut tasks, "process");
    enqueue(&mut tasks, "upload");

    // Check front task without removing
    assert_eq!(peek_front(&tasks), Some(&"download"));

    // Process tasks in order
    assert_eq!(dequeue(&mut tasks), Some("download"));
    assert_eq!(dequeue(&mut tasks), Some("process"));

    // Add more tasks
    enqueue(&mut tasks, "cleanup");

    // Continue processing
    assert_eq!(dequeue(&mut tasks), Some("upload"));
    assert_eq!(dequeue(&mut tasks), Some("cleanup"));
    assert_eq!(dequeue(&mut tasks), None);
}

#[test]
fn test_sliding_window_simulation() {
    // Simulate a sliding window of size 3
    let mut window = create_queue(&[1, 2, 3]);

    // Slide: remove front, add to back
    dequeue(&mut window);
    enqueue(&mut window, 4);
    assert_eq!(peek_front(&window), Some(&2));
    assert_eq!(peek_back(&window), Some(&4));

    dequeue(&mut window);
    enqueue(&mut window, 5);
    assert_eq!(peek_front(&window), Some(&3));
    assert_eq!(peek_back(&window), Some(&5));
}

#[test]
fn test_round_robin_scheduling() {
    let mut processes = create_queue(&["P1", "P2", "P3"]);

    // Round robin: move front to back
    rotate_left(&mut processes, 1);
    assert_eq!(peek_front(&processes), Some(&"P2"));

    rotate_left(&mut processes, 1);
    assert_eq!(peek_front(&processes), Some(&"P3"));

    rotate_left(&mut processes, 1);
    assert_eq!(peek_front(&processes), Some(&"P1"));
}

#[test]
fn test_with_complex_type() {
    #[derive(Debug, Clone, PartialEq)]
    struct Task {
        id: u32,
        name: String,
    }

    let tasks = vec![
        Task { id: 1, name: "first".to_string() },
        Task { id: 2, name: "second".to_string() },
    ];
    let mut queue = create_queue(&tasks);

    enqueue(&mut queue, Task { id: 3, name: "third".to_string() });

    assert_eq!(peek_front(&queue).unwrap().id, 1);
    assert_eq!(peek_back(&queue).unwrap().id, 3);

    let dequeued = dequeue(&mut queue).unwrap();
    assert_eq!(dequeued.name, "first");
}

#[test]
fn test_large_queue() {
    let items: Vec<i32> = (0..1000).collect();
    let mut queue = create_queue(&items);

    assert_eq!(queue.len(), 1000);
    assert_eq!(peek_front(&queue), Some(&0));
    assert_eq!(peek_back(&queue), Some(&999));

    // Dequeue first 100
    for i in 0..100 {
        assert_eq!(dequeue(&mut queue), Some(i));
    }

    assert_eq!(queue.len(), 900);
    assert_eq!(peek_front(&queue), Some(&100));

    // Rotate
    rotate_left(&mut queue, 50);
    assert_eq!(peek_front(&queue), Some(&150));
}
