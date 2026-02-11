use binary_heap_priority::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// ==================== create_max_heap tests ====================

#[test]
fn test_create_max_heap_basic() {
    let heap = create_max_heap(&[3, 1, 4, 1, 5]);
    assert_eq!(heap.len(), 5);
    assert_eq!(heap.peek(), Some(&5));
}

#[test]
fn test_create_max_heap_empty() {
    let heap = create_max_heap(&[]);
    assert!(heap.is_empty());
    assert_eq!(heap.peek(), None);
}

#[test]
fn test_create_max_heap_single() {
    let heap = create_max_heap(&[42]);
    assert_eq!(heap.len(), 1);
    assert_eq!(heap.peek(), Some(&42));
}

#[test]
fn test_create_max_heap_negative() {
    let heap = create_max_heap(&[-5, -1, -10, -3]);
    assert_eq!(heap.peek(), Some(&-1));
}

#[test]
fn test_create_max_heap_duplicates() {
    let heap = create_max_heap(&[5, 5, 5, 5]);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.peek(), Some(&5));
}

// ==================== create_min_heap tests ====================

#[test]
fn test_create_min_heap_basic() {
    let heap = create_min_heap(&[3, 1, 4, 1, 5]);
    assert_eq!(heap.len(), 5);
    assert_eq!(heap.peek(), Some(&Reverse(1)));
}

#[test]
fn test_create_min_heap_empty() {
    let heap = create_min_heap(&[]);
    assert!(heap.is_empty());
    assert_eq!(heap.peek(), None);
}

#[test]
fn test_create_min_heap_single() {
    let heap = create_min_heap(&[42]);
    assert_eq!(heap.len(), 1);
    assert_eq!(heap.peek(), Some(&Reverse(42)));
}

#[test]
fn test_create_min_heap_negative() {
    let heap = create_min_heap(&[-5, -1, -10, -3]);
    assert_eq!(heap.peek(), Some(&Reverse(-10)));
}

#[test]
fn test_create_min_heap_duplicates() {
    let heap = create_min_heap(&[5, 5, 5, 5]);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.peek(), Some(&Reverse(5)));
}

// ==================== pop_max tests ====================

#[test]
fn test_pop_max_basic() {
    let mut heap = create_max_heap(&[3, 1, 4]);
    assert_eq!(pop_max(&mut heap), Some(4));
    assert_eq!(pop_max(&mut heap), Some(3));
    assert_eq!(pop_max(&mut heap), Some(1));
    assert_eq!(pop_max(&mut heap), None);
}

#[test]
fn test_pop_max_empty() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    assert_eq!(pop_max(&mut heap), None);
}

#[test]
fn test_pop_max_with_duplicates() {
    let mut heap = create_max_heap(&[5, 5, 3, 3, 1]);
    assert_eq!(pop_max(&mut heap), Some(5));
    assert_eq!(pop_max(&mut heap), Some(5));
    assert_eq!(pop_max(&mut heap), Some(3));
    assert_eq!(pop_max(&mut heap), Some(3));
    assert_eq!(pop_max(&mut heap), Some(1));
}

#[test]
fn test_pop_max_negative() {
    let mut heap = create_max_heap(&[-10, -5, -1]);
    assert_eq!(pop_max(&mut heap), Some(-1));
    assert_eq!(pop_max(&mut heap), Some(-5));
    assert_eq!(pop_max(&mut heap), Some(-10));
}

// ==================== peek_max tests ====================

#[test]
fn test_peek_max_basic() {
    let heap = create_max_heap(&[10, 5, 20, 15]);
    assert_eq!(peek_max(&heap), Some(&20));
    assert_eq!(heap.len(), 4); // Unchanged
}

#[test]
fn test_peek_max_empty() {
    let heap: BinaryHeap<i32> = BinaryHeap::new();
    assert_eq!(peek_max(&heap), None);
}

#[test]
fn test_peek_max_single() {
    let heap = create_max_heap(&[42]);
    assert_eq!(peek_max(&heap), Some(&42));
}

#[test]
fn test_peek_max_multiple_calls() {
    let heap = create_max_heap(&[1, 2, 3]);
    assert_eq!(peek_max(&heap), Some(&3));
    assert_eq!(peek_max(&heap), Some(&3));
    assert_eq!(peek_max(&heap), Some(&3));
    assert_eq!(heap.len(), 3);
}

// ==================== top_k_largest tests ====================

