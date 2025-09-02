//! crate creates a simple echo server in rust
#![allow(unused, dead_code)]

use std::io::{self, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// handling a single client request
pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incomming from tcp-stream: {:?}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        //println!("{:?}", String::from_utf8_lossy(&buf[..bytes_read]))
    }
}

pub fn runs(address: &str) {
    let listenser = TcpListener::bind(address).expect("couldn't bind");
    for stream in listenser.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err));
                });
            }
            Err(e) => {
                eprintln!("failed: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod unittest;
