use crate::{repository::Repository, request::Request};

use super::OperationResult;

pub fn flush_all(repo: &mut Repository, _: &Request) -> OperationResult {
    repo.clear();
    OperationResult::Ok
}