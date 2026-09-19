use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use racing_with_select::{collect_until_deadline, first_of};
use tokio::sync::mpsc;
use tokio::time::{sleep, Instant};

const TICK: Duration = Duration::from_millis(10);

async fn after(delay: Duration, label: &str) -> String {
    sleep(delay).await;
    label.to_string()
}

#[tokio::test(start_paused = true)]
async fn the_quicker_of_the_two_wins() {
    let winner = first_of(after(TICK * 5, "slow"), after(TICK, "quick")).await;

    assert_eq!(winner, "quick");
}

#[tokio::test(start_paused = true)]
async fn the_first_branch_can_win_too() {
    let winner = first_of(after(TICK, "quick"), after(TICK * 5, "slow")).await;

    assert_eq!(winner, "quick");
}

#[tokio::test(start_paused = true)]
async fn racing_takes_as_long_as_the_winner_not_the_loser() {
    let start = Instant::now();

    first_of(after(TICK * 50, "slow"), after(TICK, "quick")).await;

    assert!(start.elapsed() < TICK * 2);
}

#[tokio::test(start_paused = true)]
async fn the_losing_future_is_cancelled_where_it_stands() {
    static LOSER_FINISHED: AtomicUsize = AtomicUsize::new(0);

    let loser = async {
        sleep(TICK * 5).await;
        LOSER_FINISHED.fetch_add(1, Ordering::SeqCst);
        "slow".to_string()
    };

    assert_eq!(first_of(loser, after(TICK, "quick")).await, "quick");

    sleep(TICK * 20).await;
    assert_eq!(LOSER_FINISHED.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn a_ready_future_beats_anything_that_has_to_wait() {
    let winner = first_of(async { "instant".to_string() }, after(TICK, "quick")).await;

    assert_eq!(winner, "instant");
}

#[tokio::test(start_paused = true)]
async fn a_channel_that_closes_early_ends_the_collection() {
    let (tx, mut rx) = mpsc::channel(8);

    for value in [3, 1, 4] {
        tx.send(value).await.unwrap();
    }
    drop(tx);

    let start = Instant::now();
    let collected = collect_until_deadline(&mut rx, TICK * 100).await;

    assert_eq!(collected, vec![3, 1, 4]);
    assert!(start.elapsed() < TICK);
}

#[tokio::test(start_paused = true)]
async fn a_closed_empty_channel_collects_nothing() {
    let (tx, mut rx) = mpsc::channel::<i32>(4);
    drop(tx);

    assert_eq!(collect_until_deadline(&mut rx, TICK * 10).await, Vec::new());
}

#[tokio::test(start_paused = true)]
async fn the_deadline_cuts_a_producer_off_mid_stream() {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        for value in 0..100 {
            sleep(TICK).await;
            if tx.send(value).await.is_err() {
                break;
            }
        }
    });

    let collected = collect_until_deadline(&mut rx, TICK * 5 + TICK / 2).await;

    assert_eq!(collected, vec![0, 1, 2, 3, 4]);
}

#[tokio::test(start_paused = true)]
async fn a_steady_producer_does_not_push_the_deadline_back() {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        for value in 0..1000 {
            sleep(TICK).await;
            if tx.send(value).await.is_err() {
                break;
            }
        }
    });

    let start = Instant::now();
    let collected = collect_until_deadline(&mut rx, TICK * 3 + TICK / 2).await;

    assert_eq!(collected, vec![0, 1, 2]);
    assert!(start.elapsed() < TICK * 4);
}

#[tokio::test(start_paused = true)]
async fn a_deadline_that_is_up_before_any_message_collects_nothing() {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        sleep(TICK * 10).await;
        let _ = tx.send(1).await;
    });

    assert_eq!(collect_until_deadline(&mut rx, TICK).await, Vec::new());
}
