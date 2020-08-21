//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{ExecutorProvider, RemoteBackend };
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_network::config::DummyFinalityProofRequestBuilder;
use sc_service::{error::Error as ServiceError, Configuration, ServiceComponents, TaskManager};
use sha3pow::MinimalSha3Algorithm;
use sp_inherents::InherentDataProviders;
use std::sync::Arc;
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
type OurBlockImport = sc_consensus_pow::PowBlockImport<Block, Arc<FullClient>, FullClient, FullSelectChain, MinimalSha3Algorithm>;

pub fn build_inherent_data_providers() -> Result<InherentDataProviders, ServiceError> {
	let providers = InherentDataProviders::new();

	providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	Ok(providers)
}

/// Returns most parts of a service. Not enough to run a full chain,
/// But enough to perform chain operations like purge-chain
pub fn new_full_params(config: Configuration) -> Result<(
	OurServiceParams,
	FullSelectChain,
	sp_inherents::InherentDataProviders,
	OurBlockImport,
), ServiceError> {
	let inherent_data_providers = build_inherent_data_providers()?;

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

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		sha3pow::MinimalSha3Algorithm,
		0, // check inherents starting at block 0
		Some(select_chain.clone()),
		inherent_data_providers.clone(),
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import.clone()),
		None,
		None,
		sha3pow::MinimalSha3Algorithm,
		inherent_data_providers.clone(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	)?;

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
		params, select_chain, inherent_data_providers, pow_block_import
	))
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	let (params, select_chain, inherent_data_providers, block_import) = new_full_params(config)?;

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

	let ServiceComponents { task_manager, network, .. } = sc_service::build(params)?;

	if is_authority {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
		);

		// The number of rounds of mining to try in a single call
		let rounds = 500;

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		sc_consensus_pow::start_mine(
			Box::new(block_import),
			client,
			MinimalSha3Algorithm,
			proposer,
			None, // No preruntime digests
			rounds,
			network,
			std::time::Duration::new(2, 0),
			// Choosing not to supply a select_chain means we will use the client's
			// possibly-outdated metadata when fetching the block to mine on
			Some(select_chain),
			inherent_data_providers,
			can_author_with,
		);
	}

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

	let select_chain = sc_consensus::LongestChain::new(backend.clone());
	let inherent_data_providers = build_inherent_data_providers()?;

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		sha3pow::MinimalSha3Algorithm,
		0, // check inherents starting at block 0
		Some(select_chain),
		inherent_data_providers.clone(),
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import),
		None,
		None,
		sha3pow::MinimalSha3Algorithm,
		inherent_data_providers,
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	)?;

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
