// Create a default reedline object to handle user input

use reqwest;
use tokio;


#[tokio::main]
async fn main() {
   
   
    let client = reqwest::Client::new();
    let res = client.get("http://127.0.0.1:5000/blackboards")
        .send().await
        .unwrap();
    println!("Status: {}", res.status());
    println!("Body:\n{}", res.text().await.unwrap());

}
