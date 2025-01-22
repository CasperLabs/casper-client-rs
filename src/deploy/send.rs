use clap::{ArgMatches, Command};

use async_trait::async_trait;

use casper_client::cli::CliError;

use super::creation_common::{self, DisplayOrder};
use crate::{command::ClientCommand, common, Success};

pub struct SendDeploy;

static DEPRECATION_WARNING: &str = r#"
#################################### WARNING ####################################
#                                                                               #
#       send-deploy subcommand is deprecated in favor of send-transaction       #
#                    and will be removed in a future release                    #
#                                                                               #
#################################################################################
"#;

#[async_trait]
impl ClientCommand for SendDeploy {
    const NAME: &'static str = "send-deploy";
    const ABOUT: &'static str =
        "[DEPRECATED: use `send-transaction` instead] Read a previously-saved deploy from a file and send it to the network for execution";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(creation_common::speculative_exec::arg())
            .arg(creation_common::input::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        // show deprecation warning for each use of `send-deploy` subcommand
        println!("{DEPRECATION_WARNING}");

        let is_speculative_exec = creation_common::speculative_exec::get(matches);
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let input_path = creation_common::input::get(matches);

        if is_speculative_exec {
            casper_client::cli::speculative_send_deploy_file(
                maybe_rpc_id,
                node_address,
                verbosity_level,
                input_path,
            )
            .await
            .map(Success::from)
        } else {
            casper_client::cli::send_deploy_file(
                maybe_rpc_id,
                node_address,
                verbosity_level,
                input_path,
            )
            .await
            .map(Success::from)
        }
    }
}
