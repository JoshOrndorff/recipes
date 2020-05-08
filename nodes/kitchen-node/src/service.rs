//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

// use std::sync::Arc;
// use sc_client::LongestChain;
// use sc_client_api::ExecutorProvider;
use sc_service::{error::{Error as ServiceError}, AbstractService, Configuration};
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
// use sc_network::config::DummyFinalityProofRequestBuilder;

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

/// Builds a new service for a full client.
pub fn new_full(config: Configuration)
	-> Result<impl AbstractService, ServiceError>
{
	// This variable is only used when ocw feature is enabled.
	// Suppress the warning when ocw feature is not enabled.
	#[allow(unused_variables)]
	let dev_seed = config.dev_key_seed.clone();

	// This isn't great. It includes the timestamp inherent in all blocks
	// regardless of runtime.
	let inherent_data_providers = InherentDataProviders::new();
	inherent_data_providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	let builder = new_full_start!(config);
	let service = builder.build()?;

	// Initialize seed for signing transaction using off-chain workers
	#[cfg(feature = "ocw")]
	{
		if let Some(seed) = dev_seed {
			service
				.keystore()
				.write()
				.insert_ephemeral_from_seed_by_type::<runtime::offchain_demo::crypto::Pair>(
					&seed,
					runtime::offchain_demo::KEY_TYPE,
				)
				.expect("Dev Seed should always succeed.");
		}
	}


	let proposer = sc_basic_authorship::ProposerFactory::new(
		service.client().clone(),
		service.transaction_pool(),
	);

	let authorship_future = sc_consensus_manual_seal::run_instant_seal(
		Box::new(service.client()),
		proposer,
		service.client().clone(),
		service.transaction_pool().pool().clone(),
		service.select_chain().ok_or(ServiceError::SelectChainRequired)?,
		inherent_data_providers
	);

	service.spawn_essential_task("instant-seal", authorship_future);

	Ok(service)
}

/// Builds a new service for a light client.
pub fn new_light(_config: Configuration)
	-> Result<impl AbstractService, ServiceError>
{

	// FIXME The light client can work after an upstream change in Substrate
	// see: https://github.com/substrate-developer-hub/recipes/pull/238
	unimplemented!("No light client for manual seal");
	#[allow(unreachable_code)]
	new_full(_config)

	// let inherent_data_providers = InherentDataProviders::new();
	//
	// ServiceBuilder::new_light::<runtime::opaque::Block, runtime::RuntimeApi, Executor>(config)?
	// 	.with_select_chain(|_config, backend| {
	// 		Ok(LongestChain::new(backend.clone()))
	// 	})?
	// 	.with_transaction_pool(|config, client, fetcher| {
	// 		let fetcher = fetcher
	// 			.ok_or_else(|| "Trying to start light transaction pool without active fetcher")?;
	// 		let pool_api = sc_transaction_pool::LightChainApi::new(client.clone(), fetcher.clone());
	// 		let pool = sc_transaction_pool::BasicPool::with_revalidation_type(
	// 			config, Arc::new(pool_api), sc_transaction_pool::RevalidationType::Light,
	// 		);
	// 		Ok(pool)
	// 	})?
	// 	.with_import_queue_and_fprb(|_config, client, _backend, _fetcher, select_chain, _tx_pool| {
	// 		let finality_proof_request_builder =
	// 			Box::new(DummyFinalityProofRequestBuilder::default()) as Box<_>;
	//
	// 		let import_queue = sc_consensus_manual_seal::import_queue::<_, sc_client_db::Backend<_>>(
	// 			Box::new(client)
	// 		);
	//
	// 		Ok((import_queue, finality_proof_request_builder))
	// 	})?
	// 	.with_finality_proof_provider(|client, backend| {
	// 		Ok(Arc::new(()) as _)
	// 	})?
	// 	.build()
}
