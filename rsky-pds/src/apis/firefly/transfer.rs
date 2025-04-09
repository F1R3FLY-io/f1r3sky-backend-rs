use rocket::serde::json::Json;

use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

use rocket::http::Status;

use secp256k1::SecretKey;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use toml::to_string;
use uuid::Uuid;
use firefly_api::client::helpers::FromExpr;
use firefly_api::models::rhoapi::expr::ExprInstance;


fn node_api_url(method: &str) -> String {
    let node_url = "http://localhost:40403";
    format!("{}/api/{}", node_url, method)
}

fn set_transfer_rho(wallet_address_from: &str, wallet_address_to: &str, amount: u128) -> String {
    let check_balance_rho_template = r#"
new rl(`rho:registry:lookup`), RevVaultCh in {
    rl!(`rho:rchain:revVault`, *RevVaultCh) |
    for (@(_, RevVault) <- RevVaultCh) {
        new vaultCh, vaultTo, revVaultkeyCh,
        deployerId(`rho:rchain:deployerId`),
        deployId(`rho:rchain:deployId`)
        in {
            match ("WALLET_ADDRES_FROM", "WALLET_ADDRES_TO", AMOUNT) {
                (revAddrFrom, revAddrTo, amount) => {
                    @RevVault!("findOrCreate", revAddrFrom, *vaultCh) |
                    @RevVault!("findOrCreate", revAddrTo, *vaultTo) |
                    @RevVault!("deployerAuthKey", *deployerId, *revVaultkeyCh) |
                    for (@vault <- vaultCh; key <- revVaultkeyCh; _ <- vaultTo) {
                        match vault {
                            (true, vault) => {
                                new resultCh in {
                                    @vault!("transfer", revAddrTo, amount, *key, *resultCh) |
                                    for (@result <- resultCh) {
                                        match result {
                                            (true , _  ) => deployId!((true, "Transfer successful (RANDOM)."))
                                            (false, err) => deployId!((false, err))
                                            }
                                        }
                                    }
                                }
                                err => {
                                deployId!((false, "REV vault cannot be found or created."))
                            }
                        }
                    }
                }
            }
        }
    }
}
"#;
    check_balance_rho_template
        .replace("WALLET_ADDRES_FROM", wallet_address_from)
        .replace("WALLET_ADDRES_TO", wallet_address_to)
        .replace("AMOUNT", &amount.to_string())
        .replace("RANDOM", &Uuid::new_v4().to_string())
}
async fn set_transfer_request(wallet_address_to: &str, amount: u128) -> Result<String, ApiError> {
    let wallet_secret = "6a786ec387aff99fcce1bd6faa35916bfad3686d5c98e90a89f77670f535607c";
    let wallet_key ="6a786ec387aff99fcce1bd6faa35916bfad3686d5c98e90a89f77670f535607c";
    let wallet_address_from ="1111EjdAxnKb5zKUc8ikuxfdi3kwSGH7BJCHKWjnVzfAF3SjCBvjh";


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
            let error_msg = format!("Failed to create Firefly client: {err}");
            tracing::error!("{}", &error_msg);
            return Err(ApiError::InvalidRequest(error_msg))
        }
    };

    let set_transfer = set_transfer_rho(wallet_address_from, wallet_address_to, amount);
    println!("{}", &set_transfer);

    let transfer_response = client.full_deploy(set_transfer).await;
    let response_hash = match transfer_response{
        Ok(hash) => hash,
        Err(err) => {
            let error_msg = format!("Failed to deploy transfer code: {err}");
            tracing::error!("{}", &error_msg);
            return Err(ApiError::InvalidRequest(error_msg))
        }
    };



    // let response: anyhow::Result<String> = client.get_channel_value("transfer".to_string(), response_hash).await;
    // let response = match response{
    //     Ok(response_data) => response_data,
    //     Err(err) => {
    //         let error_msg = format!("Failed to get transfer response: {:?}", err);
    //         tracing::error!("{}", &error_msg);
    //         return Err(ApiError::InvalidRequest(error_msg))
    //     }
    // };

    Ok(response_hash)
}

async fn handle_transfer_response(response_hash: String) -> Result<String, ApiError> {
    let url = node_api_url("data-at-name");

    let body = r#"{"depth":1,"name":{"UnforgDeploy":{"data":"HASH"}}}"#.replace("HASH", &response_hash);

    // Create an HTTP client
    let http_client = Client::new();


    let response = http_client
        .post(&url)
        .body(body)
        .header("Content-Type", "text/plain")
        .send()
        .await
        .map_err(|err| ApiError::InvalidRequest(format!("HTTP request failed: {}", err)))?;
    println!("response: {:?}", response);
    let response_json: Value = response.json().await.map_err(|err| ApiError::InvalidRequest(format!("Failed to parse response: {}", err)))?;
    println!("response_json: {:?}", response_json);

    let result = match response_json["exprs"]
    .as_array()
    .and_then(|exprs| {
        exprs.iter().find(|item| item.get("expr").is_some())
    })
    .and_then(|expr| expr["expr"]["ExprTuple"]["data"].as_array())
    .and_then(|tuple_data| {
        tuple_data.iter().find_map(|item| item.get("ExprString").and_then(|expr_string| expr_string["data"].as_str()))
    })
    {
        Some(expr_string_data) => {
            Some(expr_string_data.to_string())
        }
        None => None
    };

    let sid = match response_json["exprs"]
    .as_array()
    .and_then(|exprs| {
        exprs.iter().find_map(|item| item.get("block").and_then(|block| block.get("sid")))
    })
    {
        Some(sid_value) => {
            Some(sid_value.to_string())
        }
        None => None
    };

    match result {
        Some(result_data) => {
            println!("Result data value: {}", result_data);
            Ok(result_data)
        }
        None => {
            Err(ApiError::InvalidRequest("Deploy error when executing Rholang code.".to_string()))
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    amount: u128,
    to_address: String,
    description: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferResponce {
    id: String,
}


#[tracing::instrument(skip_all)]
#[rocket::post("/transfer", format = "json", data = "<body>")]
pub async fn transfer(
    body: Json<TransferRequest>,
    // auth: AccessStandard
) -> Result<(), ApiError> {
    let TransferRequest { amount, to_address, .. } = body.into_inner();
    // println!("{}", amount);
    // println!("{}", &to_address);
    let response_hash = set_transfer_request(&to_address, amount).await?;
    println!("response_hash: {}", response_hash);

    // let transfer_response_result = handle_transfer_response(response_hash).await?;
    // println!("transfer_response_result: {}", transfer_response_result);

    Ok(())
}
