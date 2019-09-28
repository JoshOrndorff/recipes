# <Provided Feature>: <API Used>
> [code]`(`**`<link to the code in kitchen>`**`)`, status`[`**`<recipe status>`**`[`, [status]`[`**`<recipe status>`**`[`

*Status isn't ready yet...still thinking about how much structure should be defined and where it should be documented*

```rust
pub enum Status {
    OutdatedAndWrong,
    ExistsImprovements(u32),


}
```

```rust
match status {
    OutdatedAndWrong => replace() && (create_issue || create_pr);
    ExistsImprovements => link to 
}
```

## (optional) motivate usage with context

## show simple api usage

## (optional) work through each part

## (optional) cite other examples/references