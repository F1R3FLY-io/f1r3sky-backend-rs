use super::models::{Direction, Transfer, WalletStateAndHistory};
use super::wallet_history_example::example_wallet_history;
use crate::apis::ApiError;
use rocket::State;
use rocket::serde::json::Json;

use crate::apis::firefly::transfer::transfer;
use firefly_api::providers::FireflyProvider;

#[tracing::instrument(skip_all)]
#[rocket::get("/state")]
pub async fn get_wallet_state_and_history(
    // auth: AccessStandard,  // remove comment to turn on auth
    // TODO: auth should be turned on when we'll solve storage parameters per user
    provider: &State<FireflyProvider>,
) -> Result<Json<WalletStateAndHistory>, ApiError> {
    let client = provider.firefly();
    let wallet_address = client.get_wallet_address()?;
    let balance = client.get_balance().await.unwrap_or(0);
    let transfers = client
        .get_transactions()
        .await
        .unwrap_or_default()
        .iter()
        .filter(|t| {
            t.name == "SET_BALANCE"
                && (t.arguments[0] == wallet_address || t.arguments[1] == wallet_address)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|t| {
            let mut transfer = t.clone();
            Transfer {
                id: String::from(""),
                direction: if transfer.arguments[1] == wallet_address {
                    Direction::INCOMING
                } else {
                    Direction::OUTGOING
                },
                date: transfer.date_time.timestamp() as u64,
                amount: transfer.arguments[2].parse::<u128>().unwrap_or(0),
                to_address: if transfer.arguments[1] == wallet_address {
                    transfer.arguments[0].clone()
                } else {
                    transfer.arguments[1].clone()
                },
            }
        })
        .collect::<Vec<_>>();

    let base_state = example_wallet_history(); // TODO: replace with real data
    let state = WalletStateAndHistory {
        balance,
        address: wallet_address,
        transfers,
        ..base_state
    };

    Ok(Json(state))
}
