mod io;
mod http;

use crate::http::Server;

#[async_std::main]
async fn main() {
    let server = Server::new("127.0.0.1:8989").await;
    server.run().await;
}