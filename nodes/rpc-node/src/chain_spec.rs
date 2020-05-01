use sp_core::sr25519;
use sc_service;
use runtime::{
	GenesisConfig,
	genesis::{
		account_id_from_seed,
		authority_keys_from_seed,
		testnet_genesis,
		dev_genesis,
	}
};

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
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
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				account_id_from_seed::<sr25519::Public>("Alice"),
				account_id_from_seed::<sr25519::Public>("Bob"),
				account_id_from_seed::<sr25519::Public>("Charlie"),
				account_id_from_seed::<sr25519::Public>("Dave"),
				account_id_from_seed::<sr25519::Public>("Eve"),
				account_id_from_seed::<sr25519::Public>("Ferdie"),
				account_id_from_seed::<sr25519::Public>("Alice//stash"),
				account_id_from_seed::<sr25519::Public>("Bob//stash"),
				account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				account_id_from_seed::<sr25519::Public>("Dave//stash"),
				account_id_from_seed::<sr25519::Public>("Eve//stash"),
				account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
		),
		vec![],
		None,
		None,
		None,
		None
	)
}
