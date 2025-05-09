pub fn get_database_url() -> String {
    // Your code here...
}

/// Example usage
pub fn main() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    std::env::remove_var("DATABASE_URL"); // Missing variable scenario
    get_database_url();

    std::env::set_var("DATABASE_URL", "mysql://localhost"); // Invalid prefix scenario
    get_database_url();
}
