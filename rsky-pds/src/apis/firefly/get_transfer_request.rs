use rocket::serde::json::Json;

use super::models::TransferRequest;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[tracing::instrument(skip_all)]
#[rocket::get("/request/<id>")]
pub async fn get_transfer_request(
    id: String,
    auth: AccessStandard,
) -> Result<Json<TransferRequest>, ApiError> {
    Ok(Json(TransferRequest {
        amount: 0,
        description: Default::default(),
        user_handle: Default::default(),
    }))
}
