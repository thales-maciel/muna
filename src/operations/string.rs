use crate::{repository::Repository, record::{Record, Types}, request::Request};

use super::OperationResult;

pub fn get(repo: &mut dyn Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    if let Ok(record) = repo.get(key.to_string()) {
        match record.value {
            Types::String(s) => OperationResult::StringRes(s),
            _ => OperationResult::Error("wrongtype".to_string()),
        }
    } else {
        OperationResult::Nil
    }
}

pub fn set(repo: &mut dyn Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    let val = &req.arguments()[1];
    let record = Record {
        value: Types::String(val.to_string()),
    };

    match repo.set(key.to_string(), record) {
        Ok(_) => OperationResult::Ok,
        Err(e) => OperationResult::Error(e),
    }
}