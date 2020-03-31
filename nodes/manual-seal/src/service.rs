//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;
use sc_client::LongestChain;
use runtime::{self, GenesisConfig, opaque::Block, RuntimeApi};
use sc_service::{error::{Error as ServiceError}, AbstractService, Configuration, ServiceBuilder};
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_network::{config::DummyFinalityProofRequestBuilder};
use sc_consensus_manual_seal::rpc::{ManualSealApi, ManualSeal};

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);

pub fn build_inherent_data_providers() -> Result<InherentDataProviders, ServiceError> {
	let providers = InherentDataProviders::new();

	providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	Ok(providers)
}

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
			.with_import_queue(|_config, client, select_chain, _transaction_pool| {
				Ok(sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(Box::new(client)))
			})?;

		builder
	}}
}
type RpcExtension = jsonrpc_core::IoHandler<()>;
/// Builds a new service for a full client.
pub fn new_full(config: Configuration, manual_seal: bool)
	-> Result<impl AbstractService, ServiceError>
{
	let inherent_data_providers = InherentDataProviders::new();
	inherent_data_providers.register_provider(sp_timestamp::InherentDataProvider);

	let (tx, rx) = futures::channel::mpsc::channel(1000);
	let builder = new_full_start!(config);
	let service = builder
		.with_rpc_extensions(|_| -> Result<RpcExtension, _> {
			let mut io = jsonrpc_core::IoHandler::default();
			if manual_seal {
				io.extend_with(
					ManualSealApi::to_delegate(ManualSeal::new(tx)),
				);
			}

			Ok(io)
		})?
		.build()?;

	let proposer = sc_basic_authorship::ProposerFactory::new(
		service.client().clone(),
		service.transaction_pool(),
	);

	let future = if manual_seal {
		println!("Running Manual-seal");

		futures::future::Either::Left(sc_consensus_manual_seal::run_manual_seal(
			Box::new(service.client()),
			proposer,
			service.client().clone(),
			service.transaction_pool().pool().clone(),
			stream.unwrap(),
			service.select_chain().unwrap(),
			inherent_data_providers
		))
	} else {
		println!("Running Instant-seal");

		futures::future::Either::Right(sc_consensus_manual_seal::run_instant_seal(
			Box::new(service.client()),
			proposer,
			service.client().clone(),
			service.transaction_pool().pool().clone(),
			service.select_chain().unwrap(),
			inherent_data_providers
		))
	};

	service.spawn_essential_task(
		if manual_seal { "manual-seal" } else { "instant-seal" },
		future
	);


	Ok(service)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<impl AbstractService, ServiceError>
{
	new_full(config, true)
}
