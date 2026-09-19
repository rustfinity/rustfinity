use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use timeouts_and_sleep::{retry, with_timeout};
use tokio::time::{sleep, Instant};

const TICK: Duration = Duration::from_millis(10);

#[tokio::test(start_paused = true)]
async fn work_that_finishes_in_time_hands_its_value_back() {
    let value = with_timeout(TICK * 10, async {
        sleep(TICK).await;
        "done"
    })
    .await;

    assert_eq!(value, Some("done"));
}

#[tokio::test(start_paused = true)]
async fn work_that_runs_long_times_out() {
    let value = with_timeout(TICK, async {
        sleep(TICK * 10).await;
        "done"
    })
    .await;

    assert_eq!(value, None);
}

#[tokio::test(start_paused = true)]
async fn an_already_finished_future_never_waits() {
    let start = Instant::now();

    assert_eq!(with_timeout(TICK * 100, async { 7 }).await, Some(7));
    assert!(start.elapsed() < TICK);
}

#[tokio::test(start_paused = true)]
async fn a_timeout_returns_as_soon_as_the_limit_is_up() {
    let start = Instant::now();

    with_timeout(TICK * 2, sleep(TICK * 100)).await;

    assert!(start.elapsed() < TICK * 3);
}

#[tokio::test(start_paused = true)]
async fn a_timed_out_future_is_dropped_before_it_finishes() {
    static REACHED_THE_END: AtomicUsize = AtomicUsize::new(0);

    with_timeout(TICK, async {
        sleep(TICK * 10).await;
        REACHED_THE_END.fetch_add(1, Ordering::SeqCst);
    })
    .await;

    assert_eq!(REACHED_THE_END.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn a_first_try_success_reports_attempt_zero() {
    let start = Instant::now();

    assert_eq!(retry(5, TICK, || true).await, Some(0));
    assert!(start.elapsed() < TICK);
}

#[tokio::test(start_paused = true)]
async fn a_later_success_reports_its_own_attempt_number() {
    let mut tries = 0;
    let found = retry(5, TICK, || {
        tries += 1;
        tries == 3
    })
    .await;

    assert_eq!(found, Some(2));
}

#[tokio::test(start_paused = true)]
async fn the_gap_is_only_ever_between_attempts() {
    let start = Instant::now();

    let mut tries = 0;
    retry(5, TICK, || {
        tries += 1;
        tries == 3
    })
    .await;

    assert_eq!(start.elapsed(), TICK * 2);
}

#[tokio::test(start_paused = true)]
async fn giving_up_costs_one_gap_less_than_the_attempt_count() {
    let start = Instant::now();

    assert_eq!(retry(4, TICK, || false).await, None);
    assert_eq!(start.elapsed(), TICK * 3);
}

#[tokio::test(start_paused = true)]
async fn zero_attempts_means_op_is_never_called() {
    let mut calls = 0;
    let found = retry(0, TICK, || {
        calls += 1;
        true
    })
    .await;

    assert_eq!(found, None);
    assert_eq!(calls, 0);
}

#[tokio::test(start_paused = true)]
async fn op_is_not_called_again_once_it_succeeds() {
    let mut calls = 0;
    retry(10, TICK, || {
        calls += 1;
        calls == 2
    })
    .await;

    assert_eq!(calls, 2);
}
