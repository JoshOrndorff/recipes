//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use runtime;
use sc_service::{
	error::{Error as ServiceError}, AbstractService, Configuration,
};
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_consensus_manual_seal::{rpc, self as manual_seal};

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
macro_rules! new_full_start {
	($config:expr) => {{
		let builder = sc_service::ServiceBuilder::new_full::<
			runtime::opaque::Block, runtime::RuntimeApi, crate::service::Executor
		>($config)?
			.with_select_chain(|_config, backend| {
				Ok(sc_client::LongestChain::new(backend.clone()))
			})?
			.with_transaction_pool(|config, client, _fetcher| {
				let pool_api = sc_transaction_pool::FullChainApi::new(client.clone());
				Ok(sc_transaction_pool::BasicPool::new(config, std::sync::Arc::new(pool_api)))
			})?
			.with_import_queue(|_config, client, _select_chain, _transaction_pool| {
				Ok(sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(Box::new(client)))
			})?;

		builder
	}}
}

type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Builds a new service for a full client.
pub fn new_full(config: Configuration)
	-> Result<impl AbstractService, ServiceError>
{
	let is_authority = config.role.is_authority();
	
	let inherent_data_providers = InherentDataProviders::new();
	inherent_data_providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	let builder = new_full_start!(config);

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);

	let service = builder
		// manual-seal relies on receiving sealing requests aka EngineCommands over rpc.
		.with_rpc_extensions(|_| -> Result<RpcExtension, _> {
			let mut io = jsonrpc_core::IoHandler::default();
			io.extend_with(
				// We provide the rpc handler with the sending end of the channel to allow the rpc
				// send EngineCommands to the background block authorship task.
				rpc::ManualSealApi::to_delegate(rpc::ManualSeal::new(command_sink)),
			);
			Ok(io)
		})?
		.build()?;

	if is_authority {
		// Proposer object for block authorship.
		let proposer = sc_basic_authorship::ProposerFactory::new(
			service.client().clone(),
			service.transaction_pool(),
		);

		// Background authorship future.
		let authorship_future = manual_seal::run_manual_seal(
				Box::new(service.client()),
				proposer,
				service.client().clone(),
				service.transaction_pool().pool().clone(),
				commands_stream,
				service.select_chain().unwrap(),
				inherent_data_providers
			);

		// we spawn the future on a background thread managed by service.
		service.spawn_essential_task("manual-seal", authorship_future);
	};

	Ok(service)
}

/// Builds a new service for a light client.
pub fn new_light(_config: Configuration) -> Result<impl AbstractService, ServiceError>
{
	// FIXME The light client can work after an upstream change in Substrate
	// see: https://github.com/substrate-developer-hub/recipes/pull/238
	unimplemented!("No light client for manual seal");

	// This needs to be here or it won't compile.
	#[allow(unreachable_code)]
	new_full(_config)
}
