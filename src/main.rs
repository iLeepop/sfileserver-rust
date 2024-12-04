mod io;
mod http;

use crate::http::Server;
use crate::http::get_local_ip_address;

#[async_std::main]
async fn main() {
    let ip_address = get_local_ip_address();
    for (_, ip) in ip_address {
        Server::new(format!("{}:8080", ip).as_str()).await.run().await;
    }
}