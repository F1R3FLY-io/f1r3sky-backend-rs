use firefly_api::providers::FireflyProvider;
use rocket::serde::json::Json;
use rocket::State;

use super::models::{Direction, Transfer, WalletStateAndHistory};
use super::wallet_history_example::example_wallet_history;
use crate::apis::ApiError;

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
    let transactions = client.get_transactions().await?;

    let mut transfers: Vec<Transfer> = vec![];
    for transaction in transactions {
        if transaction.name != "SET_TRANSFER" {
            break;
        }
        let from_address = &transaction.arguments[0];
        let to_address = &transaction.arguments[1];
        if from_address != &wallet_address && to_address != &wallet_address {
            break;
        }
        let is_incoming = to_address == &wallet_address;
        let direction = if is_incoming {
            Direction::INCOMING
        } else {
            Direction::OUTGOING
        };
        let to_address = if is_incoming {
            from_address.to_string()
        } else {
            to_address.to_string()
        };

        let amount = transaction.arguments[2].parse::<u128>().unwrap_or(0);
        let date = transaction.date_time.timestamp() as u64;

        let transfer = Transfer {
            id: transaction.id.clone(),
            direction,
            date,
            amount,
            to_address,
        };
        transfers.push(transfer);
    }

    let base_state = example_wallet_history(); // TODO: replace with real data
    let state = WalletStateAndHistory {
        balance,
        address: wallet_address,
        transfers,
        ..base_state
    };

    Ok(Json(state))
}
