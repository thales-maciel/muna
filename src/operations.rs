use std::collections::HashMap;

use crate::{
    record::{get_string, Record, Types, self},
    repository::Repository,
    request::Request,
};

type OperationHandler =
    fn(repo: &mut dyn Repository, request: Request) -> Result<ReturnValue, String>;

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnValue {
    Ok,
    StringRes(String),
    Error(String),
    Nil,
}

pub struct Operation {
    pub name: &'static str,
    pub handler: OperationHandler,
    pub arity: i32,
}

impl Operation {
    pub fn execute(
        &self,
        repo: &mut dyn Repository,
        request: Request,
    ) -> Result<ReturnValue, String> {
        if !is_valid_arity(self.arity.into(), request.arity()) {
            return Err("Wrong number of arguments".to_string());
        }
        (self.handler)(repo, request)
    }
}
pub fn get_string_handler(repo: &mut dyn Repository, req: Request) -> Result<ReturnValue, String> {
    let key = &req.arguments()[0];
    if let Ok(record) = repo.get(key.to_string()) {
        match record.value {
            Types::String(s) => Ok(ReturnValue::StringRes(s)),
            _ => Err("wrongtype".to_string()),
        }
    } else {
        Ok(ReturnValue::Nil)
    }
}

pub fn set_string_handler(repo: &mut dyn Repository, req: Request) -> Result<ReturnValue, String> {
    let key = &req.arguments()[0];
    let val = &req.arguments()[1];
    let record = Record {
        value: Types::String(val.to_string()),
    };

    match repo.set(key.to_string(), record) {
        Ok(_) => Ok(ReturnValue::Ok),
        Err(e) => Err(e),
    }
}

pub fn get_hash_handler(repo: &mut dyn Repository, req: Request) -> Result<ReturnValue, String> {
    let key = &req.arguments()[0];
    let hash_key = &req.arguments()[1];
    if let Ok(record) = repo.get(key.to_string()) {
        match record.value {
            Types::HashMap(hash) => match hash.get(hash_key) {
                Some(s) => Ok(ReturnValue::StringRes(s.to_string())),
                None => Ok(ReturnValue::Nil),
            },
            _ => Err("wrongtype".to_string()),
        }
    } else {
        Ok(ReturnValue::Nil)
    }
}

pub fn set_hash_handler(repo: &mut dyn Repository, req: Request) -> Result<ReturnValue, String> {
    if req.arity() % 2 != 0 {
        return Err("wrong number of arguments".to_string());
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
                return Ok(ReturnValue::Nil);
            }
            _ => return Err("WrongType".to_string()),
        }
    } else {
        let mut new_set = HashMap::new();
        for pair in pairs.chunks(2) {
            new_set.insert(pair[0].to_string(), pair[1].to_string());
        }
        repo.set(
            key.to_string(),
            Record {
                value: record::Types::HashMap(new_set),
            },
        );
        return Ok(ReturnValue::Nil);
    }
}

fn is_valid_arity(op_arity: i64, req_arity: i64) -> bool {
    op_arity == req_arity || (op_arity < 0 && req_arity >= op_arity.abs())
}

static OPERATIONS: &[Operation] = &[
    Operation {
        name: "get",
        handler: get_string_handler,
        arity: 2,
    },
    Operation {
        name: "set",
        handler: get_string_handler,
        arity: -3,
    },
    Operation {
        name: "hget",
        handler: get_string_handler,
        arity: 3,
    },
    Operation {
        name: "hset",
        handler: get_string_handler,
        arity: -4,
    },
    Operation {
        name: "command",
        handler: get_string_handler,
        arity: 2,
    },
];
