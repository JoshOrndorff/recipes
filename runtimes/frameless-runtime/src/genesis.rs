//! Helper module to build a genesis configuration for the frameless-runtime
//! This does nothing interesting. It is here so the runtime canwork with the
//! `basic-pow` node without having to hack that node.

use super::{
	AccountId, GenesisConfig, Signature,
};

use sp_core::Pair;
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
	GenesisConfig
}

pub fn testnet_genesis(
	_root_key: AccountId,
	_endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
		GenesisConfig
}
