use anyhow::Context;
use firefly_api::providers::FireflyProvider;
use rsky_common::env::env_str;

pub fn get_firefly_provider() -> anyhow::Result<FireflyProvider> {
    let write_node_url = env_str("WRITE_NODE_URL")
        .context("Failed to get read node url, set in .env WRITE_NODE_URL")?;
    let read_node_url = env_str("READ_NODE_URL")
        .context("Failed to get read node url, set in .env READ_NODE_URL")?;
    let deploy_service_url = env_str("DEPLOY_SERVICE_URL")
        .context("Failed to get read node url, set in .env DEPLOY_SERVICE_URL")?;
    let propose_service_url = env_str("PROPOSE_SERVICE_URL")
        .context("Failed to get read node url, set in .env PROPOSE_SERVICE_URL")?;

    let default_wallet_key = env_str("DEFAULT_WALLET_KEY")
        .context("Failed to get read node url, set in .env DEFAULT_WALLET_KEY")?;
    let default_wallet_address = env_str("DEFAULT_WALLET_ADDRESS")
        .context("Failed to get read node url, set in .env DEFAULT_WALLET_ADDRESS")?;

    let provider = FireflyProvider::new(
        write_node_url,
        read_node_url,
        deploy_service_url,
        propose_service_url,
        default_wallet_address,
        default_wallet_key,
    );
    Ok(provider)
}
