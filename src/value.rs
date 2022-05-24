use std::any::Any;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Num(Num),
    String(String),
    Array(Vec<Value>),
}

#[derive(Debug, Clone)]
pub enum Num {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn new<T: Any>(value: T) -> Value {
        let any_val = &value as &dyn Any;
        if let Some(val) = any_val.downcast_ref::<bool>() {
            Value::Bool(*val)
        } else if let Some(val) = any_val.downcast_ref::<i64>() {
            Value::Num(Num::I64(*val))
        } else if let Some(val) = any_val.downcast_ref::<u64>() {
            Value::Num(Num::U64(*val))
        } else if let Some(val) = any_val.downcast_ref::<f64>() {
            Value::Num(Num::F64(*val))
        } else if let Some(val) = any_val.downcast_ref::<&'static str>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<String>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<Vec<Value>>() {
            Value::Array(val.to_vec())
        } else {
            Value::Null
        }
    }

    pub fn bool(&self) -> Option<bool> {
        if let Value::Bool(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn i64(&self) -> Option<i64> {
        if let Value::Num(Num::I64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn u64(&self) -> Option<u64> {
        if let Value::Num(Num::U64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn f64(&self) -> Option<f64> {
        if let Value::Num(Num::F64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }
    
    pub fn string(&self) -> Option<String> {
        if let Value::String(val) = self {
            Some(val.to_string())
        } else {
            None
        }
    }
}
