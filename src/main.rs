mod repository;
mod protocol;
mod commands;
mod record;
mod operations;
mod request;

use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream}, collections::HashMap,
};

use repository::MemoryRepository;

use crate::{commands::{Command, translate_array_to_command}, protocol::write_resp_value, record::{Record, Types, get_string, get_hash_value, set_hash_value}, operations::ReturnValue};
use crate::protocol::{decode, RespValueRef};
use crate::repository::Repository;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    drop(listener);
}

fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024];
    let mut repo = MemoryRepository::new();

    while match stream.read(&mut buffer) {
        Ok(_) => {
            let req = String::from_utf8_lossy(&buffer[..]);
            println!("Request:\r\n{}", req);
            let result: ReturnValue = match decode(&mut buffer[..]) {
                Ok(o) => match o {
                    Some(RespValueRef::Array(r)) => {
                        match translate_array_to_command(&r) {
                            Ok(Command::COMMAND) => ReturnValue::Nil,
                            Ok(Command::HGET(key, field)) => {
                                if let Ok(r) = repo.get(key) {
                                    match get_hash_value(r, field) {
                                        Ok(s) => ReturnValue::StringRes(s),
                                        _ => ReturnValue::Nil
                                    }
                                } else {
                                    ReturnValue::Nil
                                }
                            },
                            Ok(Command::HSET(key, values)) => {
                                if let Ok(r) = repo.get(key.clone()) {
                                    for pair in values.chunks(2) {
                                        set_hash_value(r.clone(), pair[0].to_string(), pair[1].to_string());
                                    }
                                    repo.set(key.to_string(), r);
                                } else {
                                    let mut new_set = HashMap::new();
                                    for pair in values.chunks(2) {
                                        new_set.insert(pair[0].to_string(), pair[1].to_string());
                                    }
                                    repo.set(key, Record { value: record::Types::HashMap(new_set) });
                                }
                                ReturnValue::Nil
                            },
                            Ok(Command::SET(key, value)) => match repo.set(key, Record { value: Types::String(value) }) {
                                Ok(_) => ReturnValue::Ok,
                                Err(e) => ReturnValue::Error(e),
                            },
                            Ok(Command::GET(key)) => {
                                if let Ok(record) = repo.get(key) {
                                    get_string(record)
                                        .map(|s| ReturnValue::StringRes(s))
                                        .unwrap_or_else(|e| ReturnValue::Error(e))
                                } else {
                                    ReturnValue::Nil
                                }
                            },
                            // Ok(Command::GET(key)) => match repo.get(key) {
                            //     Ok(r) => match get_string(r) {
                            //         Ok(s) => ReturnValue::StringRes(s),
                            //         Err(e) => ReturnValue::Error(e),
                            //     },
                            //     Err(_) => ReturnValue::Nil,
                            // },
                            Err(e) => ReturnValue::Error(e),
                        }
                    },
                    _ => ReturnValue::Error("Protocol Error, not an array.".to_string()),
                },
                Err(_) => ReturnValue::Error("Protocol Error".to_string()),
            };
            let res = write_resp_value(result.into());
            stream.write(res.as_bytes()).unwrap();
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
