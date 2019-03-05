## Creating a List <a name = "list"></a>

Substrate does not natively support a list type since it may encourage dangerous habits. In runtime development, list iteration is, generally speaking, evil. Unless explicitly guarded against, it will add unbounded O(N) complexity to an operation that will only charge O(1) fees. As a result, your chain becomes attackable. Instead, a list can be emulated with a mapping and a counter like so:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        AllPeopleArray get(person): map u32 => T::AccountId;
        AllPeopleCount get(num_of_people): u32;
    }
}
```

This essentially helps us store a list of people in our runtime represented by `AccountId`s.

> [LINK TO SECURITY (Checking for Overflow/Underflow)]