use anyhow::{Context, anyhow};
use reqwest::Client as HttpClient;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ReadNodeClient {
    read_node_url: String,
}

impl ReadNodeClient {
    pub fn new(read_node_url: &str) -> Self {
        Self {
            read_node_url: read_node_url.to_string(),
        }
    }

    fn read_node_api(self) -> String {
        let read_node_method = "explore-deploy";
        format!("{}/api/{}", self.read_node_url, read_node_method)
    }

    async fn get_value(self, code: String) -> Result<Value, anyhow::Error> {
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

    fn extract_data_from_response(json: &serde_json::Value) -> Option<&serde_json::Value> {
        json["expr"]
            .as_array()
            .and_then(|expr_array| expr_array.get(0))
            .and_then(|expr| expr["ExprInt"].get("data"))
    }

    pub async fn get_data<T>(self, rholang_code: &str) -> Result<T, anyhow::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let response_json: serde_json::Value = self.get_value(rholang_code.to_string()).await?;

        let data_value = Self::extract_data_from_response(&response_json)
            .ok_or_else(|| anyhow!("Failed to extract data from response structure"))?;

        let parsed_data: T = serde_json::from_value(data_value.clone())
            .context("Failed to deserialize response data into target type")?;

        Ok(parsed_data)
    }
}
