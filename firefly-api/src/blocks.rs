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
        path_params: Option<&str>,
        args: Option<&str>,
    ) -> Result<Value, anyhow::Error> {
        let url = self.api_url(method);
        let url = match path_params {
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
        // [
        //     {
        //         "blockHash": "64640917d50bec422b5b0eaec73152ac73605be46b87040d1c35962e86bdbd57",
        //         "sender": "",
        //         "seqNum": 0,
        //         "sig": "",
        //         "sigAlgorithm": "",
        //         "shardId": "root",
        //         "extraBytes": "",
        //         "version": 1,
        //         "timestamp": 1745250032590,
        //         "headerExtraBytes": "",
        //         "parentsHashList": [],
        //         "blockNumber": 0,
        //         "preStateHash": "9619d9a34bdaf56d5de8cfb7c2304d63cd9e469a0bfc5600fd2f5b9808e290f1",
        //         "postStateHash": "7bab67cd2805e231536f3279c8c40654f522402954c889e10e9ca43cf1d268eb",
        //         "bodyExtraBytes": "",
        //         "bonds": [
        //             {
        //                 "validator": "04b103b9a8225589ce98d8417a3744f712e3b1660169e969297ed822e4edd88c13111726d35172ceb8a48065cfce5917292cbe42c8f48a73965100edb094c73365",
        //                 "stake": 4
        //             }
        //         ],
        //         "blockSize": "611829",
        //         "deployCount": 10,
        //         "faultTolerance": 1.0,
        //         "justifications": [],
        //         "rejectedDeploys": []
        //     }
        // ]

        let response = self.api_get("blocks", None, None).await?;

        Ok(response
            .as_array()
            .and_then(|blocks| blocks.get(0))
            .and_then(|block| block["blockHash"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Failed to extract block hash value."))?)
    }

    pub async fn get_blocks_values(&self, hash: String) -> Result<Vec<Value>, anyhow::Error> {
        let response = self
            .api_get("block", Some(&hash), None)
            .await
            .context("failed to get blocks")?;

        let deploys = response["deploys"].as_array().context("missing deploys")?;
        Ok(deploys.to_vec())
    }
}
