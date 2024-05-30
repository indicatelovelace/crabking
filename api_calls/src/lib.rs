pub mod api_calls {
    use reqwest::blocking::Client;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Blackboard {
        blackboard_name: String,
        blackboardMessageValitdy: u32,
    }

    // correted IP :)
    const IP: &str = "127.0.0.1:5000/v1";

    /////////////////////
    // /blackboards
    /////////////////////

    /// Get all blackboards
    pub fn get_blackboards() -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards", IP);
        let client = reqwest::blocking::Client::new();
        return client.get(route).send();
    }

    /// Delete all blackboards
    pub fn del_blackboards() -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards", IP);
        let client = reqwest::blocking::Client::new();
        return client.delete(route).send();
    }

    /////////////////////
    /// /blackboards/{blackboard_name}
    /////////////////////

    /// Get a specific blackboard
    pub fn get_blackboards_specific(
        blackboard_name: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}", IP, blackboard_name);
        let client = reqwest::blocking::Client::new();
        return client.get(route).send();
    }

    /// Creates a new blackboard
    pub fn post_blackboards(
        blackboard_name: String,
        duration: u32
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}", IP, blackboard_name);
        let client = Client::new();
        let blackboard = Blackboard {
            blackboard_name,
            blackboardMessageValitdy: duration,
        };
        return client.post(route).json(&blackboard).send();
    }

    /// Deletes a specific blackboard
    pub fn del_blackboards_specific(
        blackboard_name: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}", IP, blackboard_name);
        let client = reqwest::blocking::Client::new();
        return client.delete(route).send();
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/staus
    /////////////////////

    /// Get the status of a specific blackboard
    pub fn get_blackboards_status(
        blackboard_name: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}/status", IP, blackboard_name);
        let client = reqwest::blocking::Client::new();
        return client.get(route).send();
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/clear
    /////////////////////

    /// Clear a specific blackboard
    pub fn post_blackboards_clear(
        blackboard_name: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}/clear", IP, blackboard_name);
        let client = reqwest::blocking::Client::new();
        return client.delete(route).send();
    }

    /////////////////////
    /// /blackboards/{blackboard_name}/write
    /////////////////////

    /// Write a message to a specific blackboard
    pub fn post_blackboards_write(
        blackboard_name: String,
        message_text: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let route = std::format!("http://{}/blackboards/{}/write", IP, blackboard_name);
        let client = reqwest::blocking::Client::new();
        return client.post(route).json(&message_text).send();
    }
}
