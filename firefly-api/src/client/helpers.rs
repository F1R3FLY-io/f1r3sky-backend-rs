use std::collections::HashMap;

use anyhow::{anyhow, Context};
use blake2::digest::consts::U32;
use blake2::{Blake2b, Digest};
use prost::Message as _;
use secp256k1::{Message, Secp256k1, SecretKey};

use crate::models::casper::DeployDataProto;
use crate::models::rhoapi::expr::ExprInstance;

pub fn build_deploy_msg(key: &SecretKey, code: String) -> DeployDataProto {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let mut msg = DeployDataProto {
        term: code,
        timestamp,
        phlo_price: 1,
        phlo_limit: 500000,
        valid_after_block_number: 0,
        shard_id: "root".into(),
        ..Default::default()
    };

    let secp = Secp256k1::new();

    let mut blake = Blake2b::<U32>::new();
    blake.update(msg.encode_to_vec());
    let hash = blake.finalize();

    let signature = secp.sign_ecdsa(&Message::from_digest(hash.into()), key);

    msg.sig = signature.serialize_der().to_vec();
    msg.sig_algorithm = "secp256k1".into();

    let public_key = key.public_key(&secp);
    msg.deployer = public_key.serialize_uncompressed().into();
    msg
}

pub trait FromExpr: Sized {
    fn from(val: ExprInstance) -> anyhow::Result<Self>;
}

impl FromExpr for String {
    fn from(val: ExprInstance) -> anyhow::Result<Self> {
        match val {
            ExprInstance::GString(value) => Ok(value),
            other => Err(anyhow!("unexpected expr type: {other:?} expected GString")),
        }
    }
}

impl<T> FromExpr for Vec<T>
where
    T: FromExpr,
{
    fn from(val: ExprInstance) -> anyhow::Result<Self> {
        match val {
            ExprInstance::EListBody(list) => list
                .ps
                .into_iter()
                .map(|par| {
                    let expr = par.exprs.into_iter().next().context("missing exprs")?;
                    let expr = expr.expr_instance.context("missing expr_instance")?;
                    T::from(expr)
                })
                .collect(),
            other => Err(anyhow!(
                "unexpected expr type: {other:?} expected EListBody"
            )),
        }
    }
}

impl<T> FromExpr for HashMap<String, T>
where
    T: FromExpr,
{
    fn from(val: ExprInstance) -> anyhow::Result<Self> {
        match val {
            ExprInstance::EMapBody(map) => map
                .kvs
                .into_iter()
                .map(|pair| {
                    let key = pair
                        .key
                        .and_then(|key| key.exprs.into_iter().next())
                        .and_then(|expr| expr.expr_instance)
                        .context("missing key")?;
                    let key = FromExpr::from(key)?;

                    let value = pair
                        .value
                        .and_then(|value| value.exprs.into_iter().next())
                        .and_then(|expr| expr.expr_instance)
                        .context("missing value")?;
                    let value = FromExpr::from(value)?;

                    Ok((key, value))
                })
                .collect(),
            other => Err(anyhow!("unexpected expr type: {other:?} expected EMapBody")),
        }
    }
}

impl FromExpr for Vec<u8> {
    fn from(val: ExprInstance) -> anyhow::Result<Self> {
        match val {
            ExprInstance::GByteArray(list) => Ok(list),
            other => Err(anyhow!(
                "unexpected expr type: {other:?} expected GByteArray"
            )),
        }
    }
}

pub fn verify_rev_addr(rev_addr: &str) -> bool {
    // Decode base58 address
    let rev_bytes = match bs58::decode(rev_addr).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Convert to hex
    let rev_hex = hex::encode(&rev_bytes);

    if rev_hex.len() < 9 {
        return false;
    }

    // Split payload and checksum
    let payload = &rev_hex[..rev_hex.len() - 8];
    let checksum = &rev_hex[rev_hex.len() - 8..];

    // Decode payload hex
    let payload_bytes = match hex::decode(payload) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Calculate checksum
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(&payload_bytes);
    let hash = hasher.finalize();
    let checksum_calc = &hex::encode(&hash)[..8];

    checksum == checksum_calc
}
