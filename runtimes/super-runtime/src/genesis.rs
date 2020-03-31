//! Helper module to build a genesis configuration for the super-runtime

use super::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY,
};

use sp_consensus_babe::{AuthorityId as BabeId};
use sp_finality_grandpa::{AuthorityId as GrandpaId};

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
