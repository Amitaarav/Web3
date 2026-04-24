/*
  Problem 30: Trait with Default Method

  Define a trait Greet with a method fn name(&self) -> &str (no default)
  and a method fn greeting(&self) -> String with a default implementation
  that returns "Hello, <name>!". Implement Greet for a struct Person { name: String }.

  Run the tests for this problem with:
    cargo test --test trait_default_test
*/

pub trait Greet {
    fn name(&self) -> &str;

    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

pub struct Person {
    pub name: String,
}

impl Greet for Person {
    fn name(&self) -> &str {
        todo!()
    }
}
