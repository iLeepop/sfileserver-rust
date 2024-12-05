mod io;
mod http;

use std::net::IpAddr;

use async_std::task::spawn;

use crate::http::Server;
use crate::http::get_local_ip_address;

#[async_std::main]
async fn main() {
    let ip_address = get_local_ip_address();
    let mut servers = Vec::new();
    for (_, ip) in ip_address {
        if ip.is_ipv6() {
            println!("ipv6 is not supported now");
            continue;
        } else if ip.is_ipv4() {
            let ipa: String;
            if ip == IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)) {
                ipa = "localhost".to_string();
            } else {
                ipa = ip.clone().to_string();
            }
            let server = spawn(Server::new(ipa, String::from("8989")).await.run());
            servers.push(server);
        }
    }
    for server in servers {
        server.await;
    }
}