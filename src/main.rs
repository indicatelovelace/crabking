use api_calls::api_calls;
use tokio;

#[tokio::main]
async fn main() {
    // This is a simple example of how to use the functions in api_calls.rs
    // You can use these functions in your own code to interact with the server
    // The functions are async, so you will need to use tokio::main to run them

    api_calls::get_blackboards().await;
}
