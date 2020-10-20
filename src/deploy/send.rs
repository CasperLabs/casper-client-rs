use clap::{App, ArgMatches, SubCommand};

use super::creation_common::{self, DisplayOrder};
use crate::{command::ClientCommand, common};

pub struct SendDeploy;

impl<'a, 'b> ClientCommand<'a, 'b> for SendDeploy {
    const NAME: &'static str = "send-deploy";
    const ABOUT: &'static str = "Sends a deploy to the network for execution";

    fn build(display_order: usize) -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(creation_common::input::arg())
    }

    fn run(matches: &ArgMatches<'_>) {
        let rpc = common::rpc(matches);

        let input_path = creation_common::input::get(matches);
        let response = rpc
            .send_deploy_file(&input_path)
            .unwrap_or_else(|error| panic!("response error: {}", error));
        println!(
            "{}",
            serde_json::to_string_pretty(&response).expect("should encode to JSON")
        );
    }
}
