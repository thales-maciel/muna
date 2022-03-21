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

    pub fn set(&mut self, key: String, record: Record) {
        self.store.insert(key, record.clone());
    }

    pub fn get(&mut self, key: String) -> Option<Record> {
        if self.is_expired(key.to_string()) {
            self.delete(key);
            return None
        }

        self.store.get(&key).map(|r| r.clone())
    }

    pub fn delete(&mut self, key: String) -> Option<Record> {
        self.expires.remove(&key);
        self.store.remove(&key)
    }

    pub fn clear(&mut self) {
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

    pub fn set_expiration(&mut self, key: String, time: Instant) {
        if let Some(_) = self.get(key.to_string()) {
            self.expires.insert(key, time);
        };
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::record::Record;

    #[test]
    fn test_set() {
        let mut repo = Repository::new();
        let key = String::from("x");
        let record = Record::String("abc".to_string());
        repo.set(key.clone(), record);

        assert_eq!(repo.store.len(), 1);
        assert_eq!(repo.store.get(&key).unwrap().to_owned(), Record::String("abc".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut repo = Repository::new();
        let expires_at = Instant::now() + Duration::from_secs(10);
        let key = String::from("x");
        let record = Record::String("abc".to_string());
        repo.set(key.clone(), record);
        repo.set_expiration(key.clone(), expires_at);

        assert_eq!(repo.store.len(), 1);
        assert_eq!(repo.expires.len(), 1);

        repo.clear();
        assert_eq!(repo.store.len(), 0);
        assert_eq!(repo.expires.len(), 0);
    }

    #[test]
    fn test_delete() {
        let mut repo = Repository::new();
        let expires_at = Instant::now() + Duration::from_secs(10);
        let key = String::from("x");
        let record = Record::String("abc".to_string());
        repo.set(key.clone(), record);
        repo.set_expiration(key.clone(), expires_at);

        assert_eq!(repo.store.len(), 1);
        assert_eq!(repo.expires.len(), 1);

        repo.delete(key);
        assert_eq!(repo.store.len(), 0);
        assert_eq!(repo.expires.len(), 0);
    }

    #[test]
    fn test_get_expired() {
        let mut repo = Repository::new();
        let expires_at = Instant::now() - Duration::from_secs(10);
        let key = String::from("x");
        let record = Record::String("abc".to_string());
        repo.set(key.clone(), record);
        repo.set_expiration(key.clone(), expires_at);

        assert_eq!(repo.get(key), None);

        assert_eq!(repo.store.len(), 0);
        assert_eq!(repo.expires.len(), 0);
    }
}
