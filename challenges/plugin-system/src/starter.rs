pub trait Plugin {
    // 1. Finish the trait
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
}

// 3. Implement the PluginManager
impl PluginManager {}


// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
