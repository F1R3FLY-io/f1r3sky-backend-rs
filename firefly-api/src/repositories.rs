use crate::contracts::{check_balance_rho, set_transfer_rho};
use crate::models::TransferResult;
use crate::providers::FireflyProvider;
use crate::transaction::Transaction;
use anyhow::Context;

/// Repository for interacting with the Firefly blockchain
/// Provides methods for wallet operations, balance checking, and transaction management
#[derive(Debug, Clone)]
pub struct FireflyRepository<'a> {
    pub provider: FireflyProvider,
    pub wallet_address: &'a str,
    pub wallet_key: &'a str,
}

impl<'a> FireflyRepository<'a> {
    /// Creates a new FireflyRepository instance
    ///
    /// # Arguments
    /// * `provider` - The FireflyProvider instance for blockchain communication
    /// * `wallet_address` - The wallet address string
    /// * `wallet_key` - The wallet private key string
    ///
    /// # Returns
    /// A new FireflyRepository instance
    pub fn new(provider: FireflyProvider, wallet_address: &'a str, wallet_key: &'a str) -> Self {
        Self {
            provider,
            wallet_address,
            wallet_key,
        }
    }
    /// Retrieves the wallet address
    pub fn get_wallet_address(&self) -> &str {
        self.wallet_address
    }
    /// Retrieves the wallet private key
    pub fn get_wallet_key(&self) -> &str {
        self.wallet_key
    }

    /// Retrieves the current balance for the wallet
    ///
    /// # Returns
    /// * `Ok(u128)` - The wallet balance
    /// * `Err` - If the balance check fails
    pub async fn get_balance(&self) -> anyhow::Result<u128> {
        let wallet_address = &self.get_wallet_address();
        let check_balance_code = check_balance_rho(wallet_address)?;

        let data: u128 = self
            .provider
            .read_client()
            .get_data(check_balance_code)
            .await?;
        Ok(data)
    }

    /// Initiates a transfer request to another wallet
    ///
    /// # Arguments
    /// * `wallet_address_to` - The recipient's wallet address
    /// * `amount` - The amount to transfer
    /// * `description` - Description of the transfer
    ///
    /// # Returns
    /// * `Ok(TransferResponse)` - Transfer result.
    /// * `Err` - If the transfer fails
    pub async fn transfer_request(
        &self,
        wallet_address_to: &str,
        amount: u128,
        description: Option<String>,
    ) -> anyhow::Result<TransferResult> {
        let set_transfer = set_transfer_rho(
            &self.get_wallet_address(),
            wallet_address_to,
            amount,
            description,
        )?;
        let wallet_key = self.get_wallet_key();
        let mut client = self.provider.client(&wallet_key).await?;
        let block_client = self.provider.write_client();

        let deploy_response = client.deploy(set_transfer).await;
        let sid = deploy_response.context("Failed to deploy transfer code: ")?;

        let block_hash = client.propose().await;
        let block_hash = block_hash.context("Failed to propose transfer code: ")?;

        let response_block = block_client.get_deploy_results(&block_hash, &sid).await?;

        TransferResult::new(response_block)
    }

    /// Retrieves all transactions for the wallet
    ///
    /// # Returns
    /// * `Ok(Vec<Transaction>)` - List of transactions associated with the wallet
    /// * `Err` - If retrieving transactions fails
    pub async fn get_transactions(&self) -> anyhow::Result<Vec<Transaction>> {
        let client = self.provider.write_client();
        let raw_transactions = client.get_transactions().await?;
        let transactions = raw_transactions
            .into_iter()
            .map(|data| Transaction::new(data))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(transactions)
    }
}
