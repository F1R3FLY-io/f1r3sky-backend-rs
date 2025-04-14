use serde_json::Value;
use reqwest::Client as HttpClient;
use anyhow::{anyhow, Context};

pub struct ReadNodeClient {
    read_node_url: String,
}

impl ReadNodeClient {
    pub fn new(read_node_url: String) -> Self {
        Self { read_node_url }
    }

    fn read_node_api(self) -> String {
        let read_node_method = "explore-deploy";
        format!("{}/api/{}", self.read_node_url, read_node_method)
    }

    pub async fn get_data(self, code: String) -> Result<Value, anyhow::Error> {
        // Get the URL from the `read_node_api` function
        let url = self.read_node_api();

        // Create an HTTP client
        let http_client = HttpClient::new();

        let response = http_client
            .post(&url)
            .body(code)
            .header("Content-Type", "text/plain")
            .send()
            .await?;

        // Ensure the request was successful
        if response.status().is_success() {
            // Parse the JSON response
            let json_data: Value = response
                .json()
                .await
                .context("failed to parse response as JSON")?;
            Ok(json_data)
        } else {
            Err(anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ))
        }
    }
}