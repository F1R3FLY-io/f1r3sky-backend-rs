#![allow(
    non_camel_case_types,
    clippy::large_enum_variant,
    clippy::enum_variant_names
)]

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod servicemodelapi {
    tonic::include_proto!("servicemodelapi");
}

pub mod rhoapi {
    tonic::include_proto!("rhoapi");
}

pub mod casper {
    tonic::include_proto!("casper");

    pub mod v1 {
        tonic::include_proto!("casper.v1");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub cost: u64,
    pub errored: bool,
    pub system_deploy_error: Option<String>,
}

impl TransferResult {
    pub fn new(block_data: Value) -> anyhow::Result<Self> {
        let cost = block_data
            .get("cost")
            .and_then(Value::as_u64)
            .context("cost not found")?;
        let errored = block_data
            .get("errored")
            .and_then(Value::as_bool)
            .context("errored not found")?;
        let system_deploy_error = block_data
            .get("system_deploy_error")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        Ok(Self {
            cost,
            errored,
            system_deploy_error,
        })
    }
}
