use std::collections::HashSet;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use csv::ReaderBuilder;
use reqwest::Client as HttpClient;
use serde_json::Value;

/// Extracts and filters deploy information from a vector of JSON Values.
/// Only processes non-errored deploys that start with "//FIREFLY_OPERATION".
///
/// # Arguments
/// * `deploys` - Vector of JSON Values containing deploy information
///
/// # Returns
/// Vector of tuples containing:
/// * Deploy signature (String)
/// * Timestamp (DateTime<Utc>)
/// * CSV values extracted from the first line (Vec<String>)
///
/// # Processing Steps
/// 1. Filters out errored deploys
/// 2. Checks if term starts with "//FIREFLY_OPERATION"
/// 3. Parses first line as CSV
/// 4. Extracts signature and timestamp
/// 5. Returns tuple of (signature, timestamp, csv_values)
fn extract_filtered_deploys(deploys: Vec<Value>) -> Vec<(String, DateTime<Utc>, Vec<String>, u64)> {
    deploys
        .into_iter()
        .filter_map(|deploy| {
            if deploy["errored"].as_bool() != Some(false) {
                return None;
            }

            let term = deploy["term"].as_str()?;
            let first_line = term.lines().find(|line| !line.trim().is_empty())?;

            if !first_line.starts_with("//FIREFLY_OPERATION") {
                return None;
            }

            let mut csv_reader = ReaderBuilder::new()
                .delimiter(b';')
                .has_headers(false)
                .from_reader(first_line.as_bytes());
            let csv_values: Vec<String> = csv_reader
                .records()
                .next()
                .and_then(|record| record.ok())
                .map(|record| record.iter().map(|s| s.to_string()).collect())
                .unwrap_or_default();

            // Usage
            let unix_timestamp_ms = deploy["timestamp"]
                .as_i64()
                .expect("invalid timestamp format");
            let datetime = DateTime::from_timestamp_millis(unix_timestamp_ms)
                .expect("invalid unix timestamp value");
            let sig = deploy["sig"].as_str()?.to_string();
            let cost = deploy["cost"].as_u64().unwrap_or(0);
            return Some((sig, datetime, csv_values, cost));
        })
        .collect()
}

/// Processes a vector of tuples containing signatures, timestamps, and CSV values.
/// Filters out duplicate entries based on signatures, sorts by timestamp,
/// and returns only the CSV values portion of each tuple.
///
/// # Arguments
/// * `tuples` - Vector of tuples containing (signature, timestamp, csv_values)
///
/// # Returns
/// Vector of tuples with timestamp as Datetime and CSV value vectors, sorted by timestamp with duplicates removed
fn process_tuples(
    tuples: Vec<(String, DateTime<Utc>, Vec<String>, u64)>,
) -> Vec<(String, DateTime<Utc>, Vec<String>, u64)> {
    let mut seen_sigs = HashSet::new();
    let mut unique_tuples: Vec<_> = tuples
        .into_iter()
        .filter(|(sig, _, _, _)| seen_sigs.insert(sig.to_string()))
        .collect();
    unique_tuples.sort_by_key(|(_, datetime, _, _)| *datetime);
    unique_tuples
}

#[derive(Debug, Clone)]
pub struct BlocksClient {
    http_client: HttpClient,
    block_node_url: String,
}

impl BlocksClient {
    pub fn new(block_node_url: &str) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client,
            block_node_url: block_node_url.to_string(),
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

    async fn first_block_hash(&self) -> Result<String, anyhow::Error> {
        let response = self.api_get("blocks", None, None).await?;

        response
            .as_array()
            .and_then(|blocks| blocks.get(0))
            .and_then(|block| block["blockHash"].as_str())
            .map(|s| s.to_string())
            .context("Failed to extract block hash value.")
    }

    async fn walk_deploys_values(
        &self,
        hash: &str,
    ) -> Result<
        (
            Option<Vec<String>>,
            Vec<(String, DateTime<Utc>, Vec<String>, u64)>,
        ),
        anyhow::Error,
    > {
        let response = self
            .api_get("block", Some(hash), None)
            .await
            .context("failed to get blocks")?;

        let parents_hash_list = response["blockInfo"]["parentsHashList"]
            .as_array()
            .map(|list| {
                list.iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });

        let deploys = response["deploys"].as_array().context("missing deploys")?;

        let filtered_deploys = extract_filtered_deploys(deploys.to_vec());

        Ok((parents_hash_list, filtered_deploys))
    }

    pub async fn get_block_by_hash(&self, hash: &str) -> anyhow::Result<Value> {
        self.api_get("block", Some(hash), None)
            .await
            .context("failed to get block")
    }

    pub async fn get_deploy_results(&self, hash: &str, sid: &str) -> anyhow::Result<Value> {
        let block = self.get_block_by_hash(hash).await?;
        let deploys = block["deploys"].as_array().context("missing deploys")?;
        let deploy = deploys
            .iter()
            .find(|deploy| deploy["sig"].as_str() == Some(sid))
            .ok_or_else(|| anyhow!("Deploy not found"))?
            .to_owned();
        Ok(deploy)
    }

    pub async fn get_transactions(
        &self,
    ) -> Result<Vec<(String, DateTime<Utc>, Vec<String>, u64)>, anyhow::Error> {
        let mut current_hash = self.first_block_hash().await?;
        let mut result = vec![];
        let mut hash_list: Vec<String> = vec![];

        loop {
            let (next_hash_list, values) = self.walk_deploys_values(&current_hash).await?;
            result.extend(values.into_iter().map(|v| v));

            if let Some(hashes) = next_hash_list {
                hash_list.extend(hashes);
            }

            if let Some(hash) = hash_list.pop() {
                current_hash = hash;
            } else {
                break;
            }
        }

        let processed_values = process_tuples(result);

        Ok(processed_values)
    }
}
