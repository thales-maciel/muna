use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::{
    operations::{lookup, ReturnValue},
    protocol::{decode, RESPError, RespValueRef},
    repository::{MemoryRepository, Repository},
    request::Request,
};

#[derive(Debug)]
enum Response {
    ProtocolError(RESPError),
    EmptyMessageError,
    BadRequestError,
    NotImplementedError,
    OperationResult(ReturnValue),
}

impl From<Response> for RespValueRef {
    fn from(response: Response) -> Self {
        match response {
            Response::ProtocolError(_) => Self::Failure("Protocol error".to_string()),
            Response::EmptyMessageError => Self::Failure("Empty message".to_string()),
            Response::BadRequestError => Self::Failure("Malformed request".to_string()),
            Response::NotImplementedError => Self::Failure("Not implemented".to_string()),
            Response::OperationResult(r) => r.into(),
        }
    }
}

impl Response {
    fn encode(self) -> String {
        RespValueRef::from(self).write_resp_value()
    }
}

pub fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024];
    let mut repo = MemoryRepository::new();

    while match stream.read(&mut buffer) {
        Ok(_) => {
            let res = handle_request(&mut buffer, &mut repo);
            stream.write(res.encode().as_bytes()).unwrap();
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

fn handle_request(buffer: &mut [u8], repo: &mut dyn Repository) -> Response {
    let raw_message = String::from_utf8_lossy(&buffer[..]);
    println!("Message received:\r\n{}", raw_message);

    let decoded_message = decode(&mut buffer[..]);
    if let Err(e) = decoded_message {
        return Response::ProtocolError(e);
    }

    let Some(parsed_message) = decoded_message.unwrap() else {
        return Response::EmptyMessageError
    };

    let message_to_request_result: Result<Request, _> = parsed_message.try_into();
    if let Err(_) = message_to_request_result {
        return Response::BadRequestError;
    }
    let request = message_to_request_result.unwrap();

    let Some(operation) = lookup(&request.command()) else {
        return Response::NotImplementedError
    };

    let result = operation.execute(repo, &request);
    Response::OperationResult(result)
}
