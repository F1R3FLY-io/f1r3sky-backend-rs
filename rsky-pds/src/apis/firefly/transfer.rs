use rocket::serde::json::Json;
use crate::apis::firefly::client::FireflyClient;
use crate::apis::ApiError;
use crate::config::ServerConfig;
use rocket::State;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransferRequest {
    amount: u128,
    to_address: String,
    description: String,
}

#[tracing::instrument(skip_all)]
#[rocket::post("/transfer", format = "json", data = "<body>")]
pub async fn transfer(
    body: Json<TransferRequest>,
    cfg: &State<ServerConfig>,
    // auth: AccessStandard  // remove comment to turn on auth
    // TODO: auth should be turned on when we'll solve storage parameters per user
) -> Result<(), ApiError> {
    let TransferRequest {
        amount, to_address, ..
    } = body.into_inner();
    let client = FireflyClient::new(&cfg)?;
    let _response_hash = client.transfer_request(&to_address, amount).await?;

    Ok(())
}
