use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Create a client
    let client = Client::new();

    // Define the URL and the JSON payload
    let url = "http:localhost:8080/rest";
    let payload = json!({
        "url": "something.com",
        "vote": 1
    });

    // Send the POST request
    match client.post(url).json(&payload).send().await {
        Ok(_) => {}
        Err(err) => {
            println!("Error sending POST request: {}", err);
        }
    }
}
