use anyhow::anyhow;

use crate::contracts::{check_balance_rho, set_transfer_rho};
use crate::providers::FireflyProvider;
use crate::transaction::Transaction;

/// Repository for interacting with the Firefly blockchain
/// Provides methods for wallet operations, balance checking, and transaction management
#[derive(Debug, Clone)]
pub struct FireflyRepository {
    pub provider: FireflyProvider,
    pub wallet_address: Option<String>,
    pub wallet_key: Option<String>,
}

impl FireflyRepository {
    /// Creates a new FireflyRepository instance
    ///
    /// # Arguments
    /// * `provider` - The FireflyProvider instance for blockchain communication
    /// * `wallet_address` - The wallet address string
    /// * `wallet_key` - The wallet private key string
    ///
    /// # Returns
    /// A new FireflyRepository instance
    pub fn new(provider: FireflyProvider, wallet_address: &str, wallet_key: &str) -> Self {
        Self {
            provider,
            wallet_address: Some(wallet_address.to_string()),
            wallet_key: Some(wallet_key.to_string()),
        }
    }
    /// Retrieves the wallet address
    ///
    /// # Returns
    /// * `Ok(String)` - The wallet address if set
    /// * `Err` - If wallet address is not set
    pub fn get_wallet_address(&self) -> Result<String, anyhow::Error> {
        match self.wallet_address.clone() {
            Some(key) => Ok(key),
            None => Err(anyhow!("Wallet address is not set.")),
        }
    }
    /// Retrieves the wallet private key
    ///
    /// # Returns
    /// * `Ok(String)` - The wallet key if set
    /// * `Err` - If wallet key is not set
    pub fn get_wallet_key(&self) -> Result<String, anyhow::Error> {
        match self.wallet_key.clone() {
            Some(key) => Ok(key),
            None => Err(anyhow!("Wallet key is not set.")),
        }
    }

    /// Retrieves the current balance for the wallet
    ///
    /// # Returns
    /// * `Ok(u128)` - The wallet balance
    /// * `Err` - If the balance check fails
    pub async fn get_balance(&self) -> Result<u128, anyhow::Error> {
        let wallet_address = &self.get_wallet_address()?;
        let check_balance_code = check_balance_rho(wallet_address)?;

        let data: u64 = self
            .provider
            .read_client()?
            .get_data(&check_balance_code)
            .await?;
        Ok(data as u128)
    }

    /// Initiates a transfer request to another wallet
    ///
    /// # Arguments
    /// * `wallet_address_to` - The recipient's wallet address
    /// * `amount` - The amount to transfer
    /// * `description` - Description of the transfer
    ///
    /// # Returns
    /// * `Ok(String)` - The block hash of the transfer transaction
    /// * `Err` - If the transfer fails
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

    /// Retrieves all transactions for the wallet
    ///
    /// # Returns
    /// * `Ok(Vec<Transaction>)` - List of transactions associated with the wallet
    /// * `Err` - If retrieving transactions fails
    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, anyhow::Error> {
        let client = self.provider.write_client()?;
        let raw_transactions = client.get_transactions().await?;
        let transactions = raw_transactions
            .into_iter()
            .map(|data| Transaction::new(data))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(transactions)
    }
}
