# Safe Math

We can use the `checked` traits in
[substrate-primitives](https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/traits/index.html) to protect against
[overflow/underflow](https://medium.com/@taabishm2/integer-overflow-underflow-and-floating-point-imprecision-6ba869a99033)
when incrementing/decrementing objects in our runtime. To follow the
[Substrate collectable tutorial example](https://shawntabrizi.com/substrate-collectables-workshop/#/2/tracking-all-kitties?id=checking-for-overflowunderflow),
use [`checked_add()`](https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/traits/trait.CheckedAdd.html) to safely
handle the possibility of overflow when incremementing a global counter. _Note that this check is
similar to [`SafeMath`](https://ethereumdev.io/safemath-protect-overflows/) in Solidity_.

```rust, ignore
use runtime_primitives::traits::CheckedAdd;

let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

[`ok_or()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) transforms an
`Option` from `Some(value)` to `Ok(value)` or `None` to `Err(error)`. The
[`?` operator](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)
facilitates error propagation. In this case, using `ok_or()` is the same as writing

```rust, ignore
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

## todo

-   `?` for error propagation
-   Permill, Perbill, Fixed64 types for large arithmetic
-   `quantization` benchmarks in the `treasury` tests to verify that large arithmetic stays in a
    comfortable error bound
-   ADD BACK IN NEW RECIPE: `collide` and the question of whether maps prevent key collisions? could
    discuss `sort`, `sort_unstable`, and the ordering traits here...
