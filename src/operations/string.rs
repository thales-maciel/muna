use crate::{repository::Repository, record::{Record, Types}, request::Request};

use super::ReturnValue;

pub fn get(repo: &mut dyn Repository, req: &Request) -> ReturnValue {
    let key = &req.arguments()[0];
    if let Ok(record) = repo.get(key.to_string()) {
        match record.value {
            Types::String(s) => ReturnValue::StringRes(s),
            _ => ReturnValue::Error("wrongtype".to_string()),
        }
    } else {
        ReturnValue::Nil
    }
}

pub fn set(repo: &mut dyn Repository, req: &Request) -> ReturnValue {
    let key = &req.arguments()[0];
    let val = &req.arguments()[1];
    let record = Record {
        value: Types::String(val.to_string()),
    };

    match repo.set(key.to_string(), record) {
        Ok(_) => ReturnValue::Ok,
        Err(e) => ReturnValue::Error(e),
    }
}