//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_client_api::RemoteBackend;
use sc_network::config::DummyFinalityProofRequestBuilder;
use sc_service::{error::Error as ServiceError, Configuration, ServiceComponents, TaskManager};
use sp_inherents::InherentDataProviders;
use std::sync::Arc;
use runtime::{self, opaque::Block, RuntimeApi};
use sp_consensus::import_queue::BasicQueue;
use sp_api::TransactionFor;

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	runtime::api::dispatch,
	runtime::native_version,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type OurServiceParams = sc_service::ServiceParams<
	Block, FullClient,
	BasicQueue<Block, TransactionFor<FullClient, Block>>,
	sc_transaction_pool::FullPool<Block, FullClient>,
	(), FullBackend,
>;

/// Returns most parts of a service. Not enough to run a full chain,
/// But enough to perform chain operations like purge-chain
pub fn new_full_params(config: Configuration) -> Result<(
	OurServiceParams,
	FullSelectChain,
	sp_inherents::InherentDataProviders,
), ServiceError> {
	let inherent_data_providers = InherentDataProviders::new();
	inherent_data_providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	let (client, backend, keystore, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let pool_api = sc_transaction_pool::FullChainApi::new(
		client.clone(), config.prometheus_registry(),
	);
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		std::sync::Arc::new(pool_api),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);

	let import_queue = sc_consensus_manual_seal::import_queue(
		Box::new(client.clone()),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	);

	let params = sc_service::ServiceParams {
		backend, client, import_queue, keystore, task_manager, transaction_pool,
		config,
		block_announce_validator_builder: None,
		finality_proof_request_builder: None,
		finality_proof_provider: None,
		on_demand: None,
		remote_blockchain: None,
		rpc_extensions_builder: Box::new(|_| ()),
	};

	Ok((
		params, select_chain, inherent_data_providers,
	))
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {

	// This variable is only used when ocw feature is enabled.
	// Suppress the warning when ocw feature is not enabled.
	#[allow(unused_variables)]
	let dev_seed = config.dev_key_seed.clone();

	let (params, select_chain, inherent_data_providers) = new_full_params(config)?;

	let (
		is_authority, prometheus_registry, client, transaction_pool
	) = {
		let sc_service::ServiceParams {
			config, client, transaction_pool, ..
		} = &params;

		(
			config.role.is_authority(),
			config.prometheus_registry().cloned(),
			client.clone(),
			transaction_pool.clone(),
		)
	};

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

	let ServiceComponents { task_manager, .. } = sc_service::build(params)?;

	if is_authority {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
		);

		let authorship_future = sc_consensus_manual_seal::run_instant_seal(
			Box::new(client.clone()),
			proposer,
			client,
			transaction_pool.pool().clone(),
			select_chain,
			inherent_data_providers,
		);

		task_manager.spawn_essential_handle().spawn_blocking("instant-seal", authorship_future);
	};

	Ok(task_manager)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
	let (client, backend, keystore, task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

	let transaction_pool_api = Arc::new(sc_transaction_pool::LightChainApi::new(
		client.clone(), on_demand.clone(),
	));
	let transaction_pool = sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		transaction_pool_api,
		config.prometheus_registry(),
		task_manager.spawn_handle(),
	);

	let import_queue = sc_consensus_manual_seal::import_queue(
		Box::new(client.clone()),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	);

	let fprb = Box::new(DummyFinalityProofRequestBuilder::default()) as Box<_>;

	sc_service::build(sc_service::ServiceParams {
		block_announce_validator_builder: None,
		finality_proof_request_builder: Some(fprb),
		finality_proof_provider: None,
		on_demand: Some(on_demand),
		remote_blockchain: Some(backend.remote_blockchain()),
		rpc_extensions_builder: Box::new(|_| ()),
		transaction_pool: Arc::new(transaction_pool),
		config, client, import_queue, keystore, backend, task_manager
	 }).map(|ServiceComponents { task_manager, .. }| task_manager)
}
