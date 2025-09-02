//tcp-client.rs file
#![allow(dead_code, unused)]

use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut stream = connects("127.0.0.1:8008").expect("can't read from tcp-stream");
    loop {
        let mut line = String::new();
        let mut buffer = vec![];
        io::stdin().read_line(&mut line).expect("can't read line");
        stream
            .write(line.as_bytes())
            .expect("failed to write to server");

        let mut reader = BufReader::new(&stream);
        reader
            .read_until(('\n' as u8), &mut buffer)
            .expect("can't read into buffer");
        println!(
            "{:?}",
            std::str::from_utf8(&buffer).expect("can't read from buffer")
        )
    }
}

fn connects(address: &str) -> Result<TcpStream, std::io::Error> {
    Ok(TcpStream::connect(address)?)
}
