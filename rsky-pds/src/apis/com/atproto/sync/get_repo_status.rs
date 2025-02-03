use crate::account_manager::helpers::account::{
    format_account_status, AccountStatus, FormattedAccountStatus,
};
use crate::apis::com::atproto::repo::assert_repo_availability;
use crate::apis::ApiError;
use crate::repo::aws::s3::S3BlobStore;
use crate::repo::ActorStore;
use anyhow::Result;
use aws_config::SdkConfig;
use rocket::serde::json::Json;
use rocket::State;
use rsky_lexicon::com::atproto::sync::{GetRepoStatusOutput, RepoStatus};

async fn inner_get_repo(did: String, s3_config: &State<SdkConfig>) -> Result<GetRepoStatusOutput> {
    let account = assert_repo_availability(&did, true).await?;
    let FormattedAccountStatus { active, status } = format_account_status(Some(account));

    let mut rev: Option<String> = None;
    if active {
        let actor_store = ActorStore::new(did.clone(), S3BlobStore::new(did.clone(), s3_config));
        let storage_guard = actor_store.storage.read().await;
        let root = storage_guard.get_root_detailed()?;
        rev = Some(root.rev);
    }

    Ok(GetRepoStatusOutput {
        did,
        active,
        status: match status {
            None => None,
            Some(status) => match status {
                AccountStatus::Active => None,
                AccountStatus::Takendown => Some(RepoStatus::Takedown),
                AccountStatus::Suspended => Some(RepoStatus::Suspended),
                AccountStatus::Deleted => None,
                AccountStatus::Deactivated => Some(RepoStatus::Deactivated),
            },
        },
        rev,
    })
}

/// Get the hosting status for a repository, on this server.
/// Expected to be implemented by PDS and Relay.
#[rocket::get("/xrpc/com.atproto.sync.getRepoStatus?<did>")]
pub async fn get_repo_status(
    did: String,
    s3_config: &State<SdkConfig>,
) -> Result<Json<GetRepoStatusOutput>, ApiError> {
    match inner_get_repo(did, s3_config).await {
        Ok(res) => Ok(Json(res)),
        Err(error) => {
            eprintln!("@LOG: ERROR: {error}");
            Err(ApiError::RuntimeError)
        }
    }
}
