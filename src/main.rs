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
    let ip = warp::header::optional::<String>("x-forwarded-for").map(|header: Option<String>| {
        if let Some(x_forwarded_for) = header {
            // Split the header value by commas and take the first IP address
            let ip_addresses: Vec<&str> = x_forwarded_for.split(',').collect();
            let ip = ip_addresses[0].trim();
            ip.to_string()
        } else {
            "Unknown IP address".to_string()
        }
    });

    println!("rusty-ip started on port {}", port);

    warp::serve(ip).run(([0, 0, 0, 0], port)).await;
}
