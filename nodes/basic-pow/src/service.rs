//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{ExecutorProvider, RemoteBackend };
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_network::config::DummyFinalityProofRequestBuilder;
use sc_service::{error::Error as ServiceError, Configuration, PartialComponents, TaskManager};
use sha3pow::MinimalSha3Algorithm;
use sp_inherents::InherentDataProviders;
use std::{sync::Arc, time::Duration};
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
#[allow(clippy::type_complexity)]
pub fn new_partial(config: &Configuration) -> Result<
	PartialComponents<
		FullClient, FullBackend, FullSelectChain,
		BasicQueue<Block, TransactionFor<FullClient, Block>>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		sc_consensus_pow::PowBlockImport<Block, Arc<FullClient>, FullClient, FullSelectChain, MinimalSha3Algorithm, impl sp_consensus::CanAuthorWith<Block>>,
	>,
ServiceError> {
	let inherent_data_providers = build_inherent_data_providers()?;

	let (client, backend, keystore, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);

	let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		sha3pow::MinimalSha3Algorithm,
		0, // check inherents starting at block 0
		select_chain.clone(),
		inherent_data_providers.clone(),
		can_author_with,
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

	Ok(PartialComponents {
		client, backend, import_queue, keystore, task_manager, transaction_pool,
		select_chain, inherent_data_providers,
		other: pow_block_import,
	})
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	let sc_service::PartialComponents {
		client, backend, mut task_manager, import_queue, keystore, select_chain, transaction_pool,
		inherent_data_providers,
		other: pow_block_import,
	} = new_partial(&config)?;

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: None,
			block_announce_validator_builder: None,
			finality_proof_request_builder: None,
			finality_proof_provider: None,
		})?;

	let is_authority = config.role.is_authority();
	let prometheus_registry = config.prometheus_registry().cloned();
	let telemetry_connection_sinks = sc_service::TelemetryConnectionSinks::default();

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore,
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		telemetry_connection_sinks,
		rpc_extensions_builder: Box::new(|_, _| ()),
		on_demand: None,
		remote_blockchain: None,
		backend, network_status_sinks, system_rpc_tx, config,
	})?;

	if is_authority {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		// Parameter details: https://substrate.dev/rustdocs/v2.0.0/sc_consensus_pow/fn.start_mining_worker.html
		let (worker, future) = sc_consensus_pow::start_mining_worker(
			Box::new(pow_block_import),	// block_import: BoxBlockImport
			client,											// client: Arc<C>
			// Choosing not to supply a select_chain means we will use the client's
			//   possibly-outdated metadata when fetching the block to mine on.
			select_chain,								// select_chain: S
			MinimalSha3Algorithm,				// algorithm: Algorithm
			proposer,										// env: E
      network.clone(),						// sync_oracle: SO
			None, 											// pre_runtime: Option<Vec<u8>>
			inherent_data_providers,		// inherent_data_providers: InherentDataProviders
			Duration::from_secs(5),			// timeout: Duration[Q: what is this for?]
			Duration::from_secs(10),		// build_time: Duration[Q: what is this for?]
			can_author_with,						// can_author_with: CAW
		);

		// Q: where should I poll the future getting above?
	}

	network_starter.start_network();
	Ok(task_manager)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
	let (client, backend, keystore, mut task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
		on_demand.clone(),
	));

	let select_chain = sc_consensus::LongestChain::new(backend.clone());
	let inherent_data_providers = build_inherent_data_providers()?;
	// FixMe #375
	let _can_author_with =
		sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		sha3pow::MinimalSha3Algorithm,
		0, // check inherents starting at block 0
		select_chain,
		inherent_data_providers.clone(),
		// FixMe #375
		sp_consensus::AlwaysCanAuthor,
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

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: Some(on_demand.clone()),
			block_announce_validator_builder: None,
			finality_proof_request_builder: Some(fprb),
			finality_proof_provider: None,
		})?;

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		remote_blockchain: Some(backend.remote_blockchain()),
		transaction_pool,
		task_manager: &mut task_manager,
		on_demand: Some(on_demand),
		rpc_extensions_builder: Box::new(|_, _| ()),
		telemetry_connection_sinks: sc_service::TelemetryConnectionSinks::default(),
		config,
		client,
		keystore,
		backend,
		network,
		network_status_sinks,
		system_rpc_tx,
	 })?;

	 network_starter.start_network();

	 Ok(task_manager)
}
