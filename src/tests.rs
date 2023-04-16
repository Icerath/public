use std::borrow::Cow;

use crate::{
    bytecode::{BinOp, Pool},
    value::Value,
    vm,
};

#[test]
pub fn test_ints() {
    let ints = [10, 15, 20, 25];

    let mut pool = Pool::default();
    for int in ints {
        pool.push_int(int);
    }

    let stack = vm::create_and_run(pool.as_bytes());
    let expected_stack: Vec<Value> = ints.into_iter().map(Value::Int).collect();
    assert_eq!(stack, expected_stack);
}

#[test]
pub fn test_floats() {
    let floats = [1.1, 0.2, 12.2, f64::MAX, f64::INFINITY, f64::NEG_INFINITY];

    let mut pool = Pool::default();
    for float in floats {
        pool.push_float(float);
    }

    let stack = vm::create_and_run(pool.as_bytes());
    let expected_stack: Vec<Value> = floats.into_iter().map(Value::Float).collect();
    assert_eq!(stack, expected_stack);
}

#[test]
pub fn test_strings() {
    let strings = ["", "Hello, World!", &"repeated ".repeat(100)];

    let mut pool = Pool::default();
    for str in strings {
        pool.push_str(str);
    }

    let stack = vm::create_and_run(pool.as_bytes());

    let expected_stack: Vec<Value> = strings
        .into_iter()
        .map(|str| Value::Str(Cow::Borrowed(str)))
        .collect();

    assert_eq!(stack, expected_stack);
}

#[test]
pub fn test_int_add() {
    let mut pool = Pool::default();
    pool.push_int(1);
    pool.push_int(2);
    pool.push_binop(BinOp::Add);

    let stack = vm::create_and_run(pool.as_bytes());
    assert_eq!(stack, vec![Value::Int(3)]);
}

#[test]
pub fn test_float_add() {
    let mut pool = Pool::default();
    pool.push_float(1.23);
    pool.push_float(4.56);

    pool.push_binop(BinOp::Add);

    let stack = vm::create_and_run(pool.as_bytes());
    assert_eq!(stack, vec![Value::Float(1.23 + 4.56)]);
}

#[test]
pub fn test_int_float_add() {
    let mut pool = Pool::default();
    pool.push_int(1);
    pool.push_float(0.5);
    pool.push_binop(BinOp::Add);

    pool.push_float(12.5);
    pool.push_int(2);
    pool.push_binop(BinOp::Add);

    let stack = vm::create_and_run(pool.as_bytes());
    assert_eq!(
        stack,
        vec![Value::Float(1.0 + 0.5), Value::Float(12.5 + 2.0)]
    );
}

#[test]
pub fn test_str_add() {
    let mut pool = Pool::default();
    pool.push_str("Hello, ");
    pool.push_str("World!");
    pool.push_binop(BinOp::Add);

    let stack = vm::create_and_run(pool.as_bytes());
    assert_eq!(stack, vec![Value::Str(Cow::Owned("Hello, World!".into()))]);
}
