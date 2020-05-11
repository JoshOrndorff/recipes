//! Helper module to build a genesis configuration for the weight-fee-runtime

use super::{
	AccountId, BalancesConfig, GrandpaConfig,
	SudoConfig, SystemConfig, GenesisConfig, WASM_BINARY,
};

use sp_finality_grandpa::{AuthorityId as GrandpaId};

/// Helper function to build a genesis configuration
pub fn testnet_genesis(initial_authorities: Vec<GrandpaId>,
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
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.clone(), 1)).collect(),
		}),
	}
}
