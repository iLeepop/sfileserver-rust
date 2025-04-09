use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use std::{fs, path::PathBuf};
use local_ip_address::list_afinet_netifas;
use async_std::net::IpAddr;

struct Server {
    listener: TcpListener,
    address: String,
    port: String,
}

impl Server {
    async fn new(address: String, port: String) -> Server {
        Server {
            listener: TcpListener::bind(format!("{}:{}", address, port)).await.unwrap(),
            address,
            port,
        }
    }

    async fn run(&self) {
        println!("Server is running at http://{}:{}", self.address, self.port);
        loop {
            let (mut socket, _) = self.listener.accept().await.unwrap();
            
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let _ = socket.read(&mut buffer).await;
                
                // 解析 HTTP 请求，提取路径
                let request = String::from_utf8_lossy(&buffer);
                let path = parse_request_path(&request);
                let file_path = PathBuf::from("./public").join(path);
                
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

#[tokio::main]
async fn main() {
    let ip_address = get_local_ip_address();
    let mut servers = Vec::new();
    for (_, ip) in ip_address {
        if ip.is_ipv6() {
            println!("ipv6 is not supported now");
            continue;
        } else if ip.is_ipv4() {
            let ipa: String;
            ipa = ip.clone().to_string();
            println!("Scan IPAdress:{}", ipa);
            let server = tokio::spawn(async move {
                Server::new(ipa, String::from("8000")).await.run().await;
            });
            servers.push(server);
        }
    }
    for server in servers {
        let _ = server.await;
    }
}

fn parse_request_path(request: &str) -> &str {
    request.lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/")
        .trim_start_matches('/')
}

fn get_content_type(path: &PathBuf) -> &str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("mp4") => "video/mp4",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}

fn get_local_ip_address() -> Vec<(std::string::String, IpAddr)> {
    let network_interfaces = list_afinet_netifas().unwrap();
    return network_interfaces;
}
