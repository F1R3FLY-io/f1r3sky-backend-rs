use crate::client::Client;
use crate::read_node_client::ReadNodeClient;
use crate::repositories::FireflyRepository;
use crate::write_node_client::BlocksClient;
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct FireflyProvider {
    write_node_url: String,
    read_node_url: String,
    deploy_service_url: String,
    propose_service_url: String,
    wallet_address: String,
    wallet_key: String,
}

impl FireflyProvider {
    pub fn new(
        write_node_url: String,
        read_node_url: String,
        deploy_service_url: String,
        propose_service_url: String,
        wallet_address: String,
        wallet_key: String,
    ) -> Self {
        Self {
            write_node_url,
            read_node_url,
            deploy_service_url,
            propose_service_url,
            wallet_address,
            wallet_key,
        }
    }

    pub async fn client(&self, wallet_key: &str) -> Result<Client, anyhow::Error> {
        let client = Client::new(
            wallet_key,
            &self.deploy_service_url,
            &self.propose_service_url,
        )
        .await;
        client.context("Failed to create Firefly client: ")
    }

    pub fn read_client(&self) -> ReadNodeClient {
        ReadNodeClient::new(&self.read_node_url)
    }

    pub fn write_client(&self) -> BlocksClient {
        BlocksClient::new(&self.write_node_url)
    }

    pub fn firefly(&self) -> FireflyRepository {
        FireflyRepository::new(self.clone(), &self.wallet_address, &self.wallet_key)
    }
}
