use std::collections::HashMap;

use crate::{repository::Repository, record::{Record, Types}, request::Request};

use super::ReturnValue;

pub fn hget(repo: &mut dyn Repository, req: &Request) -> ReturnValue {
    let key = &req.arguments()[0];
    let hash_key = &req.arguments()[1];
    if let Ok(record) = repo.get(key.to_string()) {
        match record.value {
            Types::HashMap(hash) => match hash.get(hash_key) {
                Some(s) => ReturnValue::StringRes(s.to_string()),
                None => ReturnValue::Nil,
            },
            _ => ReturnValue::Error("wrongtype".to_string()),
        }
    } else {
        ReturnValue::Nil
    }
}

pub fn hset(repo: &mut dyn Repository, req: &Request) -> ReturnValue {
    if req.arity() % 2 != 0 {
        return ReturnValue::Error("wrong number of arguments".to_string());
    };
    let key = &req.arguments()[0];
    let pairs = &req.arguments()[1..];
    if let Ok(mut record) = repo.get(key.to_string()) {
        match record.value {
            Types::HashMap(ref mut hash) => {
                for pair in pairs.chunks(2) {
                    hash.insert(pair[0].to_string(), pair[0].to_string());
                }
                repo.set(key.to_string(), record.clone());
                return ReturnValue::Nil;
            }
            _ => return ReturnValue::Error("WrongType".to_string()), // essa linha não deve retornar wrongtype, ela deve apenas setar um novo hash no repo com os valores informados
        }
    } else {
        let mut new_set = HashMap::new();
        for pair in pairs.chunks(2) {
            new_set.insert(pair[0].to_string(), pair[1].to_string());
        }
        repo.set(
            key.to_string(),
            Record {
                value: Types::HashMap(new_set),
            },
        );
        return ReturnValue::Nil;
    }
}