#[test]
fn test_top_k_largest_basic() {
    assert_eq!(top_k_largest(&[3, 1, 4, 1, 5, 9, 2, 6], 3), vec![9, 6, 5]);
}

#[test]
fn test_top_k_largest_all() {
    assert_eq!(top_k_largest(&[3, 1, 4], 3), vec![4, 3, 1]);
}

#[test]
fn test_top_k_largest_k_greater_than_len() {
    assert_eq!(top_k_largest(&[5, 2, 8], 10), vec![8, 5, 2]);
}

#[test]
fn test_top_k_largest_k_zero() {
    let result = top_k_largest(&[1, 2, 3], 0);
    assert!(result.is_empty());
}

#[test]
fn test_top_k_largest_empty_input() {
    let result = top_k_largest(&[], 5);
    assert!(result.is_empty());
}

#[test]
fn test_top_k_largest_with_duplicates() {
    assert_eq!(top_k_largest(&[5, 5, 3, 3, 1], 3), vec![5, 5, 3]);
}

#[test]
fn test_top_k_largest_negative() {
    assert_eq!(top_k_largest(&[-1, -5, -2, -10], 2), vec![-1, -2]);
}

#[test]
fn test_top_k_largest_one() {
    assert_eq!(top_k_largest(&[3, 1, 4, 1, 5], 1), vec![5]);
}

// ==================== top_k_smallest tests ====================

#[test]
fn test_top_k_smallest_basic() {
    assert_eq!(top_k_smallest(&[3, 1, 4, 1, 5, 9, 2, 6], 3), vec![1, 1, 2]);
}

#[test]
fn test_top_k_smallest_all() {
    assert_eq!(top_k_smallest(&[3, 1, 4], 3), vec![1, 3, 4]);
}

#[test]
fn test_top_k_smallest_k_greater_than_len() {
    assert_eq!(top_k_smallest(&[5, 2, 8], 10), vec![2, 5, 8]);
}

#[test]
fn test_top_k_smallest_k_zero() {
    let result = top_k_smallest(&[1, 2, 3], 0);
    assert!(result.is_empty());
}

#[test]
fn test_top_k_smallest_empty_input() {
    let result = top_k_smallest(&[], 5);
    assert!(result.is_empty());
}

#[test]
fn test_top_k_smallest_with_duplicates() {
    assert_eq!(top_k_smallest(&[5, 5, 3, 3, 1], 3), vec![1, 3, 3]);
}

#[test]
fn test_top_k_smallest_negative() {
    assert_eq!(top_k_smallest(&[-1, -5, -2, -10], 2), vec![-10, -5]);
}

#[test]
fn test_top_k_smallest_one() {
    assert_eq!(top_k_smallest(&[3, 1, 4, 1, 5], 1), vec![1]);
}

// ==================== merge_heaps tests ====================

#[test]
fn test_merge_heaps_basic() {
    let heap1 = create_max_heap(&[1, 3, 5]);
    let heap2 = create_max_heap(&[2, 4, 6]);
    let merged = merge_heaps(heap1, heap2);
    assert_eq!(merged.len(), 6);
    assert_eq!(merged.peek(), Some(&6));
}

#[test]
fn test_merge_heaps_empty_first() {
    let heap1: BinaryHeap<i32> = BinaryHeap::new();
    let heap2 = create_max_heap(&[1, 2, 3]);
    let merged = merge_heaps(heap1, heap2);
    assert_eq!(merged.len(), 3);
    assert_eq!(merged.peek(), Some(&3));
}

#[test]
fn test_merge_heaps_empty_second() {
    let heap1 = create_max_heap(&[1, 2, 3]);
    let heap2: BinaryHeap<i32> = BinaryHeap::new();
    let merged = merge_heaps(heap1, heap2);
    assert_eq!(merged.len(), 3);
    assert_eq!(merged.peek(), Some(&3));
}

#[test]
fn test_merge_heaps_both_empty() {
    let heap1: BinaryHeap<i32> = BinaryHeap::new();
    let heap2: BinaryHeap<i32> = BinaryHeap::new();
    let merged = merge_heaps(heap1, heap2);
    assert!(merged.is_empty());
}

#[test]
fn test_merge_heaps_with_duplicates() {
    let heap1 = create_max_heap(&[1, 2, 3]);
    let heap2 = create_max_heap(&[2, 3, 4]);
    let merged = merge_heaps(heap1, heap2);
    assert_eq!(merged.len(), 6);
    assert_eq!(merged.peek(), Some(&4));
}

