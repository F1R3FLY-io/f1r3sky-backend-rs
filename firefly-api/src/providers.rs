use anyhow::anyhow;

use crate::client::Client;
use crate::read_node_client::ReadNodeClient;
use crate::repositories::FireflyRepository;
use crate::write_node_client::BlocksClient;

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
        let read_client = ReadNodeClient::new(&self.read_node_url);
        Ok(read_client)
    }

    pub fn write_client(&self) -> Result<BlocksClient, anyhow::Error> {
        let blocks_client = BlocksClient::new(&self.write_node_url);
        Ok(blocks_client)
    }

    pub fn firefly(&self) -> FireflyRepository {
        FireflyRepository::new(self.clone(), &self.wallet_address, &self.wallet_key)
    }
}
