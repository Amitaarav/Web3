/*
  Problem 65: Data Processing — JSON-like Enum

  Define an enum Value representing a simplified JSON value:
  Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>),
  and Object(HashMap<String, Value>). Implement a method fn to_json_string(&self) -> String
  that produces a JSON-formatted string.

  Run the tests for this problem with:
    cargo test --test json_enum_test
*/

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn to_json_string(&self) -> String {
        todo!()
    }
}
