use crate::apis::ApiError;
use firefly_api::models::TransferResponse;
use firefly_api::providers::FireflyProvider;
use rocket::serde::json::Json;
use rocket::State;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    amount: u128,
    to_address: String,
    description: String,
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
        amount,
        to_address,
        description,
        ..
    } = body.into_inner();
    let client = provider.firefly();
    let response_block: TransferResponse = client
        .transfer_request(&to_address, amount, &description)
        .await?;

    Ok(Json(response_block))
}
