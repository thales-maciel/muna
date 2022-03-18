use std::{collections::HashMap, time::Instant};

use crate::record::Record;

pub struct Repository {
    store: HashMap<String, Record>,
    expires: HashMap<String, Instant>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            expires: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, record: Record) -> Result<(), String> {
        match self.store.insert(key, record.clone()) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    pub fn get(&mut self, key: String) -> Option<Record> {
        if self.is_expired(key.to_string()) {
            self.delete(key);
            return None
        }

        self.store.get(&key).map(|r| r.clone())
    }

    fn delete(&mut self, key: String) -> Option<Record> {
        self.expires.remove(&key);
        self.store.remove(&key)
    }

    fn clear(&mut self) {
        self.store.clear();
        self.expires.clear();

        self.store.shrink_to_fit();
        self.expires.shrink_to_fit();
    }

    fn is_expired(&mut self, key: String) -> bool {
        match self.expires.get(&key.to_string()).copied() {
            Some(expiration) => Instant::now() > expiration,
            _ => false,
        }
    }

    fn set_expiration(&mut self, key: String, time: Instant) -> Result<(), String> {
        if let Some(record) = self.get(key.to_string()) {
            self.expires.insert(key, time);
        };
        Ok(())
    }
}
