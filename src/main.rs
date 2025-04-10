mod http;
mod io;
mod utils;

use utils::get_local_ip_address;
use http::Server;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// directory of the contents to view
    #[arg(short, long, default_value_t = String::from("./public"))]
    directory: String,

    /// number of the connection's port
    #[arg(short, long, default_value_t = String::from("8000"))]
    port: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut servers = Vec::new();
    for (_, ip) in get_local_ip_address() {

        let port = args.port.clone();
        let directory = args.directory.clone();

        if ip.is_ipv6() {
            println!("ipv6 is not supported now");
            continue;
        } else if ip.is_ipv4() {
            let ipa: String;
            ipa = ip.clone().to_string();
            let server = tokio::spawn(async move {
                Server::new(ipa, String::from(port), String::from(directory)).await.run().await;
            });
            servers.push(server);
        }
    }
    for server in servers {
        let _ = server.await;
    }
}




