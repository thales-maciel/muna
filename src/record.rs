use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Record {
    String(String),
    HashMap(HashMap<String, String>),
}
