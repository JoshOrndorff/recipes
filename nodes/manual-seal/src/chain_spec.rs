use sp_core::sr25519;
use sc_service;
use runtime::{
	GenesisConfig,
	genesis::{
		account_id_from_seed,
		testnet_genesis,
		dev_genesis,
	}
};

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate `ChainSpec` type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn dev_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		sc_service::ChainType::Development,
		dev_genesis,
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
			account_id_from_seed::<sr25519::Pair>("Alice"),
			vec![
				account_id_from_seed::<sr25519::Pair>("Alice"),
				account_id_from_seed::<sr25519::Pair>("Bob"),
				account_id_from_seed::<sr25519::Pair>("Charlie"),
				account_id_from_seed::<sr25519::Pair>("Dave"),
				account_id_from_seed::<sr25519::Pair>("Eve"),
				account_id_from_seed::<sr25519::Pair>("Ferdie"),
				account_id_from_seed::<sr25519::Pair>("Alice//stash"),
				account_id_from_seed::<sr25519::Pair>("Bob//stash"),
				account_id_from_seed::<sr25519::Pair>("Charlie//stash"),
				account_id_from_seed::<sr25519::Pair>("Dave//stash"),
				account_id_from_seed::<sr25519::Pair>("Eve//stash"),
				account_id_from_seed::<sr25519::Pair>("Ferdie//stash"),
			],
		),
		vec![],
		None,
		None,
		None,
		None
	)
}
