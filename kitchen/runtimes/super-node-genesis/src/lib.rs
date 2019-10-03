use super_node_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY,
};
use babe_primitives::{AuthorityId as BabeId};
use grandpa_primitives::{AuthorityId as GrandpaId};

pub fn testnet_genesis(initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		babe: Some(BabeConfig {
			authorities: initial_authorities.iter().map(|x| (x.3.clone(), 1)).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
	}
}
