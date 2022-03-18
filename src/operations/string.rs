use crate::{repository::Repository, record::{Record}, request::Request};

use super::OperationResult;

pub fn get(repo: &mut Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    if let Some(record) = repo.get(key.to_string()) {
        match record {
            Record::String(s) => OperationResult::StringRes(s),
            _ => OperationResult::Error("wrongtype".to_string()),
        }
    } else {
        OperationResult::Nil
    }
}

pub fn set(repo: &mut Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    let val = &req.arguments()[1];
    let record = Record::String(val.to_string());

    repo.set(key.to_string(), record);
    OperationResult::Ok
}