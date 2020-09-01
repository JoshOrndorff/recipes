use crate::chain_spec;
use crate::cli::Cli;
use crate::service;
use sc_cli::{SubstrateCli, RuntimeVersion, Role, ChainSpec};
use sc_service::PartialComponents;
use crate::service::new_partial;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"RPC Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/substrate-developer-hub/recipes/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2019
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::dev_config()),
			"" | "local" => Box::new(chain_spec::local_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(subcommand) => {
			let runner = cli.create_runner(subcommand)?;
			runner.run_subcommand(subcommand, |config| {
				let PartialComponents { client, backend, task_manager, import_queue, .. }
					= new_partial(&config)?;
				Ok((client, backend, import_queue, task_manager))
			})
		}
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| match config.role {
				Role::Light => service::new_light(config),
				_ => service::new_full(config),
			})
		}
	}
}
