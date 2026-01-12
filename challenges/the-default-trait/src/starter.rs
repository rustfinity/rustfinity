#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
}

// TODO: implement the `Default` trait for `AppConfig`

// Example usage
pub fn main() {
    // Create a default configuration
    let default_config = AppConfig::default();
    println!("Default Config: {:?}", default_config);

    // Create a custom configuration using ..Default::default()
    let custom_config = AppConfig {
        theme: String::from("Dark"),
        ..Default::default()
    };
    println!("Custom Config: {:?}", custom_config);
}
