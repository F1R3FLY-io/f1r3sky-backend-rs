use anyhow::{anyhow, Context};
use secp256k1::SecretKey;

use firefly_api::client::Client;
use firefly_api::read_node_client::ReadNodeClient;
use rsky_common::env::env_str;

use crate::apis::firefly::repositories::FireflyRepository;

#[derive(Debug, Clone)]
pub struct FireflyProvider {
    wallet_address: String,
    read_node_url: String,
    deploy_service_url: String,
    propose_service_url: String,
    wallet_key: String,
}

impl FireflyProvider {
    pub fn new() -> Result<FireflyProvider, anyhow::Error> {
        let read_node_url = env_str("READ_NODE_URL")
            .context("Failed to get read node url, set in .env READ_NODE_URL")?;
        let default_wallet_key = &env_str("DEFAULT_WALLET_KEY")
            .context("Failed to get read node url, set in .env DEFAULT_WALLET_KEY")?;
        let default_wallet_address = env_str("DEFAULT_WALLET_ADDRESS")
            .context("Failed to get read node url, set in .env DEFAULT_WALLET_ADDRESS")?;
        let deploy_service_url = env_str("DEPLOY_SERVICE_URL")
            .context("Failed to get read node url, set in .env DEPLOY_SERVICE_URL")?;
        let propose_service_url = env_str("PROPOSE_SERVICE_URL")
            .context("Failed to get read node url, set in .env PROPOSE_SERVICE_URL")?;

        Ok(FireflyProvider {
            wallet_address: default_wallet_address.to_string(),
            read_node_url: read_node_url.to_string(),
            deploy_service_url: deploy_service_url.to_string(),
            propose_service_url: propose_service_url.to_string(),
            wallet_key: default_wallet_key.to_string(),
        })
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

    pub fn firefly(&self) -> FireflyRepository {
        FireflyRepository {
            provider: self.clone(),
            wallet_address: self.wallet_address.clone(),
            wallet_key: self.wallet_key.clone(),
        }
    }
}
