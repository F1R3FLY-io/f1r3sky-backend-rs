use rocket::serde::json::Json;

use super::models::WalletStateAndHistory;
use crate::apis::ApiError;
use crate::auth_verifier::AccessStandard;

#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(
    auth: AccessStandard,
) -> Result<Json<WalletStateAndHistory>, ApiError> {
    Ok(Json(WalletStateAndHistory {
        address: Default::default(),
        balance: 0,
        requests: vec![],
        exchanges: vec![],
        boosts: vec![],
        transfers: vec![],
    }))
}
