use first_async_function::{greet, greet_and_count};
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn greet_builds_a_greeting() {
    assert_eq!(greet("Ada").await, "Hello, Ada!");
    assert_eq!(greet("Grace").await, "Hello, Grace!");
}

#[tokio::test]
async fn greet_handles_an_empty_name() {
    assert_eq!(greet("").await, "Hello, !");
}

#[tokio::test]
async fn greet_and_count_greets_too() {
    let calls = AtomicUsize::new(0);

    assert_eq!(greet_and_count(&calls, "Ada").await, "Hello, Ada!");
}

#[tokio::test]
async fn greet_and_count_records_every_call() {
    let calls = AtomicUsize::new(0);

    for name in ["Ada", "Grace", "Alan"] {
        greet_and_count(&calls, name).await;
    }

    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn the_body_does_not_run_until_it_is_awaited() {
    let calls = AtomicUsize::new(0);

    let future = greet_and_count(&calls, "Ada");

    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "calling an async fn must not run its body"
    );

    let greeting = future.await;

    assert_eq!(greeting, "Hello, Ada!");
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "awaiting the future must run the body exactly once"
    );
}

#[tokio::test]
async fn a_future_that_is_never_awaited_does_nothing() {
    let calls = AtomicUsize::new(0);

    drop(greet_and_count(&calls, "Ada"));

    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
