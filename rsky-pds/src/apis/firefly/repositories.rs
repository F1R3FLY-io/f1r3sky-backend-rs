use crate::apis::firefly::contracts::{check_balance_rho, set_transfer_rho};
use crate::apis::firefly::providers::FireflyProvider;
use anyhow::anyhow;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FireflyRepository {
    pub provider: FireflyProvider,
    pub wallet_address: String,
    pub wallet_key: String,
}

impl FireflyRepository {
    pub fn get_wallet_address(&self) -> String {
        self.wallet_address.clone()
    }

    pub async fn get_balance(&self) -> Result<u128, anyhow::Error> {
        let wallet_address = &self.get_wallet_address();
        let check_balance_code = check_balance_rho(wallet_address);

        let json: Value = self
            .provider
            .read_client()?
            .get_data(check_balance_code)
            .await?;
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
        let mut client = self.provider.client().await?;

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
