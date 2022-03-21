use crate::{repository::Repository, request::Request};

mod hash;
mod string;
mod key;
mod server;

use self::{
    hash::{hget, hset},
    string::{get, set},
    key::expire,
    server::flush_all
};

type OperationHandler = fn(repo: &mut Repository, request: &Request) -> OperationResult;

#[derive(Debug, PartialEq, Clone)]
pub enum OperationResult {
    Ok,
    StringRes(String),
    Error(String),
    Int(i64),
    Nil,
}

pub struct Operation {
    pub name: &'static str,
    pub handler: OperationHandler,
    pub arity: i32,
}

impl Operation {
    pub fn execute(&self, repo: &mut Repository, request: &Request) -> OperationResult {
        if !is_valid_arity(self.arity.into(), request.arity()) {
            return OperationResult::Error("Wrong number of arguments".to_string());
        }
        (self.handler)(repo, request)
    }
}

pub fn commands_handler(_: &mut Repository, _: &Request) -> OperationResult {
    OperationResult::Ok
}

fn is_valid_arity(op_arity: i64, req_arity: i64) -> bool {
    op_arity == req_arity || (op_arity < 0 && req_arity >= op_arity.abs())
}

static OPERATIONS: &[Operation] = &[
    Operation {
        name: "get",
        handler: get,
        arity: 2,
    },
    Operation {
        name: "set",
        handler: set,
        arity: -3,
    },
    Operation {
        name: "hget",
        handler: hget,
        arity: 3,
    },
    Operation {
        name: "hset",
        handler: hset,
        arity: -4,
    },
    Operation {
        name: "command",
        handler: commands_handler,
        arity: -1,
    },
    Operation {
        name: "expire",
        handler: expire,
        arity: 3
    },
    Operation {
        name: "flushall",
        handler: flush_all,
        arity: 1
    },
];

pub fn lookup(name: &str) -> Option<&Operation> {
    OPERATIONS
        .iter()
        .find(|o| name.eq_ignore_ascii_case(o.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_arity() {
        assert_eq!(is_valid_arity(2, 2), true);
        assert_eq!(is_valid_arity(-2, 2), true);
        assert_eq!(is_valid_arity(-2, 3), true);

        assert_eq!(is_valid_arity(2, 3), false);
        assert_eq!(is_valid_arity(2, 1), false);
        assert_eq!(is_valid_arity(-2, 1), false);
    }

    #[test]
    fn test_lookup() {
        assert!(lookup("not implemented operation").is_none());
        assert!(lookup("get").unwrap().name == "get");
    }
}