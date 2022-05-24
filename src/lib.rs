pub mod value;

use std::collections::HashMap;

use crate::value::Value;

pub type GenericMap = HashMap<String, Value>;

pub type StringMap = HashMap<String, String>;


pub trait FromMap: Default {
    fn from_map(hashmap: GenericMap) -> Self;
}

pub trait ToMap: Default {
    fn to_map(hashmap: Self) -> StringMap;
}
