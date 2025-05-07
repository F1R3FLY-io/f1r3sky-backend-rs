use rocket::http::Status;
use rocket::serde::json::Json;

use super::models::U128Stringified;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTransferRequest {
    amount: U128Stringified,
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
