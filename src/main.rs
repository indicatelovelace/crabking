use reqwest;
use std;
use tokio;
use json;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1:5000";

    get_blackboards(ip.to_string()).await;
}

async fn get_blackboards(ip: String) {
    let route = std::format!("http://{}/blackboards", ip);
    let client = reqwest::Client::new();
    let res = client.get(route).send().await.unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());
}

async fn del_blackboards(ip: String) {
    let route = std::format!("http://{}/blackboards", ip);
    let client = reqwest::Client::new();
    let res = client.delete(route).send().await.unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());
}

async fn get_blackboards_specific(ip: String, name: String) {
    let route = std::format!("http://{}/blackboards/{}", ip, name);
    let client = reqwest::Client::new();
    let res = client.get(route).send().await.unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());
}

async fn post_blackboards(ip: String, name: String, duration: i32) {
    let route = std::format!("http://{}/blackboards", ip);
    let client = reqwest::Client::new();
    let res = client.post(route).json(&json!({"name": name, "duration": duration.to_string()})).send().await.unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());
}
