use std::time::Duration;
use tokio::time::sleep;

/// A user profile assembled from three separate lookups.
#[derive(Debug, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub posts: usize,
}

/// Pretend network lookup. Provided for you.
pub async fn load_name(id: u32) -> String {
    sleep(Duration::from_millis(40)).await;
    format!("user{id}")
}

/// Pretend network lookup. Provided for you.
pub async fn load_email(id: u32) -> String {
    sleep(Duration::from_millis(40)).await;
    format!("user{id}@example.com")
}

/// Pretend network lookup. Provided for you.
pub async fn load_posts(id: u32) -> usize {
    sleep(Duration::from_millis(40)).await;
    id as usize * 3
}

/// Pretend network lookup that can fail. Provided for you.
pub async fn load_quota(id: u32) -> Result<u32, String> {
    sleep(Duration::from_millis(40)).await;

    if id == 0 {
        Err("no quota for user 0".to_string())
    } else {
        Ok(id * 100)
    }
}

/// Pretend network lookup that can fail. Provided for you.
pub async fn load_rate_limit(id: u32) -> Result<u32, String> {
    sleep(Duration::from_millis(40)).await;

    if id > 1000 {
        Err("unknown user".to_string())
    } else {
        Ok(60)
    }
}

/// Loads all three fields concurrently.
///
/// `join!` polls every future on the current task. Nothing is spawned,
/// so the futures do not need to be `'static` or `Send`, and the whole
/// thing costs one task instead of three.
pub async fn load_profile(id: u32) -> Profile {
    let (name, email, posts) = tokio::join!(load_name(id), load_email(id), load_posts(id));

    Profile { name, email, posts }
}

/// Loads both limits concurrently, giving up as soon as one fails.
///
/// `try_join!` returns `Err` the moment any branch returns `Err`, and
/// drops the other futures where they stand.
pub async fn load_limits(id: u32) -> Result<(u32, u32), String> {
    tokio::try_join!(load_quota(id), load_rate_limit(id))
}

// Example usage
#[tokio::main]
pub async fn main() {
    println!("{:?}", load_profile(7).await);
    println!("{:?}", load_limits(7).await);
    println!("{:?}", load_limits(0).await);
}
