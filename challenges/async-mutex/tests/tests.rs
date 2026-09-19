use std::sync::Arc;
use std::time::Duration;

use async_mutex::{load_value, Cache};
use tokio::time::Instant;

const LOAD_TIME: Duration = Duration::from_millis(10);

#[tokio::test(start_paused = true)]
async fn a_fresh_cache_has_loaded_nothing() {
    let cache = Cache::new();

    assert_eq!(cache.loads(), 0);
    assert_eq!(cache.snapshot().await, Vec::new());
}

#[tokio::test(start_paused = true)]
async fn a_miss_returns_what_the_loader_produced() {
    let cache = Cache::new();

    assert_eq!(
        cache.get_or_load("config").await,
        load_value("config").await
    );
    assert_eq!(cache.loads(), 1);
}

#[tokio::test(start_paused = true)]
async fn a_second_look_at_the_same_key_does_not_load_again() {
    let cache = Cache::new();

    let first = cache.get_or_load("config").await;
    let second = cache.get_or_load("config").await;

    assert_eq!(first, second);
    assert_eq!(cache.loads(), 1);
}

#[tokio::test(start_paused = true)]
async fn a_hit_costs_no_time_at_all() {
    let cache = Cache::new();
    cache.get_or_load("config").await;

    let start = Instant::now();
    cache.get_or_load("config").await;

    assert!(start.elapsed() < LOAD_TIME);
}

#[tokio::test(start_paused = true)]
async fn different_keys_load_separately() {
    let cache = Cache::new();

    cache.get_or_load("a").await;
    cache.get_or_load("b").await;
    cache.get_or_load("a").await;

    assert_eq!(cache.loads(), 2);
}

#[tokio::test(start_paused = true)]
async fn the_snapshot_is_sorted_by_key() {
    let cache = Cache::new();

    for key in ["pear", "apple", "quince"] {
        cache.get_or_load(key).await;
    }

    let keys: Vec<String> = cache
        .snapshot()
        .await
        .into_iter()
        .map(|(key, _)| key)
        .collect();

    assert_eq!(keys, vec!["apple", "pear", "quince"]);
}

#[tokio::test(start_paused = true)]
async fn the_snapshot_pairs_every_key_with_its_value() {
    let cache = Cache::new();
    cache.get_or_load("config").await;

    assert_eq!(
        cache.snapshot().await,
        vec![("config".to_string(), load_value("config").await)]
    );
}

#[tokio::test(start_paused = true)]
async fn eight_tasks_racing_for_one_key_load_it_once() {
    let cache = Arc::new(Cache::new());
    let mut handles = Vec::new();

    for _ in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(
            async move { cache.get_or_load("config").await },
        ));
    }

    let expected = load_value("config").await;

    for handle in handles {
        assert_eq!(handle.await.unwrap(), expected);
    }

    assert_eq!(cache.loads(), 1);
}

#[tokio::test(start_paused = true)]
async fn racing_tasks_across_four_keys_load_four_times() {
    let cache = Arc::new(Cache::new());
    let mut handles = Vec::new();

    for index in 0..16 {
        let cache = Arc::clone(&cache);
        let key = format!("key-{}", index % 4);
        handles.push(tokio::spawn(async move { cache.get_or_load(&key).await }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(cache.loads(), 4);
    assert_eq!(cache.snapshot().await.len(), 4);
}

#[tokio::test(start_paused = true)]
async fn a_cache_can_be_shared_across_threads() {
    fn assert_shareable<T: Send + Sync + 'static>() {}

    assert_shareable::<Cache>();
}
