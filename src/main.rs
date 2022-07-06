use std::net::{TcpListener, TcpStream};

mod error;
mod request;

use error::MyError;
use request::Request;

fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(SERVER_ADDR)
        .unwrap_or_else(|_| panic!("Failed to bind server to {}", SERVER_ADDR));

    println!("Successfully bound server to {} !", SERVER_ADDR);
    println!("Listening for clients on {}...", SERVER_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream).expect("Error when handling client"),
            Err(err) => eprintln!("Error from incoming client {}", err),
        }
    }
}

fn handle_client(stream: TcpStream) -> Result<(), MyError> {
    println!("New client !");
    let req: Request = stream.try_into()?;
    println!("{}", req);
    Ok(())
}
