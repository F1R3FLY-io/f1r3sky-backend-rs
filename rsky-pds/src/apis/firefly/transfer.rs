use rocket::serde::json::Json;

use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    amount: u128,
    to_address: String,
    description: String,
}

#[tracing::instrument(skip_all)]
#[rocket::post("/transfer", format = "json", data = "<body>")]
pub async fn transfer(body: Json<TransferRequest>, auth: AccessStandard) -> Result<(), ApiError> {
    Ok(())
}
