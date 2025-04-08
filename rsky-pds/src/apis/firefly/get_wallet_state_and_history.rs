use std::collections::HashMap;
use anyhow::anyhow;
use rocket::serde::json::Json;
use super::models::WalletStateAndHistory;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;
// use firefly_api::client::helpers::FromExpr;
// use firefly_api::models::rhoapi::expr::ExprInstance;
use secp256k1::{Keypair, Secp256k1, SecretKey};
use uuid::Uuid;
use firefly_api::client::helpers::FromExpr;
use firefly_api::models::rhoapi::expr::ExprInstance;

use reqwest::Client;
use rocket::http::tls::rustls::internal::msgs::enums::HeartbeatMessageType::Response;
use serde_json::Value;

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
            // Get the URL from the `read_node_api` function
    let url = read_node_api();

    let check_balance = check_balance_rho(wallet_address);

    // Create an HTTP client
    let http_client = Client::new();


    let response = http_client
        .post(&url)
        .body(check_balance)
        .header("Content-Type", "text/plain")
        .send()
        .await?;

        // Ensure the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let json: Value = response.json().await?;
        println!("JSON response: {:?}", json);
        // Navigate into `expr[0].ExprInt.data`
        if let Some(balance) = json["expr"]
            .as_array()
            .and_then(|expr_array| expr_array.get(0))
            .and_then(|expr| expr["ExprInt"].get("data"))
        {
            println!("ExprInt data value: {}", balance);
            Ok(balance.as_u64().unwrap() as u128)
        } else {
            panic!("Failed to extract ExprInt data value.")
        }


    } else {
        panic!("Failed to send request: {:?}", response.status())
    }
}

#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(
    // auth: AccessStandard,
) -> Result<Json<WalletStateAndHistory>, ApiError> {

    // let wallet_key = auth.wallet_key;
    let wallet_address = "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh";

    let balance = get_balance(wallet_address).await.unwrap_or(0);

    Ok(Json(WalletStateAndHistory {
        address: Default::default(),
        balance,
        requests: vec![],
        exchanges: vec![],
        boosts: vec![],
        transfers: vec![],
    }))
}
