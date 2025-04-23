use firefly_api::providers::FireflyProvider;
use rocket::State;
use rocket::serde::json::Json;

use super::models::WalletStateAndHistory;
use crate::apis::ApiError;
use crate::apis::firefly::state::example_wallet_history;

#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(
    // auth: AccessStandard,  // remove comment to turn on auth
    // TODO: auth should be turned on when we'll solve storage parameters per user
    provider: &State<FireflyProvider>,
) -> Result<Json<WalletStateAndHistory>, ApiError> {
    let client = provider.firefly();
    let wallet_address = client.get_wallet_address();
    let balance = client.get_balance().await.unwrap_or(0);

    let base_state = example_wallet_history(); // TODO: replace with real data
    let state = WalletStateAndHistory {
        balance,
        address: wallet_address,
        ..base_state
    };

    Ok(Json(state))
}
