use crate::models::casper::v1::deploy_service_client::DeployServiceClient;
use crate::models::casper::v1::propose_service_client::ProposeServiceClient;
use crate::models::casper::v1::{deploy_response, propose_response, rho_data_response};
use crate::models::casper::{DataAtNameByBlockQuery, ProposeQuery};
use crate::models::rhoapi::expr::ExprInstance;
use crate::models::rhoapi::{Expr, Par};
use anyhow::{Context, anyhow};
use helpers::{FromExpr, build_deploy_msg};
use secp256k1::SecretKey;

pub mod helpers;

pub struct Client {
    wallet_key: SecretKey,
    deploy_client: DeployServiceClient<tonic::transport::Channel>,
    propose_client: ProposeServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(
        wallet_key: &str,
        deploy_service_url: &str,
        propose_service_url: &str,
    ) -> anyhow::Result<Self> {
        let wallet_key = SecretKey::from_slice(&hex::decode(wallet_key)?)?;
        let deploy_client = DeployServiceClient::connect(deploy_service_url.to_string())
            .await
            .context("failed to connect to deploy service")?;

        let propose_client = ProposeServiceClient::connect(propose_service_url.to_string())
            .await
            .context("failed to connect to propose service")?;

        Ok(Self {
            wallet_key,
            deploy_client,
            propose_client,
        })
    }

    pub async fn deploy(&mut self, code: String) -> anyhow::Result<String> {
        let msg = build_deploy_msg(&self.wallet_key, code);

        let deply_response = self
            .deploy_client
            .do_deploy(msg)
            .await
            .context("do_deploy grpc error")?
            .into_inner();

        let resp_message = deply_response
            .message
            .context("missing do_deploy responce")?;

        let message = match resp_message {
            deploy_response::Message::Result(message) => message,
            deploy_response::Message::Error(err) => {
                return Err(anyhow!("do_deploy error: {err:?}"));
            }
        };

        Ok(message)
    }

    pub async fn propose(&mut self) -> anyhow::Result<String> {
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

        let block_hash = block_hash
            .strip_prefix("Success! Block ")
            .and_then(|block_hash| block_hash.strip_suffix(" created and added."))
            .map(Into::into)
            .context("failed to extract block hash")?;
        Ok(block_hash)
    }

    pub async fn full_deploy(&mut self, code: String) -> anyhow::Result<String> {
        let _response_hash = self.deploy(code).await.context("deploy error")?;
        let block_hash = self.propose().await.context("propose error")?;
        Ok(block_hash)
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
