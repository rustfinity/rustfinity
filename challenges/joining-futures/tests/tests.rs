use joining_futures::{load_limits, load_profile, Profile};
use std::time::{Duration, Instant};

#[tokio::test(start_paused = true)]
async fn a_profile_has_every_field_filled_in() {
    assert_eq!(
        load_profile(7).await,
        Profile {
            name: "user7".to_string(),
            email: "user7@example.com".to_string(),
            posts: 21,
        }
    );
}

#[tokio::test(start_paused = true)]
async fn a_different_id_gives_different_data() {
    assert_eq!(
        load_profile(2).await,
        Profile {
            name: "user2".to_string(),
            email: "user2@example.com".to_string(),
            posts: 6,
        }
    );
}

#[tokio::test(start_paused = true)]
async fn the_three_lookups_overlap() {
    let start = Instant::now();

    load_profile(1).await;

    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(80),
        "three 40ms lookups took {elapsed:?}: they ran one after another. \
         Await them together instead of one at a time"
    );
}

#[tokio::test(start_paused = true)]
async fn limits_are_returned_in_order() {
    assert_eq!(load_limits(3).await, Ok((300, 60)));
    assert_eq!(load_limits(1).await, Ok((100, 60)));
}

#[tokio::test(start_paused = true)]
async fn a_failing_quota_fails_the_whole_call() {
    assert_eq!(load_limits(0).await, Err("no quota for user 0".to_string()));
}

#[tokio::test(start_paused = true)]
async fn a_failing_rate_limit_fails_the_whole_call() {
    assert_eq!(load_limits(5000).await, Err("unknown user".to_string()));
}

#[tokio::test(start_paused = true)]
async fn the_two_limit_lookups_overlap_too() {
    let start = Instant::now();

    let _ = load_limits(9).await;

    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(70),
        "two 40ms lookups took {elapsed:?}: run them concurrently"
    );
}
