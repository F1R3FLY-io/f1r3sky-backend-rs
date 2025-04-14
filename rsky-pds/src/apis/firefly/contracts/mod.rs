use rand::distributions::Alphanumeric;
use rand::Rng;

const CHECK_BALANCE_TMP: &str = include_str!("check_balance.rho");
pub fn check_balance_rho(wallet_address: &str) -> String {
    CHECK_BALANCE_TMP.replace("WALLET_ADDRES", wallet_address)
}

const SET_TRANSFER_TMP: &str = include_str!("set_transfer.rho");

pub fn set_transfer_rho(
    wallet_address_from: &str,
    wallet_address_to: &str,
    amount: u128,
) -> String {
    let random = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>();
    SET_TRANSFER_TMP
        .replace("WALLET_ADDRES_FROM", wallet_address_from)
        .replace("WALLET_ADDRES_TO", wallet_address_to)
        .replace("AMOUNT", &amount.to_string())
        .replace("RANDOM", &random)
}