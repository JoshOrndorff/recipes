/// A very simple substrate runtime
use support::{
	decl_module, decl_event, decl_storage, StorageValue, StorageMap,
	dispatch::Result, ensure
};
use system::ensure_signed;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!{
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
	{
		ValueSet(AccountId, u64),
		ValueGot(AccountId, u64),
		UserValueGot(AccountId, u64),
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as HelloWorld {
		pub LastValue get(fn last_value): u64;
		pub UserValue get(fn user_value): map T::AccountId => u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn set_value(origin, value: u64) -> Result {
			let setter = ensure_signed(origin)?;
			LastValue::put(value);
			UserValue::<T>::insert(&setter, value);
			Self::deposit_event(RawEvent::ValueSet(setter, value));
			Ok(())
		}

		pub fn get_last_value(origin) -> Result {
			let getter = ensure_signed(origin)?;
			ensure!(LastValue::exists(), "no value stored in LastValue storage item");
			let value = LastValue::get();
			Self::deposit_event(RawEvent::ValueGot(getter, value));
			Ok(())
		}

		pub fn get_user_value(origin) -> Result {
			let getter = ensure_signed(origin)?;
			ensure!(UserValue::<T>::exists(&getter), "value for user's AccountId does not exist");
			let value = UserValue::<T>::get(&getter);
			Self::deposit_event(RawEvent::ValueGot(getter, value));
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	// TODO: add mock runtime here
}
