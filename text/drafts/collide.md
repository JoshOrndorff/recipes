# Checking for Collisions

When using unique identifiers that depend on an absence of key collision, it is best to check for key collision before adding new entries. 

For example, assume an object's hash the unique identifier (key) in a map defined in the `decl_storage` block. Before adding a new `(key, value)` pair to the map, verify that the key (hash) does not already have an associated value in the map.

```rust
fn insert_value(origin, hash: Hash, value: u32) {
    // check that key doesn't have an associated value
    ensure!( !(Self::map::exists(&hash)), "key already has an associated value" );

    // add key-value pair
    <Map<T>>::insert(hash, value);
}
```

 If it does, it is necessary to decide between the new item and the existing item to prevent an inadvertent key collision.

*See how the [Substrate Collectables Tutorial](https://shawntabrizi.com/substrate-collectables-workshop/#/2/generating-random-data?id=checking-for-collision) covers this pattern.*