#[test]
fn test_merge_heaps_negative() {
    let heap1 = create_max_heap(&[-5, -3, -1]);
    let heap2 = create_max_heap(&[-4, -2, 0]);
    let merged = merge_heaps(heap1, heap2);
    assert_eq!(merged.len(), 6);
    assert_eq!(merged.peek(), Some(&0));
}

// ==================== heap_sort_descending tests ====================

#[test]
fn test_heap_sort_descending_basic() {
    assert_eq!(heap_sort_descending(&[3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
}

#[test]
fn test_heap_sort_descending_empty() {
    let result = heap_sort_descending(&[]);
    assert!(result.is_empty());
}

#[test]
fn test_heap_sort_descending_single() {
    assert_eq!(heap_sort_descending(&[42]), vec![42]);
}

#[test]
fn test_heap_sort_descending_already_sorted() {
    assert_eq!(heap_sort_descending(&[5, 4, 3, 2, 1]), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_heap_sort_descending_reverse_sorted() {
    assert_eq!(heap_sort_descending(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_heap_sort_descending_all_same() {
    assert_eq!(heap_sort_descending(&[7, 7, 7, 7]), vec![7, 7, 7, 7]);
}

#[test]
fn test_heap_sort_descending_negative() {
    assert_eq!(
        heap_sort_descending(&[-3, -1, -4, -1, -5]),
        vec![-1, -1, -3, -4, -5]
    );
}

#[test]
fn test_heap_sort_descending_mixed() {
    assert_eq!(
        heap_sort_descending(&[-2, 0, 3, -1, 2]),
        vec![3, 2, 0, -1, -2]
    );
}

// ==================== Integration tests ====================

#[test]
fn test_integration_task_scheduler() {
    // Simulate a priority task scheduler
    // Higher numbers = higher priority
    let tasks = vec![
        (3, "Low priority task"),
        (10, "Urgent task"),
        (5, "Medium priority task"),
        (1, "Background task"),
        (8, "High priority task"),
    ];

    let mut scheduler: BinaryHeap<(i32, &str)> = tasks.into_iter().collect();

    // Process in priority order
    let mut processed = Vec::new();
    while let Some((priority, task)) = scheduler.pop() {
        processed.push((priority, task));
    }

    assert_eq!(processed[0], (10, "Urgent task"));
    assert_eq!(processed[1], (8, "High priority task"));
    assert_eq!(processed[2], (5, "Medium priority task"));
    assert_eq!(processed[3], (3, "Low priority task"));
    assert_eq!(processed[4], (1, "Background task"));
}

#[test]
fn test_integration_find_median_elements() {
    // Use heaps to find elements around the median
    let data = vec![10, 5, 20, 15, 25, 30, 35];

    // Get top half (larger elements)
    let top_half = top_k_largest(&data, 4);
    assert_eq!(top_half, vec![35, 30, 25, 20]);

    // Get bottom half (smaller elements)
    let bottom_half = top_k_smallest(&data, 4);
    assert_eq!(bottom_half, vec![5, 10, 15, 20]);
}

#[test]
fn test_integration_streaming_top_k() {
    // Simulate processing a stream of numbers
    let stream = vec![5, 15, 10, 20, 8, 25, 3, 18, 12, 30];

    // At each point, get top 3
    let mut current_data: Vec<i32> = Vec::new();
    let mut snapshots = Vec::new();

    for num in stream {
        current_data.push(num);
        snapshots.push(top_k_largest(&current_data, 3));
    }

    // After [5], top 3 = [5]
    assert_eq!(snapshots[0], vec![5]);
    // After [5, 15], top 3 = [15, 5]
    assert_eq!(snapshots[1], vec![15, 5]);
    // After [5, 15, 10], top 3 = [15, 10, 5]
    assert_eq!(snapshots[2], vec![15, 10, 5]);
    // After all, top 3 = [30, 25, 20]
    assert_eq!(snapshots[9], vec![30, 25, 20]);
}

#[test]
fn test_integration_merge_multiple_heaps() {
    let heap1 = create_max_heap(&[1, 4, 7]);
    let heap2 = create_max_heap(&[2, 5, 8]);
    let heap3 = create_max_heap(&[3, 6, 9]);

    let merged12 = merge_heaps(heap1, heap2);
    let merged_all = merge_heaps(merged12, heap3);

    assert_eq!(merged_all.len(), 9);
    assert_eq!(merged_all.peek(), Some(&9));

    // Verify all elements are present by sorting
    let sorted: Vec<i32> = merged_all.into_sorted_vec();
    assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
