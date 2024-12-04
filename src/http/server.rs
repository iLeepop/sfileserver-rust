use std::io::BufRead;
use async_std::{io::{ReadExt, WriteExt}, net::{TcpListener as AsyncTcpListener, TcpStream as AsyncTcpStream}, task::spawn};
use rocket::futures::StreamExt;
use crate::{http, io};

pub struct Server<'a> {
    listener: AsyncTcpListener,
    address: &'a str,
}

impl<'a> Server<'a> {
    pub async fn new(address: &str) -> Server {
        Server {
            listener: AsyncTcpListener::bind(&address).await.unwrap(),
            address,
        }
    }
    pub async fn run(self) {
        println!("Server is running at http://{}", self.address);
        self.listener.incoming().for_each_concurrent(None, |async_tcp_stream| async move {
            let stream = async_tcp_stream.unwrap();
            spawn(handle_connection(stream));
        }).await;
    }
}

async fn handle_connection(mut stream: AsyncTcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    
    let line = buffer.lines().nth(0).unwrap().unwrap();
    let url = line.split(" ").nth(1).unwrap();
    println!("{}", url);
    let content: String;
    if io::is_exists(url) {
        if io::is_file(url) {
            content = io::read_file(url);
        } else {
            content = http::content(io::read_dir(url));
        }
    } else {
        content = String::from("404 Not Found");
    }

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

}
