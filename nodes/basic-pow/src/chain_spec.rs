use sp_core::{Pair, sr25519};
use sc_service;
use sp_runtime::traits::{Verify, IdentifyAccount};
use runtime::{AccountId, GenesisConfig, Signature, genesis::testnet_genesis};

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate `ChainSpec` type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPair: Pair>(seed: &str) -> TPair::Public {
	TPair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPair: Pair>(seed: &str) -> AccountId where
	AccountPublic: From<TPair::Public>
{
	AccountPublic::from(get_from_seed::<TPair>(seed)).into_account()
}

pub fn dev_config() -> ChainSpec{
	 ChainSpec::from_genesis(
		"Development",
		"dev",
		sc_service::ChainType::Development,
		|| testnet_genesis(
			get_account_id_from_seed::<sr25519::Pair>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Pair>("Alice"),
				get_account_id_from_seed::<sr25519::Pair>("Bob"),
				get_account_id_from_seed::<sr25519::Pair>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		sc_service::ChainType::Local,
		|| testnet_genesis(
			get_account_id_from_seed::<sr25519::Pair>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Pair>("Alice"),
				get_account_id_from_seed::<sr25519::Pair>("Bob"),
				get_account_id_from_seed::<sr25519::Pair>("Charlie"),
				get_account_id_from_seed::<sr25519::Pair>("Dave"),
				get_account_id_from_seed::<sr25519::Pair>("Eve"),
				get_account_id_from_seed::<sr25519::Pair>("Ferdie"),
				get_account_id_from_seed::<sr25519::Pair>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Pair>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None
	)
}
