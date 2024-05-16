use std::{io::{BufRead, Read, Write}, net::{TcpListener, TcpStream}};

use crate::{http, io};

pub struct Server<'a> {
    listener: TcpListener,
    address: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(address: &str) -> Server {
        Server {
            listener: TcpListener::bind(&address).unwrap(),
            address,
        }
    }
    pub fn run(self) {
        println!("Server is running at {}", self.address);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let line = buffer.lines().nth(0).unwrap().unwrap();
    let url = line.split(" ").nth(1).unwrap();

    let content = http::content(io::read_dir(url));

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn handle_route() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {
        let server = Server::new("127.0.0.1:8080");
        server.run();
    }
}