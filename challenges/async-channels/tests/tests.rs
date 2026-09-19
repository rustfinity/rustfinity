use async_channels::{drain, produce_and_sum};
use tokio::sync::mpsc;

#[tokio::test]
async fn everything_sent_comes_back_in_order() {
    let (tx, rx) = mpsc::channel(8);

    for value in [3, 1, 4, 1, 5] {
        tx.send(value).await.unwrap();
    }
    drop(tx);

    assert_eq!(drain(rx).await, vec![3, 1, 4, 1, 5]);
}

#[tokio::test]
async fn a_closed_empty_channel_drains_to_nothing() {
    let (tx, rx) = mpsc::channel::<i32>(4);
    drop(tx);

    assert_eq!(drain(rx).await, Vec::<i32>::new());
}

#[tokio::test]
async fn draining_waits_for_a_slow_producer() {
    let (tx, rx) = mpsc::channel(1);

    tokio::spawn(async move {
        for value in 0..50 {
            tx.send(value).await.unwrap();
        }
    });

    assert_eq!(drain(rx).await, (0..50).collect::<Vec<_>>());
}

#[tokio::test]
async fn every_sender_has_to_go_before_draining_ends() {
    let (tx, rx) = mpsc::channel(8);
    let second = tx.clone();

    tx.send(1).await.unwrap();
    second.send(2).await.unwrap();

    drop(tx);
    drop(second);

    assert_eq!(drain(rx).await, vec![1, 2]);
}

#[tokio::test]
async fn a_small_run_adds_up() {
    assert_eq!(produce_and_sum(5, 4).await, 10);
}

#[tokio::test]
async fn sending_nothing_sums_to_zero() {
    assert_eq!(produce_and_sum(0, 4).await, 0);
}

#[tokio::test]
async fn a_capacity_of_one_still_delivers_everything() {
    assert_eq!(produce_and_sum(500, 1).await, 124_750);
}

#[tokio::test]
async fn a_roomy_channel_delivers_everything_too() {
    assert_eq!(produce_and_sum(500, 512).await, 124_750);
}

#[tokio::test]
async fn a_rendezvous_sized_channel_is_not_a_deadlock() {
    assert_eq!(produce_and_sum(1000, 3).await, 499_500);
}

#[tokio::test]
async fn one_item_arrives_intact() {
    assert_eq!(produce_and_sum(1, 1).await, 0);
    assert_eq!(produce_and_sum(2, 1).await, 1);
}
