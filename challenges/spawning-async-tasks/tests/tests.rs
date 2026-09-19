use spawning_async_tasks::{checked_divide, run_all};
use std::time::{Duration, Instant};

#[tokio::test(start_paused = true)]
async fn results_come_back_in_input_order() {
    assert_eq!(run_all(vec![30, 20, 10]).await, vec![30, 20, 10]);
}

#[tokio::test(start_paused = true)]
async fn an_empty_input_gives_an_empty_result() {
    assert_eq!(run_all(vec![]).await, Vec::<u64>::new());
}

#[tokio::test(start_paused = true)]
async fn a_single_delay_works() {
    assert_eq!(run_all(vec![5]).await, vec![5]);
}

#[tokio::test(start_paused = true)]
async fn duplicate_delays_are_all_reported() {
    assert_eq!(run_all(vec![10, 10, 10, 10]).await, vec![10, 10, 10, 10]);
}

#[tokio::test(start_paused = true)]
async fn the_tasks_run_concurrently() {
    let start = Instant::now();

    run_all(vec![40, 40, 40, 40, 40]).await;

    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(100),
        "five 40ms tasks took {elapsed:?}: they ran one after another, \
         so spawn every task before awaiting any of them"
    );
}

#[tokio::test(start_paused = true)]
async fn a_slow_task_does_not_hold_back_the_others() {
    let start = Instant::now();

    assert_eq!(run_all(vec![100, 1, 1]).await, vec![100, 1, 1]);

    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(150),
        "the whole batch should take as long as its slowest task, not the sum"
    );
}

#[tokio::test]
async fn a_normal_division_succeeds() {
    assert_eq!(checked_divide(10, 2).await, Ok(5));
    assert_eq!(checked_divide(-9, 3).await, Ok(-3));
    assert_eq!(checked_divide(7, 2).await, Ok(3));
}

#[tokio::test]
async fn dividing_by_zero_is_reported_as_an_error() {
    assert_eq!(checked_divide(1, 0).await, Err("task panicked".to_string()));
}

#[tokio::test]
async fn the_runtime_survives_a_panicking_task() {
    assert!(checked_divide(1, 0).await.is_err());
    assert_eq!(checked_divide(8, 4).await, Ok(2));
}
