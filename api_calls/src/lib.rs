pub mod api_calls {
    use reqwest::Client;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Blackboard {
        blackboard_name: String,
        duration: i32,
    }

    // needs to be updated to the correct IP
    const IP: &str = "127.0.0.1:5000";

    /////////////////////
    // /blackboards
    /////////////////////
    pub async fn get_blackboards() {
        let route = std::format!("http://{}/blackboards", IP);
        let client = Client::new();
        let res = client.get(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Status: {}", res.status());
            println!("Body:\n{}", res.text().await.unwrap());
        } else {
            println!("Failed to send GET request. Status: {:?}", res.status());
        }
    }

    pub async fn del_blackboards() {
        let route = std::format!("http://{}/blackboards", IP);
        let client = Client::new();
        let res = client.delete(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Status: {}", res.status());
            println!("Body:\n{}", res.text().await.unwrap());
        } else {
            println!("Failed to send DELETE request. Status: {:?}", res.status());
        }
    }

    /////////////////////
    /// /blackboards/{blackboard_name}
    /////////////////////
    pub async fn get_blackboards_specific(blackboard_name: String) {
        let route = std::format!("http://{}/blackboards/{}", IP, blackboard_name);
        let client = reqwest::Client::new();
        let res = client.get(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Status: {}", res.status());
            println!("Body:\n{}", res.text().await.unwrap());
        } else {
            println!("Failed to send GET request. Status: {:?}", res.status());
        }
    }

    pub async fn post_blackboards(blackboard_name: String, duration: i32) {
        let route = std::format!("http://{}/blackboards", IP);
        let blackboard = Blackboard {
            blackboard_name,
            duration,
        };
        let client = Client::new();
        let res = client.post(route).json(&blackboard).send().await.unwrap();
        if res.status().is_success() {
            println!("Success! Response: {:?}", res.text().await);
        } else {
            println!("Failed to send POST request. Status: {:?}", res.status());
        }
    }

    pub async fn del_blackboards_specific(blackboard_name: String) {
        let route = std::format!("http://{}/blackboards/{}", IP, blackboard_name);
        let client = Client::new();
        let res = client.delete(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Status: {}", res.status());
            println!("Body:\n{}", res.text().await.unwrap());
        } else {
            println!("Failed to send DELETE request. Status: {:?}", res.status());
        }
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/staus
    /////////////////////

    pub async fn get_blackboards_status(blackboard_name: String) {
        let route = std::format!("http://{}/blackboards/{}/status", IP, blackboard_name);
        let client = reqwest::Client::new();
        let res = client.get(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Status: {}", res.status());
            println!("Body:\n{}", res.text().await.unwrap());
        } else {
            println!("Failed to send GET request. Status: {:?}", res.status());
        }
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/clear
    /////////////////////

    pub async fn post_blackboards_clear(blackboard_name: String) {
        let route = std::format!("http://{}/blackboards/{}/clear", IP, blackboard_name);
        let client = Client::new();
        let res = client.post(route).send().await.unwrap();
        if res.status().is_success() {
            println!("Success! Response: {:?}", res.text().await);
        } else {
            println!("Failed to send POST request. Status: {:?}", res.status());
        }
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/write
    /////////////////////

    pub async fn post_blackboards_write(blackboardblackboard_name: String, message_text: String) {
        let route = std::format!(
            "http://{}/blackboards/{}/write",
            IP,
            blackboardblackboard_name
        );
        let client = Client::new();
        let res = client.post(route).json(&message_text).send().await.unwrap();
        if res.status().is_success() {
            println!("Success! Response: {:?}", res.text().await);
        } else {
            println!("Failed to send POST request. Status: {:?}", res.status());
        }
    }
}
