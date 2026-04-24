/*
  Problem 21: Enum Calculator

  Define an enum Operation with variants Add(f64, f64), Sub(f64, f64), Mul(f64, f64), and Div(f64, f64).
  Write a function that takes an Operation and returns Result<f64, String>,
  returning an error for division by zero.

  Run the tests for this problem with:
    cargo test --test enum_calculator_test
*/

pub enum Operation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

pub fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Sub(a, b) => Ok(a - b),
        Operation::Mul(a, b) => Ok(a * b),
        Operation::Div(a, b) => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            }else{
                Ok( a / b)
            }
        }
    }
}
