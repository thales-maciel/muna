use std::collections::HashMap;

use crate::{repository::Repository, record::{Record}, request::Request};

use super::OperationResult;

pub fn hget(repo: &mut Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    let hash_key = &req.arguments()[1];
    if let Some(record) = repo.get(key.to_string()) {
        match record {
            Record::HashMap(hash) => match hash.get(hash_key) {
                Some(s) => OperationResult::StringRes(s.to_string()),
                None => OperationResult::Nil,
            },
            _ => OperationResult::Error("wrongtype".to_string()),
        }
    } else {
        OperationResult::Nil
    }
}

pub fn hset(repo: &mut Repository, req: &Request) -> OperationResult {
    if req.arity() % 2 != 0 {
        return OperationResult::Error("wrong number of arguments".to_string());
    };
    let key = &req.arguments()[0];
    let pairs = &req.arguments()[1..];
    if let Some(mut record) = repo.get(key.to_string()) {
        match record {
            Record::HashMap(ref mut hash) => {
                for pair in pairs.chunks(2) {
                    hash.insert(pair[0].to_string(), pair[0].to_string());
                }
                repo.set(key.to_string(), record.clone());
                return OperationResult::Nil;
            }
            _ => {
                let record = new_hash_from_pairs(pairs);
                repo.set(key.to_string(), record);
                OperationResult::Nil
            }
        }
    } else {
        let record = new_hash_from_pairs(pairs);
        repo.set(key.to_string(), record);
        OperationResult::Nil
    }
}

fn new_hash_from_pairs(pairs: &[String]) -> Record {
    let mut new_set = HashMap::new();
    for pair in pairs.chunks(2) {
        new_set.insert(pair[0].to_string(), pair[1].to_string());
    }
    let record = Record::HashMap(new_set);
    record
}