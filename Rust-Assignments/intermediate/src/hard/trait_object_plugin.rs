/*
  Problem 63: Trait Objects — Plugin System

  Define a trait Plugin with a method name(&self) -> &str and run(&self, input: i32) -> i32.
  Implement it for AddOne and Double. Create a struct PluginManager that holds a
  Vec<Box<dyn Plugin>> and implement a method run_all(input: i32) -> i32
  that passes the result of one plugin as input to the next.

  Run the tests for this problem with:
    cargo test --test trait_object_plugin_test
*/

pub trait Plugin {
    fn name(&self) -> &str;
    fn run(&self, input: i32) -> i32;
}

pub struct AddOne;
pub struct Double;

impl Plugin for AddOne {
    fn name(&self) -> &str { todo!() }
    fn run(&self, input: i32) -> i32 { todo!() }
}

impl Plugin for Double {
    fn name(&self) -> &str { todo!() }
    fn run(&self, input: i32) -> i32 { todo!() }
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, plugin: Box<dyn Plugin>) {
        todo!()
    }

    pub fn run_all(&self, input: i32) -> i32 {
        todo!()
    }
}
