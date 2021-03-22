use runtime::{
	genesis::{account_id_from_seed, dev_genesis, testnet_genesis},
	GenesisConfig, WASM_BINARY,
};
use sp_core::sr25519;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Build a Development ChainSpec
pub fn dev_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		sc_service::ChainType::Development,
		move || dev_genesis(wasm_binary),
		vec![],
		None,
		None,
		None,
		None,
	))
}

/// Build a Local Chainspec
pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		sc_service::ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
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
			)
		},
		vec![],
		None,
		None,
		None,
		None,
	))
}
