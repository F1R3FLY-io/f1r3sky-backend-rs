use rocket::serde::json::Json;
use super::models::WalletStateAndHistory;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;
// use firefly_api::client::helpers::FromExpr;
// use firefly_api::models::rhoapi::expr::ExprInstance;
use secp256k1::{Keypair, Secp256k1, SecretKey};


#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(
    // auth: AccessStandard,
) -> Result<Json<WalletStateAndHistory>, ApiError> {

    // let wallet_key = auth.wallet_key;
    let wallet_address = "1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh";
    let wallet_secret = "6a786ec387aff99fcce1bd6faa35916bfad3686d5c98e90a89f77670f535607c";

    let deploy_service_url = "http://127.0.0.1:40403";
    let propose_service_url = "http://127.0.0.1:40413";

    let wallet_key = SecretKey::from_slice(&hex::decode(wallet_secret).unwrap()).unwrap();

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
    let check_balance_rho = check_balance_rho_template.replace("WALLET_ADDRES", wallet_address);

    let mut client = firefly_api::Client::new(
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

    let balance_response = client.full_deploy(check_balance_rho).await;
    let balance_response = balance_response.unwrap_or_else(|err| {
        err.to_string()
    });

    println!("balance_response: {:?}", balance_response);




    Ok(Json(WalletStateAndHistory {
        address: Default::default(),
        balance: 0,
        requests: vec![],
        exchanges: vec![],
        boosts: vec![],
        transfers: vec![],
    }))
}
