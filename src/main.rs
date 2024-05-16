mod io;
mod http;

use crate::http::Server;

fn main() {
    let server = Server::new("127.0.0.1:8989");
    server.run();
}