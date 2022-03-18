use std::time::{Instant, Duration};

use crate::{repository::Repository, request::Request};

use super::OperationResult;

pub fn expire(repo: &mut Repository, req: &Request) -> OperationResult {
    let key = &req.arguments()[0];
    if repo.get(key.to_string()).is_none() {
        return OperationResult::Int(0)
    }
    
    let secs = &req.arguments()[1];
    let secs: i64 = secs.parse().unwrap();

    if !secs.is_positive() {
        repo.delete(key.to_string());
        return OperationResult::Int(1)
    };

    let expires_at = Instant::now() + Duration::from_secs(secs.try_into().unwrap());
    repo.set_expiration(key.to_string(), expires_at);
    return OperationResult::Int(1)
}