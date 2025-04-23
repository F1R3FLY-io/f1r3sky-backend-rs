use anyhow::Context;

use crate::client::Client;
use crate::read_node_client::ReadNodeClient;
use crate::repositories::FireflyRepository;

#[derive(Debug, Clone)]
pub struct FireflyProvider {
    read_node_url: String,
    deploy_service_url: String,
    propose_service_url: String,
    wallet_address: String,
    wallet_key: String,
}

impl FireflyProvider {
    pub fn new(
        read_node_url: String,
        deploy_service_url: String,
        propose_service_url: String,
        wallet_address: String,
        wallet_key: String,
    ) -> Self {
        Self {
            read_node_url,
            deploy_service_url,
            propose_service_url,
            wallet_address,
            wallet_key,
        }
    }

    pub async fn client(&self, wallet_key: &str) -> anyhow::Result<Client> {
        Client::new(
            wallet_key,
            self.deploy_service_url.clone(),
            self.propose_service_url.clone(),
        )
        .await
        .context("Failed to create Firefly client")
    }

    pub fn read_client(&self) -> ReadNodeClient {
        ReadNodeClient::new(self.read_node_url.clone())
    }

    pub fn firefly(&self) -> FireflyRepository {
        FireflyRepository::new(
            self.clone(),
            self.wallet_address.clone(),
            self.wallet_key.clone(),
        )
    }
}
