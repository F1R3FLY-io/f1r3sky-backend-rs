use anyhow::anyhow;
use serde_json::Value;

use crate::contracts::{check_balance_rho, set_transfer_rho};
use crate::providers::FireflyProvider;

#[derive(Debug, Clone)]
pub struct FireflyRepository {
    pub provider: FireflyProvider,
    pub wallet_address: Option<String>,
    pub wallet_key: Option<String>,
}

impl FireflyRepository {
    pub fn new(provider: FireflyProvider, wallet_address: &str, wallet_key: &str) -> Self {
        Self {
            provider,
            wallet_address: Some(wallet_address.to_string()),
            wallet_key: Some(wallet_key.to_string()),
        }
    }
    pub fn get_wallet_address(&self) -> Result<String, anyhow::Error> {
        match self.wallet_address.clone() {
            Some(key) => Ok(key),
            None => Err(anyhow!("Wallet address is not set.")),
        }
    }
    pub fn get_wallet_key(&self) -> Result<String, anyhow::Error> {
        match self.wallet_key.clone() {
            Some(key) => Ok(key),
            None => Err(anyhow!("Wallet key is not set.")),
        }
    }

    pub async fn get_balance(&self) -> Result<u128, anyhow::Error> {
        let wallet_address = &self.get_wallet_address()?;
        let check_balance_code = check_balance_rho(wallet_address)?;

        // let json: Value = self
        //     .provider
        //     .read_client()?
        //     .get_data(check_balance_code)
        //     .await?;
        // if let Some(balance) = json["expr"]
        //     .as_array()
        //     .and_then(|expr_array| expr_array.get(0))
        //     .and_then(|expr| expr["ExprInt"].get("data"))
        // {
        //     Ok(balance.as_u64().unwrap() as u128)
        // } else {
        //     Err(anyhow!("Failed to extract balance value."))
        // }
        let data: u64 = self
            .provider
            .read_client()?
            .get_data(&check_balance_code)
            .await?;
        Ok(data as u128)
    }

    pub async fn transfer_request(
        &self,
        wallet_address_to: &str,
        amount: u128,
        description: &str,
    ) -> Result<String, anyhow::Error> {
        let set_transfer = set_transfer_rho(
            &self.get_wallet_address()?,
            wallet_address_to,
            amount,
            description,
        )?;
        let wallet_key = self.get_wallet_key()?;
        let mut client = self.provider.client(&wallet_key).await?;

        let deploy_response = client.deploy(set_transfer).await;
        let _deploy_response_msg = match deploy_response {
            Ok(msg) => msg,
            Err(err) => {
                let error_msg = format!("Failed to deploy transfer code: {err}");
                tracing::error!("{}", &error_msg);
                return Err(anyhow!(error_msg));
            }
        };

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
