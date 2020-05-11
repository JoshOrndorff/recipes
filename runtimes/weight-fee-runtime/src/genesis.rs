//! Helper module to build a genesis configuration for the weight-fee-runtime

use super::{
	AccountId, BalancesConfig, GenesisConfig,
	SudoConfig, SystemConfig, GenericAssetConfig, WASM_BINARY, Signature,
};
use sp_core::{Pair, sr25519};
use sp_runtime::traits::{Verify, IdentifyAccount};

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPair: Pair>(seed: &str) -> TPair::Public {
	TPair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn account_id_from_seed<TPair: Pair>(seed: &str) -> AccountId where
	AccountPublic: From<TPair::Public>
{
	AccountPublic::from(get_from_seed::<TPair>(seed)).into_account()
}

pub fn dev_genesis() -> GenesisConfig {
	testnet_genesis(
		// Root Key
		account_id_from_seed::<sr25519::Pair>("Alice"),
		// Endowed Accounts
		vec![
			account_id_from_seed::<sr25519::Pair>("Alice"),
			account_id_from_seed::<sr25519::Pair>("Bob"),
			account_id_from_seed::<sr25519::Pair>("Alice//stash"),
			account_id_from_seed::<sr25519::Pair>("Bob//stash"),
		],
	)
}

/// Helper function to build a genesis configuration
pub fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		generic_asset: Some(GenericAssetConfig {
			assets: vec![
				13,
				1,
			],
			initial_balance: 10u128.pow(18 + 9), // 1 billion token with 18 decimals
			endowed_accounts: endowed_accounts.clone().into_iter().map(Into::into).collect(),
			next_asset_id: 100,
			staking_asset_id: 1,
			spending_asset_id: 1,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
