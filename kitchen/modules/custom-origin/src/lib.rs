#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "128"]

pub mod dao;

pub use dao::{Call, Event, Module, RawEvent, Trait};

// test scaffolding
#[cfg(test)]
mod tests {
    pub use super::*;
    pub use dao;
    pub use primitives::{Blake2Hasher, H256};
    pub use runtime_io::with_externalities;
    pub use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{impl_outer_dispatch, impl_outer_event, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {
            dao<T>
        }
    }

    impl_outer_event! {
        pub enum Event for Test {
            dao<T>,
        }
    }

    impl_outer_dispatch! {
        pub enum Call for Test where origin: Origin {
            dao::DAO,
        }
    }

    // Workaround for https://github.com/rust-lang/rust/issues/26925
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type Log = DigestItem; // not part of system::Trait on master
    }
    impl dao::Trait for Test {
        type Origin = Origin;
        type Proposal = Call;
        type Event = Event;
    }

    // builds a genesis storage key/value store according to our desired mockup.
    pub fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    pub type DAO = dao::Module<Test>;
    pub type System = system::Module<Test>;
}
