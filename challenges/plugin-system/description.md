We now have an overview of traits, how to define them and how they work, now it's time to put that knowledge to use and build a plugin system using traits.

A plugin in this system is any type that implements a specific trait. Each plugin will perform a specific task, and the system should manage a collection of these plugins, executing them in sequence. You’ll also address advanced issues like object safety and resolving potential conflicts between overlapping trait implementations.

## Your Task

Design and implement a plugin system using trait objects. You will:

1. Define a `Plugin` trait that includes methods for initialization and execution.
2. Create a `PluginManager` struct to manage plugins. It should:
   - Dynamically load plugins implementing the `Plugin` trait.
   - Allow adding and removing plugins.
   - Execute all registered plugins in sequence.

### Requirements

- The `Plugin` trait should include the following methods:
  - `fn name(&self) -> &str;` - Returns the name of the plugin.
  - `fn execute(&self);` - Executes the plugin's functionality.
- The `PluginManager` should:
  - Have the following methods and associated functions:
    - `new() -> Self` - Creates a new `PluginManager` instance.
    - `add_plugin` - Adds a plugin to the list.
    - `remove_plugin` - Removes a plugin from the list and returns the removed plugin if found.
    - `execute_all` - Executes all registered plugins.
- If a duplicate plugin is added (with the same name), it should **panic** with the message "Plugin with name [name] already exists".

Make sure you make all relevant items public.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- **Dynamic Dispatch**: Store plugins in a `Vec<Box<dyn Plugin>>` for dynamic dispatch.
- **Plugin Uniqueness**: Use the `name` method to identify and ensure uniqueness among plugins.

</details>
