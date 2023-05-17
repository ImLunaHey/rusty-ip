use std::{env, net::SocketAddr};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Read the port from the environment variable
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT value");

    // GET / => 200 OK with body "127.0.0.1"
    let ip = warp::addr::remote().map(|addr: Option<SocketAddr>| {
        if let Some(socket_addr) = addr {
            format!("{:?}", socket_addr.ip())
        } else {
            "Unknown IP address".to_string()
        }
    });

    warp::serve(ip).run(([127, 0, 0, 1], port)).await;
}
