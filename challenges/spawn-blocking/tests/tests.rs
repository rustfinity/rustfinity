use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use spawn_blocking::{checksum, checksum_all, checksum_one};
use tokio::time::sleep;

/// How long one `checksum` call blocks its thread.
const BLOCK: Duration = Duration::from_millis(30);

fn inputs(count: usize) -> Vec<String> {
    (0..count).map(|index| format!("payload-{index}")).collect()
}

#[tokio::test]
async fn one_checksum_matches_the_blocking_function() {
    assert_eq!(checksum_one("hello".to_string()).await, checksum("hello"));
}

#[tokio::test]
async fn the_empty_input_checksums_too() {
    assert_eq!(checksum_one(String::new()).await, checksum(""));
}

#[tokio::test]
async fn checksumming_nothing_gives_nothing() {
    assert_eq!(checksum_all(Vec::new()).await, Vec::<u64>::new());
}

#[tokio::test]
async fn every_result_keeps_its_input_position() {
    let items = inputs(4);
    let expected: Vec<u64> = items.iter().map(|data| checksum(data)).collect();

    assert_eq!(checksum_all(items).await, expected);
}

#[tokio::test]
async fn equal_inputs_produce_equal_results() {
    let results = checksum_all(vec!["same".to_string(), "same".to_string()]).await;

    assert_eq!(results[0], results[1]);
}

#[tokio::test(flavor = "multi_thread")]
async fn eight_payloads_do_not_cost_eight_blocks() {
    let start = Instant::now();
    checksum_all(inputs(8)).await;

    assert!(
        start.elapsed() < BLOCK * 4,
        "checksum_all took {:?}, which means the work ran one item at a time",
        start.elapsed()
    );
}

#[tokio::test]
async fn the_runtime_keeps_running_while_the_work_blocks() {
    let ticks = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&ticks);

    let ticker = tokio::spawn(async move {
        for _ in 0..20 {
            sleep(Duration::from_millis(5)).await;
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });

    checksum_all(inputs(8)).await;
    ticker.abort();

    assert!(
        ticks.load(Ordering::SeqCst) >= 3,
        "the timer only ticked {} times, so the blocking work was run on the runtime's own thread",
        ticks.load(Ordering::SeqCst)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn a_single_worker_thread_is_enough() {
    let items = inputs(6);
    let expected: Vec<u64> = items.iter().map(|data| checksum(data)).collect();

    let start = Instant::now();
    let results = checksum_all(items).await;

    assert_eq!(results, expected);
    assert!(
        start.elapsed() < BLOCK * 3,
        "one worker thread should still hand every payload to the blocking pool"
    );
}
