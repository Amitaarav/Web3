/*
  Problem 69: Macro — measure_time!

  Write a declarative macro measure_time! that takes an expression,
  executes it, prints the elapsed time using std::time::Instant,
  and returns the result of the expression.

  Run the tests for this problem with:
    cargo test --test macro_measure_test
*/

#[macro_export]
macro_rules! measure_time {
    ($e:expr) => {
        todo!()
    };
}
