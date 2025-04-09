use anyhow::{Context, anyhow};
use helpers::{FromExpr, build_deploy_msg};
use reqwest::Client as HttpClient;
use secp256k1::SecretKey;
use serde_json::Value;

use crate::models::casper::v1::deploy_service_client::DeployServiceClient;
use crate::models::casper::v1::propose_service_client::ProposeServiceClient;
use crate::models::casper::v1::{deploy_response, propose_response, rho_data_response};
use crate::models::casper::{DataAtNameByBlockQuery, ProposeQuery};
use crate::models::rhoapi::expr::ExprInstance;
use crate::models::rhoapi::{Expr, Par};

pub mod helpers;

pub struct Client {
    wallet_key: SecretKey,
    deploy_client: DeployServiceClient<tonic::transport::Channel>,
    propose_client: ProposeServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(
        wallet_key: SecretKey,
        deploy_service_url: String,
        propose_service_url: String,
    ) -> anyhow::Result<Self> {
        let deploy_client = DeployServiceClient::connect(deploy_service_url)
            .await
            .context("failed to connect to deploy service")?;

        let propose_client = ProposeServiceClient::connect(propose_service_url)
            .await
            .context("failed to connect to propose service")?;

        Ok(Self {
            wallet_key,
            deploy_client,
            propose_client,
        })
    }

    pub async fn full_deploy(&mut self, code: String) -> anyhow::Result<String> {
        let msg = build_deploy_msg(&self.wallet_key, code);

        let resp = self
            .deploy_client
            .do_deploy(msg)
            .await
            .context("do_deploy grpc error")?
            .into_inner()
            .message
            .context("missing do_deploy responce")?;

        match resp {
            deploy_response::Message::Result(_) => (),
            deploy_response::Message::Error(err) => {
                return Err(anyhow!("do_deploy error: {err:?}"));
            }
        }

        let resp = self
            .propose_client
            .propose(ProposeQuery { is_async: false })
            .await
            .context("propose grpc error")?
            .into_inner()
            .message
            .context("missing propose responce")?;

        let block_hash = match resp {
            propose_response::Message::Result(block_hash) => block_hash,
            propose_response::Message::Error(err) => return Err(anyhow!("propose error: {err:?}")),
        };

        block_hash
            .strip_prefix("Success! Block ")
            .and_then(|block_hash| block_hash.strip_suffix(" created and added."))
            .map(Into::into)
            .context("failed to extract block hash")
    }

    pub async fn get_channel_value<T>(&mut self, hash: String, channel: String) -> anyhow::Result<T>
    where
        T: FromExpr,
    {
        let mut par = Par::default();
        par.exprs.push(Expr {
            expr_instance: Some(ExprInstance::GString(channel)),
        });

        let resp = self
            .deploy_client
            .get_data_at_name(DataAtNameByBlockQuery {
                par: Some(par),
                block_hash: hash,
                use_pre_state_hash: false,
            })
            .await
            .context("get_data_at_name grpc error")?
            .into_inner()
            .message
            .context("missing get_data_at_name responce")?;

        let payload = match resp {
            rho_data_response::Message::Payload(payload) => payload,
            rho_data_response::Message::Error(err) => {
                return Err(anyhow!("get_data_at_name error: {err:?}"));
            }
        };

        let par = payload
            .par
            .into_iter()
            .last()
            .context("missing par in get_data_at_name")?;
        let expr = par
            .exprs
            .into_iter()
            .next()
            .context("missing exprs in get_data_at_name")?;
        let expr = expr
            .expr_instance
            .context("missing expr_instance in get_data_at_name")?;

        T::from(expr)
    }
}

pub struct ReadNodeClient {}

impl ReadNodeClient {
    pub fn new() -> Self {
        Self {}
    }

    fn read_node_api(self) -> String {
        let read_node_url = "http://localhost:40413";
        let read_node_method = "explore-deploy";
        format!("{}/api/{}", read_node_url, read_node_method)
    }

    pub async fn get_data(self, code: String) -> Result<Value, anyhow::Error> {
        // Get the URL from the `read_node_api` function
        let url = self.read_node_api();

        // Create an HTTP client
        let http_client = HttpClient::new();

        let response = http_client
            .post(&url)
            .body(code)
            .header("Content-Type", "text/plain")
            .send()
            .await?;

        // Ensure the request was successful
        if response.status().is_success() {
            // Parse the JSON response
            let json_data: Value = response
                .json()
                .await
                .context("failed to parse response as JSON")?;
            Ok(json_data)
        } else {
            Err(anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ))
        }
    }
}
