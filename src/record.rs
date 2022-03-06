use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    TypeMismatch
}

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone)]
pub enum Types {
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Debug, Clone)]
pub struct Record {
    pub value: Types,
}

pub fn get_string(record: Record) -> Result<String> {
    match record.value {
        Types::String(s) => Ok(s),
        _ => Err("wrongtype".to_string()),
    }
}

pub fn get_hash_value(record: Record, key: String) -> Result<String> {
    match record.value {
        Types::HashMap(r) => {
            match r.get(&key) {
                Some(s) => Ok(s.to_string()),
                None => Err("empty".to_string()),
            }
        },
        _ => Err("wrongtype".to_string()),
    }
}

pub fn set_hash_value(record: Record, key: String, value: String) -> Result<()> {
    match record.value {
        Types::HashMap(mut r) => {
            r.insert(key, value);
            Ok(())
        },
        _ => Err("wrongtype".to_string()),
    }
}
