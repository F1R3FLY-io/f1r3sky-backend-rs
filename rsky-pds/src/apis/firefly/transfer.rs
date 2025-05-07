use firefly_api::client::helpers::verify_rev_addr;
use firefly_api::models::TransferResult;
use firefly_api::providers::FireflyProvider;
use rocket::State;
use rocket::serde::json::Json;
use serde_json::Value;

use super::models::U128Stringified;
use crate::apis::ApiError;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferRequest {
    amount: U128Stringified,
    to_address: String,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferResponse {
    cost: String,
}

#[tracing::instrument(skip_all)]
#[rocket::post("/transfer", format = "json", data = "<body>")]
pub async fn transfer(
    body: Json<TransferRequest>,
    provider: &State<FireflyProvider>,
    // auth: AccessStandard  // remove comment to turn on auth
    // TODO: auth should be turned on when we'll solve storage parameters per user
) -> Result<Json<TransferResponse>, ApiError> {
    let TransferRequest {
        amount: U128Stringified(amount),
        to_address,
        description,
        ..
    } = body.into_inner();
    if !verify_rev_addr(to_address.as_str()) {
        return Err(ApiError::InvalidRequest(format!(
            "Invalid address: {}",
            to_address
        )));
    }
    let client = provider.firefly();
    let response_block = client
        .transfer_request(&to_address, amount, description)
        .await
        .map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let system_deploy_error = response_block.system_deploy_error.map(|s| s.to_string());
    if response_block.errored {
        return Err(ApiError::InvalidRequest(
            system_deploy_error.unwrap_or_else(|| "Unknown error".to_string()),
        ));
    }
    Ok(Json(TransferResponse {
        cost: response_block.cost.to_string(),
    }))
}
