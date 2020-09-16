use json::JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingType {
    pub name: String,
    pub params: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub settings: HashMap<String, SettingType>,
    pub inputs: HashMap<String, Type>,
    pub outputs: HashMap<String, Type>,
}
