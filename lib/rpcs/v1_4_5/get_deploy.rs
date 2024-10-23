use serde::{Deserialize, Serialize};

use casper_types::{Deploy, DeployHash, ProtocolVersion};

use crate::types::DeployExecutionInfo;

pub(crate) const GET_DEPLOY_METHOD: &str = "info_get_deploy";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetDeployParams {
    deploy_hash: DeployHash,
    finalized_approvals: bool,
}

impl GetDeployParams {
    pub(crate) fn new(deploy_hash: DeployHash, finalized_approvals: bool) -> Self {
        GetDeployParams {
            deploy_hash,
            finalized_approvals,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to an `info_get_deploy` request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GetDeployResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy.
    pub deploy: Deploy,
    /// The map of block hash to execution result.
    pub execution_info: Option<DeployExecutionInfo>,
}
