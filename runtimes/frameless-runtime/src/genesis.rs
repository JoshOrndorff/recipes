//! Helper module to build a genesis configuration for the frameless-runtime
//! This does nothing interesting. It is here so the runtime canwork with the
//! `basic-pow` node without having to hack that node.

use super::{
	AccountId, GenesisConfig,
};

pub fn testnet_genesis(
	_root_key: AccountId,
	_endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
		GenesisConfig
}
