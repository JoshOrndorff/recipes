//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::ExecutorProvider;
use sc_consensus::LongestChain;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_finality_grandpa::{
	self, FinalityProofProvider as GrandpaFinalityProofProvider, StorageAndProofProvider,
};
use sc_service::{error::Error as ServiceError, AbstractService, Configuration, ServiceBuilder};
use sha3pow::MinimalSha3Algorithm;
use sp_inherents::InherentDataProviders;
use std::sync::Arc;
use std::time::Duration;

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
		let mut import_setup = None;
		let inherent_data_providers = crate::service::build_inherent_data_providers()
			.expect("Inherent data providers are present.");

		let builder = sc_service::ServiceBuilder::new_full::<
			runtime::opaque::Block,
			runtime::RuntimeApi,
			crate::service::Executor,
		>($config)?
		.with_select_chain(|_config, backend| Ok(sc_consensus::LongestChain::new(backend.clone())))?
		.with_transaction_pool(|config, client, _fetcher, prometheus_registry| {
			let pool_api = sc_transaction_pool::FullChainApi::new(client.clone());
			Ok(sc_transaction_pool::BasicPool::new(
				config,
				std::sync::Arc::new(pool_api),
				prometheus_registry,
			))
		})?
		.with_import_queue(
			|_config, client, mut select_chain, _transaction_pool, spawn_task_handle, registry| {
				let select_chain = select_chain
					.take()
					.ok_or_else(|| sc_service::Error::SelectChainRequired)?;
				let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
					client.clone(),
					&(client.clone() as std::sync::Arc<_>),
					select_chain.clone(),
				)?;

				let justification_import = grandpa_block_import.clone();

				let pow_block_import = sc_consensus_pow::PowBlockImport::new(
					grandpa_block_import,
					client.clone(),
					sha3pow::MinimalSha3Algorithm,
					0, // check inherents starting at block 0
					Some(select_chain),
					inherent_data_providers.clone(),
				);

				let import_queue = sc_consensus_pow::import_queue(
					Box::new(pow_block_import.clone()),
					Some(Box::new(justification_import)),
					None,
					sha3pow::MinimalSha3Algorithm,
					inherent_data_providers.clone(),
					spawn_task_handle,
					registry,
				)?;

				import_setup = Some((pow_block_import, grandpa_link));

				Ok(import_queue)
			},
		)?;

		(builder, import_setup, inherent_data_providers)
		}};
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<impl AbstractService, ServiceError> {
	let role = config.role.clone();
	// let force_authoring = config.force_authoring;
	let name = config.network.node_name.clone();
	let disable_grandpa = config.disable_grandpa;

	let (builder, mut import_setup, inherent_data_providers) = new_full_start!(config);

	let (block_import, grandpa_link) = import_setup.take().expect(
		"Link Half and Block Import are present for Full Services or setup failed before. qed",
	);

	let service = builder
		.with_finality_proof_provider(|client, backend| {
			let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
			Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
		})?
		.build()?;

	if role.is_authority() {
		let proposer =
			sc_basic_authorship::ProposerFactory::new(
				service.client(),
				service.transaction_pool(),
				service.prometheus_registry().as_ref(),
			);

		let client = service.client();
		let select_chain = service
			.select_chain()
			.ok_or(ServiceError::SelectChainRequired)?;

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		sc_consensus_pow::start_mine(
			Box::new(block_import),
			client,
			MinimalSha3Algorithm,
			proposer,
			None, // TODO Do I need some grandpa preruntime digests?
			500,  // Rounds
			service.network(),
			std::time::Duration::new(2, 0),
			Some(select_chain),
			inherent_data_providers.clone(),
			can_author_with,
		);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() {
		Some(service.keystore())
	} else {
		None
	};

	let grandpa_config = sc_finality_grandpa::Config {
		gossip_duration: Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		is_authority: role.is_network_authority(),
	};

	let enable_grandpa = !disable_grandpa;
	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network: service.network(),
			inherent_data_providers,
			telemetry_on_connect: Some(service.telemetry_on_connect_stream()),
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry: service.prometheus_registry(),
			shared_voter_state: sc_finality_grandpa::SharedVoterState::empty(),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		service.spawn_essential_task(
			"grandpa-voter",
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	} else {
		sc_finality_grandpa::setup_disabled_grandpa(
			service.client(),
			&inherent_data_providers,
			service.network(),
		)?;
	}

	Ok(service)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<impl AbstractService, ServiceError> {
	let inherent_data_providers = InherentDataProviders::new();

	ServiceBuilder::new_light::<Block, RuntimeApi, Executor>(config)?
		.with_select_chain(|_config, backend| Ok(LongestChain::new(backend.clone())))?
		.with_transaction_pool(|config, client, fetcher, prometheus_registry| {
			let fetcher = fetcher
				.ok_or_else(|| "Trying to start light transaction pool without active fetcher")?;
			let pool_api = sc_transaction_pool::LightChainApi::new(client, fetcher);
			let pool = sc_transaction_pool::BasicPool::with_revalidation_type(
				config,
				Arc::new(pool_api),
				prometheus_registry,
				sc_transaction_pool::RevalidationType::Light,
			);
			Ok(pool)
		})?
		.with_import_queue_and_fprb(
			|_config,
			 client,
			 backend,
			 fetcher,
			 select_chain,
			 _tx_pool,
			 spawn_task_handle,
			 registry| {
				let fetch_checker = fetcher
					.map(|fetcher| fetcher.checker().clone())
					.ok_or_else(|| {
						"Trying to start light import queue without active fetch checker"
					})?;
				let grandpa_block_import = sc_finality_grandpa::light_block_import(
					client.clone(),
					backend,
					&(client.clone() as Arc<_>),
					Arc::new(fetch_checker),
				)?;

				let finality_proof_import = grandpa_block_import.clone();
				let finality_proof_request_builder =
					finality_proof_import.create_finality_proof_request_builder();

				let pow_block_import = sc_consensus_pow::PowBlockImport::new(
					grandpa_block_import,
					client,
					MinimalSha3Algorithm,
					0, // check inherents starting at block 0
					select_chain,
					inherent_data_providers.clone(),
				);

				let import_queue = sc_consensus_pow::import_queue(
					Box::new(pow_block_import),
					None,
					Some(Box::new(finality_proof_import)),
					MinimalSha3Algorithm,
					inherent_data_providers,
					spawn_task_handle,
					registry,
				)?;

				Ok((import_queue, finality_proof_request_builder))
			},
		)?
		.with_finality_proof_provider(|client, backend| {
			let provider = client as Arc<dyn StorageAndProofProvider<_, _>>;
			Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, provider)) as _)
		})?
		.build()
}
