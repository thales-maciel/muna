use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Record {
    String(String),
    HashMap(HashMap<String, String>),
}
