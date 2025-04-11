use super::models::WalletStateAndHistory;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;
use anyhow::anyhow;
use rocket::serde::json::Json;
use std::collections::HashMap;
// use firefly_api::client::helpers::FromExpr;
// use firefly_api::models::rhoapi::expr::ExprInstance;
use apis::firefly::state::example_wallet_history;
use firefly_api::client::helpers::FromExpr;
use firefly_api::models::rhoapi::expr::ExprInstance;
use secp256k1::{Keypair, Secp256k1, SecretKey};
use uuid::Uuid;

use firefly_api::client::ReadNodeClient;
use reqwest::Client;
use rocket::http::tls::rustls::internal::msgs::enums::HeartbeatMessageType::Response;
use serde_json::Value;
use crate::apis;

fn check_balance_rho(wallet_address: &str) -> String {
    let check_balance_rho_template = r#"
  new return, rl(`rho:registry:lookup`), RevVaultCh, vaultCh in {
    rl!(`rho:rchain:revVault`, *RevVaultCh) |
    for (@(_, RevVault) <- RevVaultCh) {
      @RevVault!("findOrCreate", "WALLET_ADDRES", *vaultCh) |
      for (@maybeVault <- vaultCh) {
        match maybeVault {
          (true, vault) => @vault!("balance", *return)
          (false, err)  => return!(err)
        }
      }
    }
  }
"#;
    check_balance_rho_template.replace("WALLET_ADDRES", wallet_address)
}
fn read_node_api() -> String {
    let read_node_url = "http://localhost:40413";
    let read_node_method = "explore-deploy";
    format!("{}/api/{}", read_node_url, read_node_method)
}

async fn get_balance(wallet_address: &str) -> Result<u128, anyhow::Error> {
    let check_balance_code = check_balance_rho(wallet_address);

    let client = ReadNodeClient::new();

    let json: Value = client.get_data(check_balance_code).await?;
    println!("JSON response: {:?}", json);
    // Navigate into `expr[0].ExprInt.data`
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

#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(// auth: AccessStandard,
) -> Result<Json<WalletStateAndHistory>, ApiError> {
    // let wallet_key = auth.wallet_key;
    let wallet_address = "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh";
    let balance = get_balance(wallet_address).await.unwrap_or(0);

    let mut base_state = example_wallet_history();
    let state = WalletStateAndHistory{
        balance,
        address: wallet_address.to_string(),
        ..base_state
    };

    Ok(Json(state))
}
