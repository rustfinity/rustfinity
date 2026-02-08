pub const GITHUB_CHALLENGES_BASE_URL: &'static str =
    "https://raw.githubusercontent.com/dcodesdev/rustfinity.com/main/challenges";

pub const GITHUB_REPO_URL: &'static str = "https://www.github.com/dcodesdev/rustfinity.com";

const DEFAULT_API_BASE_URL: &str = "http://localhost:7970/api";

pub fn api_base_url() -> String {
    std::env::var("RUSTFINITY_API_URL").unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string())
}
