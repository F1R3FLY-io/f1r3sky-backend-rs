use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "check_balance.rho")]
#[template(rm_whitespace = true)]
struct CheckBallanceTemplate {
    wallet_address: String,
}

#[derive(TemplateSimple)]
#[template(path = "set_transfer.rho")]
#[template(rm_whitespace = true)]
struct SetTransferTemplate {
    wallet_address_from: String,
    wallet_address_to: String,
    amount: u128,
    description: String,
}

pub fn check_balance_rho(wallet_address: &str) -> Result<String, anyhow::Error> {
    let ctx = CheckBallanceTemplate {
        wallet_address: wallet_address.to_string(),
    };
    Ok(ctx.render_once()?)
}

pub fn set_transfer_rho(
    wallet_address_from: &str,
    wallet_address_to: &str,
    amount: u128,
    description: &str,
) -> Result<String, anyhow::Error> {
    let ctx = SetTransferTemplate {
        wallet_address_from: wallet_address_from.to_string(),
        wallet_address_to: wallet_address_to.to_string(),
        amount,
        description: description.to_string(),
    };
    Ok(ctx.render_once()?)
}
