#[path = "../src/hard/json_enum.rs"]
mod json_enum;
use json_enum::Value;

use std::collections::HashMap;

#[test]
fn test_null() {
    assert_eq!(Value::Null.to_json_string(), "null");
}

#[test]
fn test_bool() {
    assert_eq!(Value::Bool(true).to_json_string(), "true");
}

#[test]
fn test_number() {
    assert_eq!(Value::Number(42.5).to_json_string(), "42.5");
}

#[test]
fn test_array() {
    let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    assert_eq!(arr.to_json_string(), "[1, 2]");
}

#[test]
fn test_complex_object() {
    let mut map = HashMap::new();
    map.insert("active".to_string(), Value::Bool(true));
    let obj = Value::Object(map);
    assert_eq!(obj.to_json_string(), "{\"active\": true}");
}

#[test]
fn test_bool_false() {
    assert_eq!(Value::Bool(false).to_json_string(), "false");
}

#[test]
fn test_string_value() {
    assert_eq!(Value::String("hello".to_string()).to_json_string(), "\"hello\"");
}

#[test]
fn test_integer_number() {
    assert_eq!(Value::Number(42.0).to_json_string(), "42");
}

#[test]
fn test_empty_array() {
    assert_eq!(Value::Array(vec![]).to_json_string(), "[]");
}

#[test]
fn test_empty_object() {
    assert_eq!(Value::Object(HashMap::new()).to_json_string(), "{}");
}

#[test]
fn test_nested_array() {
    let arr = Value::Array(vec![Value::Null, Value::Bool(true)]);
    assert_eq!(arr.to_json_string(), "[null, true]");
}
