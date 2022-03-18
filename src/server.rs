use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use thiserror::Error;

use crate::{
    operations::{lookup, OperationResult},
    protocol::{decode, RESPError, RespValueRef},
    repository::Repository,
    request::Request,
};

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Failed to parse request")]
    ProtocolError(#[from] RESPError),
    #[error("Empty message")]
    EmptyMessageError,
    #[error("Malformed Request")]
    BadRequestError,
    #[error("Not implemented")]
    NotImplementedError,
}

pub fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024];
    let mut repo = Repository::new();

    while match stream.read(&mut buffer) {
        Ok(_) => {
            let result = handle_request(&mut buffer, &mut repo);
            let res = match result {
                Ok(v) => v.into(),
                Err(e) => RespValueRef::Failure(e.to_string()),
            };

            stream.write(res.write_resp_value().as_bytes()).unwrap();
            stream.flush().unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn handle_request(buffer: &mut [u8], repo: &mut Repository) -> Result<OperationResult, ResponseError> {
    let raw_message = String::from_utf8_lossy(&buffer[..]);
    println!("Message received:\r\n{}", raw_message);

    let decoded_message = decode(&mut buffer[..])?;

    let Some(parsed_message) = decoded_message else {
        return Err(ResponseError::EmptyMessageError)
    };

    let message_to_request_result: Result<Request, _> = parsed_message.try_into();
    if let Err(_) = message_to_request_result {
        return Err(ResponseError::BadRequestError);
    }
    let request = message_to_request_result.unwrap();

    let Some(operation) = lookup(&request.command()) else {
        return Err(ResponseError::NotImplementedError)
    };

    Ok(operation.execute(repo, &request))
}
