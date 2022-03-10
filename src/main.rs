#![feature(let_else)]

mod server;
mod operations;
mod protocol;
mod record;
mod repository;
mod request;

use std::net::TcpListener;

use crate::server::handle_connection;

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
