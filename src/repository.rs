use std::collections::HashMap;

use crate::record::{Record};

pub trait Repository {
    fn set(&mut self, key: String, record: Record) -> Result<(), String>;
    fn get(&mut self, key: String) -> Result<Record, String>;
    fn delete(&mut self, key: String) -> Result<(), String>;
    fn clear(&mut self) -> Result<(), String>;
}
pub struct MemoryRepository {
    store: HashMap<String, Record>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Repository for MemoryRepository {
    fn set(&mut self, key: String, record: Record) -> Result<(), String> {
        match self.store.insert(key, record.clone()) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    fn get(&mut self, key: String) -> Result<Record, String> {
        match self.store.get(&key) {
            Some(r) => Ok(r.clone()),
            None => Err("No value found".to_string()),
        }
    }

    fn delete(&mut self, key: String) -> Result<(), String> {
        match self.store.remove_entry(&key) {
            Some(_) => Ok(()),
            None => Err("Nothing to delete".to_string()),
        }
    }

    fn clear(&mut self) -> Result<(), String> {
        self.store = HashMap::<String, Record>::new();
        Ok(())
    }
}
