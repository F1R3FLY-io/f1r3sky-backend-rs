use anyhow::Context;

use crate::contracts::{check_balance_rho, set_transfer_rho};
use crate::providers::FireflyProvider;

#[derive(Debug, Clone)]
pub struct FireflyRepository {
    pub provider: FireflyProvider,
    pub wallet_address: String,
    pub wallet_key: String,
}

impl FireflyRepository {
    pub fn new(provider: FireflyProvider, wallet_address: String, wallet_key: String) -> Self {
        Self {
            provider,
            wallet_address,
            wallet_key,
        }
    }

    pub fn get_wallet_address(&self) -> String {
        self.wallet_address.clone()
    }

    pub async fn get_balance(&self) -> anyhow::Result<u128> {
        let check_balance_code = check_balance_rho(&self.wallet_address)?;
        self.provider
            .read_client()
            .get_data(check_balance_code)
            .await
    }

    pub async fn transfer_request(
        &self,
        wallet_address_to: &str,
        amount: u128,
        description: &str,
    ) -> anyhow::Result<String> {
        let set_transfer =
            set_transfer_rho(&self.wallet_address, wallet_address_to, amount, description)?;
        let mut client = self.provider.client(&self.wallet_key).await?;

        client
            .full_deploy(set_transfer)
            .await
            .context("Failed to deploy transfer code")
    }
}
