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

fn example_rho() { r#"
  new return, rl(`rho:registry:lookup`), RevVaultCh, vaultCh in {    rl!(`rho:rchain:revVault`, *RevVaultCh) |    for (@(_, RevVault) <- RevVaultCh) {      @RevVault!("findOrCreate", "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh", *vaultCh) |      for (@maybeVault <- vaultCh) {        match maybeVault {          (true, vault) => @vault!("balance", *return)          (false, err)  => return!(err)        }      }    }  }
"#;}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceHash {
    block_hash: String,
    channel_name: Uuid,
}

impl FromExpr for ServiceHash {
    fn from(val: ExprInstance) -> anyhow::Result<Self> {
        let mut map: HashMap<String, String> = FromExpr::from(val)?;

        let block_hash = map.remove("block_hash").unwrap();
        let channel_name = map
            .remove("channel_name").unwrap();
        let channel_name: Uuid = channel_name.parse()?;

        anyhow::Ok(ServiceHash {
            block_hash,
            channel_name,
        })
    }
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
        .post(&read_node_api())
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

async fn get_balance_rho(wallet_address: &str) -> String {
        let wallet_secret = "6a786ec387aff99fcce1bd6faa35916bfad3686d5c98e90a89f77670f535607c";


    let deploy_service_url = "http://127.0.0.1:40401";
    let propose_service_url = "http://127.0.0.1:40402";

    let wallet_key = SecretKey::from_slice(&hex::decode(wallet_secret).unwrap()).unwrap();

    let client = firefly_api::Client::new(
        wallet_key,
        deploy_service_url.parse().unwrap(),
        propose_service_url.parse().unwrap(),
    )
    .await;
    let mut client = match client {
        Ok(client) => client,
        Err(err) => {
            tracing::error!("Failed to create Firefly client: {err}");
            return panic!(
                "Failed to create Firefly client: {err}"
            )
        }
    };

    let check_balance = check_balance_rho(wallet_address);

    let balance_response = client.full_deploy(check_balance).await;
    let balance_response_hash = balance_response.unwrap_or_else(|err| {
        return panic!(
            "Failed to deploy check_balance: {err}"
        )
    });

    println!("balance response hash: {:?}", balance_response_hash);

    let entry: ServiceHash = client
                .get_channel_value(balance_response_hash, "balance".parse().unwrap())
                .await?;

    let balance_response: String = client
        .get_channel_value(entry.block_hash, entry.channel_name.to_string())
        .await
        .unwrap_or_else(|err| {
            format!(
                "Failed to get channel value: {err}"
            )
        });

    println!("balance response: {:?}", balance_response);
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
