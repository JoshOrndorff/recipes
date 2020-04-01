//! Helper module to build a genesis configuration for the pow-runtime

use super::{
	AccountId, BalancesConfig, GenesisConfig,
	SudoConfig, SystemConfig, WASM_BINARY,
};

/// This helper function is unlike the testnet_genesis functions in other runtimes.
/// It does not accept a list of initial authorities because there is no concept of
/// authorities in a purely PoW chain.
pub fn testnet_genesis(
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
	}
}
