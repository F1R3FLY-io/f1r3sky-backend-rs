use anyhow::anyhow;
use secp256k1::SecretKey;

use crate::client::Client;
use crate::read_node_client::ReadNodeClient;
use crate::repositories::FireflyRepository;

#[derive(Debug, Clone)]
pub struct FireflyProvider {
    read_node_url: String,
    deploy_service_url: String,
    propose_service_url: String,
    wallet_address: Option<String>,
    wallet_key: Option<String>,
}

impl FireflyProvider {
    pub fn new(
        read_node_url: String,
        deploy_service_url: String,
        propose_service_url: String,
        wallet_address: Option<String>,
        wallet_key: Option<String>,
    ) -> Result<FireflyProvider, anyhow::Error> {
        Ok(FireflyProvider {
            read_node_url,
            deploy_service_url,
            propose_service_url,
            wallet_address,
            wallet_key,
        })
    }

    pub async fn client(&self, wallet_key: &str) -> Result<Client, anyhow::Error> {
        let wallet_key = SecretKey::from_slice(&hex::decode(wallet_key)?)?;

        let client = Client::new(
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
