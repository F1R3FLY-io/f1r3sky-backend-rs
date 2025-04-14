use anyhow::{anyhow, Context};
use secp256k1::SecretKey;
use firefly_api::client::Client;
use firefly_api::read_node_client::ReadNodeClient;
use serde_json::Value;

use crate::config::ServerConfig;
use crate::apis::firefly::contracts::check_balance_rho;
use crate::apis::firefly::contracts::set_transfer_rho;

pub struct FireflyClient {
    wallet_address: String,
    read_node_url: String,
    deploy_service_url: String,
    propose_service_url: String,
    wallet_key: String,
}

impl FireflyClient {
    pub fn new(server_cfg: &ServerConfig) -> Result<FireflyClient, anyhow::Error> {
        let cfg = server_cfg.clone();
        let read_node_url = &cfg
            .read_node_url
            .context("Failed to get read node url, set in .env READ_NODE_URL")?;
        let default_wallet_key = &cfg
            .default_wallet_key
            .context("Failed to get read node url, set in .env DEFAULT_WALLET_KEY")?;
        let default_wallet_address = &cfg
            .default_wallet_address
            .context("Failed to get read node url, set in .env DEFAULT_WALLET_ADDRESS")?;
        let deploy_service_url = &cfg
            .deploy_service_url
            .context("Failed to get read node url, set in .env DEPLOY_SERVICE_URL")?;
        let propose_service_url = &cfg
            .propose_service_url
            .context("Failed to get read node url, set in .env PROPOSE_SERVICE_URL")?;
        Ok(FireflyClient {
            wallet_address: default_wallet_address.to_string(),
            read_node_url: read_node_url.to_string(),
            deploy_service_url: deploy_service_url.to_string(),
            propose_service_url: propose_service_url.to_string(),
            wallet_key: default_wallet_key.to_string(),
        })
    }

    pub fn get_wallet_address(&self) -> String {
        self.wallet_address.clone()
    }

    pub async fn client(&self) -> Result<Client, anyhow::Error> {
        let wallet_key = &self.wallet_key;

        let wallet_key = SecretKey::from_slice(&hex::decode(wallet_key)?)?;

        let client = firefly_api::Client::new(
            wallet_key,
            self.deploy_service_url.parse()?,
            self.propose_service_url.parse()?,
        )
        .await;
        let client = match client {
            Ok(client) => client,
            Err(err) => {
                let error_msg = format!("Failed to create Firefly client: {err}");
                tracing::error!("{}", &error_msg);
                return Err(anyhow!(error_msg));
            }
        };

        Ok(client)
    }

    pub fn read_client(&self) -> Result<ReadNodeClient, anyhow::Error> {
        let read_client = ReadNodeClient::new(self.read_node_url.parse()?);
        Ok(read_client)
    }

    pub async fn get_balance(&self) -> Result<u128, anyhow::Error> {
        let wallet_address = &self.get_wallet_address();
        let check_balance_code = check_balance_rho(wallet_address);

        let json: Value = self.read_client()?.get_data(check_balance_code).await?;
        if let Some(balance) = json["expr"]
            .as_array()
            .and_then(|expr_array| expr_array.get(0))
            .and_then(|expr| expr["ExprInt"].get("data"))
        {
            Ok(balance.as_u64().unwrap() as u128)
        } else {
            Err(anyhow!("Failed to extract balance value."))
        }
    }

    pub async fn transfer_request(
        &self,
        wallet_address_to: &str,
        amount: u128,
    ) -> Result<String, anyhow::Error> {
        let set_transfer = set_transfer_rho(&self.get_wallet_address(), wallet_address_to, amount);
        // println!("{}", &set_transfer);
        let mut client = self.client().await?;

        let deploy_response = client.deploy(set_transfer).await;
        let _deploy_response_msg = match deploy_response {
            Ok(msg) => msg,
            Err(err) => {
                let error_msg = format!("Failed to deploy transfer code: {err}");
                tracing::error!("{}", &error_msg);
                return Err(anyhow!(error_msg));
            }
        };
        // println!("deploy response: {}", &deploy_response_msg);

        let block_hash = client.propose().await;
        let block_hash = match block_hash {
            Ok(hash) => hash,
            Err(err) => {
                let error_msg = format!("Failed to propose transfer code: {err}");
                tracing::error!("{}", &error_msg);
                return Err(anyhow!(error_msg));
            }
        };

        Ok(block_hash)
    }
}
