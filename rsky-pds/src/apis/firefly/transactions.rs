use firefly_api::providers::FireflyProvider;
use firefly_api::transaction::Transaction;
use rocket::State;
use rocket::serde::json::Json;

use crate::apis::ApiError;

#[rocket::get("/transactions")]
pub async fn get_transactions(
    provider: &State<FireflyProvider>,
) -> Result<Json<Vec<Transaction>>, ApiError> {
    let client = provider.firefly();
    let wallet_address = client.get_wallet_address()?;
    let balance = client.get_balance().await.unwrap_or(0);
    let transactions = client.get_transactions().await?;

    Ok(Json(transactions))
}
