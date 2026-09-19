use std::time::Duration;

use joinset_concurrent_tasks::{first_n_to_finish, results_in_completion_order, with_limit};
use tokio::time::Instant;

const MS: Duration = Duration::from_millis(1);

#[tokio::test(start_paused = true)]
async fn no_ids_means_no_results() {
    assert_eq!(results_in_completion_order(Vec::new()).await, Vec::new());
    assert_eq!(first_n_to_finish(Vec::new(), 3).await, Vec::new());
    assert_eq!(with_limit(Vec::new(), 2).await, Vec::new());
}

#[tokio::test(start_paused = true)]
async fn one_id_gives_one_result() {
    assert_eq!(results_in_completion_order(vec![7]).await, vec![70]);
}

#[tokio::test(start_paused = true)]
async fn results_arrive_shortest_first() {
    assert_eq!(
        results_in_completion_order(vec![30, 10, 20]).await,
        vec![100, 200, 300]
    );
}

#[tokio::test(start_paused = true)]
async fn the_spawn_order_does_not_decide_the_result_order() {
    assert_eq!(
        results_in_completion_order(vec![50, 40, 30, 20, 10]).await,
        vec![100, 200, 300, 400, 500]
    );
}

#[tokio::test(start_paused = true)]
async fn everything_runs_at_once_so_the_slowest_sets_the_pace() {
    let start = Instant::now();
    results_in_completion_order(vec![10, 20, 30]).await;

    assert_eq!(start.elapsed(), MS * 30);
}

#[tokio::test(start_paused = true)]
async fn taking_two_gives_the_two_quickest() {
    assert_eq!(first_n_to_finish(vec![50, 10, 20], 2).await, vec![100, 200]);
}

#[tokio::test(start_paused = true)]
async fn the_stragglers_are_not_waited_for() {
    let start = Instant::now();
    first_n_to_finish(vec![10, 20, 900], 2).await;

    assert_eq!(start.elapsed(), MS * 20);
}

#[tokio::test(start_paused = true)]
async fn asking_for_more_than_exists_returns_everything() {
    assert_eq!(first_n_to_finish(vec![20, 10], 9).await, vec![100, 200]);
}

#[tokio::test(start_paused = true)]
async fn asking_for_none_costs_nothing() {
    let start = Instant::now();

    assert_eq!(first_n_to_finish(vec![10, 20], 0).await, Vec::new());
    assert_eq!(start.elapsed(), Duration::ZERO);
}

#[tokio::test(start_paused = true)]
async fn a_limit_of_one_runs_them_one_after_another() {
    let start = Instant::now();
    let results = with_limit(vec![20, 30, 40], 1).await;

    assert_eq!(results, vec![200, 300, 400]);
    assert_eq!(start.elapsed(), MS * 90);
}

#[tokio::test(start_paused = true)]
async fn a_limit_wider_than_the_work_runs_it_all_at_once() {
    let start = Instant::now();
    let results = with_limit(vec![30, 10, 20], 10).await;

    assert_eq!(results, vec![100, 200, 300]);
    assert_eq!(start.elapsed(), MS * 30);
}

#[tokio::test(start_paused = true)]
async fn a_finished_task_makes_room_for_the_next_one() {
    let start = Instant::now();
    let results = with_limit(vec![50, 10, 25, 5], 2).await;

    assert_eq!(results, vec![100, 250, 50, 500]);
    assert_eq!(start.elapsed(), MS * 50);
}

#[tokio::test(start_paused = true)]
async fn the_limit_holds_for_a_long_queue() {
    let start = Instant::now();
    let results = with_limit(vec![10, 20, 30, 40, 50, 60], 2).await;

    assert_eq!(results, vec![100, 200, 300, 400, 500, 600]);
    assert_eq!(start.elapsed(), MS * 120);
}
