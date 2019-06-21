# Collisions and Overflows

* [checking for overflows/underflows](#overunder)
* [preventing collision in key-value maps](#collision)

## Checking for Overflows/Underflows <a name = "overunder"></a>

We can use the `checked` traits in [substrate-primitives](https://crates.parity.io/sr_primitives/traits/index.html) to protect against [overflow/underflow](https://medium.com/@taabishm2/integer-overflow-underflow-and-floating-point-imprecision-6ba869a99033) when incrementing/decrementing objects in our runtime. To follow the [Substrat collectable tutorial example](https://shawntabrizi.com/substrate-collectables-workshop/#/2/tracking-all-kitties?id=checking-for-overflowunderflow), we use [`checked_add()`](https://crates.parity.io/sr_primitives/traits/trait.CheckedAdd.html) to safely handle the possibility of overflow when incremementing a global counter. *Note that this check is similar to [`SafeMath`](https://ethereumdev.io/safemath-protect-overflows/) in Solidity*. 

```rust
use runtime_primitives::traits::CheckedAdd;

let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

[`ok_or()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) transforms an `Option` from `Some(value)` to `Ok(value)` or `None` to `Err(error)`. The [`?` operator](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html) facilitates error propagation. In this case, using `ok_or()` is the same as writing

```rust
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

*See the [precise arithmetic](./precision.md) section for more on safe mathematical operations*

## Collision in Key-Value Maps <a name = "collision"></a>

Often times we may intend for keys to be unique identifiers that map to a specific storage item. In this case, it is necessary to check for collisions before adding new entries. Before adding a new item to the mapping, we can check if the unique id already has an associated storage item.

In [SunshineDAO](https://github.com/4meta5/SunshineDAO), we use the hash of a proposal as the unique identifier in a `Proposals` map in the `decl_storage` block. Before adding a new proposal to the `Proposals` map, we check that the hash doesn't already have an associated value in the map. If it does, we do not allow subsequent storage changes because this would cause a key collision.

```rust
/// decl_module{} in runtime/src/dao.rs
fn propose(origin, applicant: AccountId, shares: u32, tokenTribute: BalanceOf<T>) -> Result {
    // check that a proposal associated with the given key does not already exist in the map
	ensure!(!(Self::proposals::exists(&prop.base_hash)), "Key collision :(");
    // .. more checks

    //add proposal
	Self::proposals::insert(prop.base_hash, prop);
}
```

For another example, see how the [Substrate collectables tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/2/generating-random-data?id=checking-for-collision) covers this pattern.