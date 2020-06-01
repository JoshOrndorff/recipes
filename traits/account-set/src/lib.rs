#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::collections::btree_set::BTreeSet;

/// Types that implement the AccountSet trait are able to supply a set of accounts
/// The trait is generic over the notion of Account used.
pub trait AccountSet {
	type AccountId;

	fn accounts() -> BTreeSet<Self::AccountId>;
}
