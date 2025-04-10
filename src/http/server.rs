use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use std::{fs, path::PathBuf};

use super::parse_request_path;
use crate::utils::get_content_type;

pub struct Server {
    address: String,
    port: String,
    directory: String,
}

impl Server {
    pub async fn new(address: String, port: String, directory: String) -> Server {
        Server {
            address,
            port,
            directory,
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).await.unwrap();
        println!("Server is running at http://{}:{}", self.address, self.port);
        
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let directory = self.directory.clone();

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let _ = socket.read(&mut buffer).await;
                
                // 解析 HTTP 请求，提取路径
                let request = String::from_utf8_lossy(&buffer);
                let path = parse_request_path(&request);
                // 指定文件夹
                let file_path = PathBuf::from(directory).join(path);
                
                if file_path.exists() {
                    match fs::read(&file_path) {
                        Ok(content) => {
                            let header = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                                content.len(),
                                get_content_type(&file_path)
                            );
                            let _ = socket.write_all(header.as_bytes()).await;
                            let _ = socket.write_all(&content).await;
                        }
                        Err(_) => {
                            let response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: 21\r\n\r\n500 Internal Server Error";
                            let _ = socket.write_all(response.as_bytes()).await;
                        }
                    }
                } else {
                    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found";
                    let _ = socket.write_all(response.as_bytes()).await;
                }
                
                let _ = socket.flush().await;
            });
        }
    }
}
