#[path = "../src/hard/trait_object_plugin.rs"]
mod trait_object_plugin;
use trait_object_plugin::{AddOne, Double, PluginManager};


#[test]
fn test_plugin_chain() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(AddOne)); // (x + 1)
    manager.add(Box::new(Double)); // (x + 1) * 2
    assert_eq!(manager.run_all(5), 12);
}

#[test]
fn test_empty_manager() {
    let manager = PluginManager::new();
    assert_eq!(manager.run_all(10), 10);
}

#[test]
fn test_plugin_names() {
    let add = AddOne;
    use intermediate::hard::trait_object_plugin::Plugin;
    assert_eq!(add.name(), "AddOne");
}

#[test]
fn test_double_name() {
    use intermediate::hard::trait_object_plugin::Plugin;
    let d = Double;
    assert_eq!(d.name(), "Double");
}

#[test]
fn test_add_one_only() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(AddOne));
    assert_eq!(manager.run_all(0), 1);
}

#[test]
fn test_double_only() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(Double));
    assert_eq!(manager.run_all(5), 10);
}

#[test]
fn test_empty_manager_zero() {
    let manager = PluginManager::new();
    assert_eq!(manager.run_all(0), 0);
}

#[test]
fn test_add_one_twice() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(AddOne));
    manager.add(Box::new(AddOne));
    assert_eq!(manager.run_all(0), 2);
}

#[test]
fn test_double_then_add_one() {
    let mut manager = PluginManager::new();
    manager.add(Box::new(Double));
    manager.add(Box::new(AddOne));
    assert_eq!(manager.run_all(3), 7);
}
