use firefly_api::models::TransferResponse;
use firefly_api::providers::FireflyProvider;
use rocket::State;
use rocket::serde::json::Json;

use super::models::U128Stringified;
use crate::apis::ApiError;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferRequest {
    amount: U128Stringified,
    to_address: String,
    description: Option<String>,
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
    let client = provider.firefly();
    let response_block: TransferResponse = client
        .transfer_request(&to_address, amount, description)
        .await?;

    Ok(Json(response_block))
}
