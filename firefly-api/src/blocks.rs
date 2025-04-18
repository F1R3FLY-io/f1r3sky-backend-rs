use anyhow::{Context, anyhow};
use reqwest::Client as HttpClient;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BlocksClient {
    http_client: HttpClient,
    block_node_url: String,
}

impl BlocksClient {
    pub fn new(block_node_url: String) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client,
            block_node_url,
        }
    }

    fn api_url(&self, method: &str) -> String {
        format!("{}/api/{}", self.block_node_url, method)
    }

    async fn api_get(
        &self,
        method: &str,
        params: Option<&str>,
        args: Option<&str>,
    ) -> Result<Value, anyhow::Error> {
        let url = self.api_url(method);
        let url = match params {
            Some(params) => format!("{}/{}", url, params),
            None => url,
        };
        let url = match args {
            Some(args) => format!("{}?{}", url, args),
            None => url,
        };
        let response = self
            .http_client
            .get(&url)
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

    pub async fn get_blocks_hash(&self, code: String) -> Result<String, anyhow::Error> {
        // Get the URL from the `read_node_api` function
        let url = self.api_url("blocks");

        // Create an HTTP client

        let response = self.api_get("blocks", None, None).await?;
    }
}
