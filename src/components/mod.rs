mod input_bool;
pub use input_bool::*;
mod input_string;
pub use input_string::*;
mod input_i64;
pub use input_i64::*;

#[derive(Debug, Clone)]
pub enum InputValue<T: Clone> {
    Value(T),
    InvalidValue,
}

impl<T: Clone> InputValue<T> {
    pub fn has_value(&self) -> bool {
        match self {
            InputValue::Value(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_value(&self) -> &T {
        match self {
            InputValue::Value(value) => value,
            _ => panic!("InputValue::unwrap_value called on non-Value variant"),
        }
    }
}
