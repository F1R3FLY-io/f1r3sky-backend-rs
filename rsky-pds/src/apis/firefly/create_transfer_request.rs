use rocket::http::Status;
use rocket::serde::json::Json;

use super::models::Stringified;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTransferRequest {
    amount: Stringified<u128>,
    description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTransferResponce {
    id: String,
}

#[tracing::instrument(skip_all)]
#[rocket::post("/request", format = "json", data = "<body>")]
pub async fn create_transfer_request(
    body: Json<CreateTransferRequest>,
    auth: AccessStandard,
) -> Result<(Status, Json<CreateTransferResponce>), ApiError> {
    Ok((
        Status::Created,
        Json(CreateTransferResponce {
            id: Default::default(),
        }),
    ))
}
