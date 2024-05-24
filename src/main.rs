// Create a default reedline object to handle user input

use reqwest;
use std;
use tokio;

#[tokio::main]
async fn main() {
    let ip = "192.168.2.159";

    get_blackboards(ip.to_string()).await;
}

async fn get_blackboards(ip: String) {
    let route = std::format!("http://{}/blackboards", ip);
    let client = reqwest::Client::new();
    let res = client.get(route).send().await.unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());
}
