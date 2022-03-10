use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Types {
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Debug, Clone)]
pub struct Record {
    pub value: Types,
}
