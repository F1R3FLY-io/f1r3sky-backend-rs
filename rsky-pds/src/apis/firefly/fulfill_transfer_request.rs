use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[tracing::instrument(skip_all)]
#[rocket::get("/request/<id>/fulfill")]
pub async fn fulfill_transfer_request(id: String, auth: AccessStandard) -> Result<(), ApiError> {
    Ok(())
}
