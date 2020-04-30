//! Helper module to build a genesis configuration for the super-runtime

use super::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature,
};
use sp_core::{Pair, Public, sr25519};
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_finality_grandpa::{AuthorityId as GrandpaId};
use sp_runtime::traits::{Verify, IdentifyAccount};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId) {
	(
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
	)
}

/// Build a Development ChainSpec
pub fn dev_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		sc_service::ChainType::Development,
		|| testnet_genesis(vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		],
		true),
		vec![],
		None,
		None,
		None,
		None
	)
}

/// Helper function to build a genesis configuration
pub fn testnet_genesis(initial_authorities: Vec<(BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		babe: Some(BabeConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone(), 1)).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
	}
}
