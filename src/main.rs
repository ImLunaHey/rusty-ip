use axiom_rs::Client;
use psutil::process::Process;
use serde_json::{json, Value};
use std::{env, time::Duration};
use tokio::time;
use warp::Filter;

async fn log(json: &Value) {
    // Check if TOKEN environment variable exists
    if let Ok(_token) = env::var("AXIOM_TOKEN") {
        // Axiom client
        let client = Client::new().expect("Failed to get axiom client");
        let _ = client.ingest("logs", vec![json]).await;
    } else {
        println!("{}", serde_json::to_string(&json).unwrap());
    }
}

async fn log_stats() {
    // Get memory stats
    let process = Process::new(std::process::id()).expect("Failed to get current process");
    let memory_info = process.memory_info().expect("Failed to get memory info");

    // Generate JSON
    let stats = json!({
        "service": "rusty-ip",
        "memoryUsage": {
            "rss": memory_info.rss()
        },
    });

    // Check if TOKEN environment variable exists
    if let Ok(_token) = env::var("AXIOM_TOKEN") {
        // Axiom client
        let client = Client::new().expect("Failed to get axiom client");
        let _ = client.ingest("stats", vec![stats]).await;
    } else {
        println!("{}", serde_json::to_string(&stats).unwrap());
    }
}

#[tokio::main]
async fn main() {
    // Web port
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT value");

    // Replicate ID
    let replica_id = env::var("RAILWAY_REPLICA_ID")
        .unwrap_or_else(|_| "-1".to_string())
        .unwrap();

    // Every 30s log stats
    let mut interval = time::interval(Duration::from_secs(30));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            log_stats().await;
        }
    });

    // GET / => 200 OK with body
    let ip =
        warp::header::optional::<String>("x-forwarded-for").map(move |header: Option<String>| {
            if let Some(x_forwarded_for) = header {
                // Split the header value by commas and take the first IP address
                let ip_addresses: Vec<&str> = x_forwarded_for.split(',').collect();
                let ip = ip_addresses[0].trim();
                format!("Replica ID: {} IP: {}", replica_id, ip.to_string())
            } else {
                "Unknown IP address".to_string()
            }
        });

    log(&json!({
        "id": replica_id,
        "service": "rusty-ip",
        "port": port,
    }))
    .await;

    warp::serve(ip).run(([0, 0, 0, 0], port)).await;
}
