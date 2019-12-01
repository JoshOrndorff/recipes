# <Provided Feature>: <API Used>
*[code]`(`**`<link to the code in kitchen>`**`)`, [status]`(`**`<recipe status>`**`)`*

*brainstorming*
```rust, ignore
pub enum Status<T : Recipe>  {
    OutdatedAndWrong(T),
    ExistsImprovements(T),
    MotivatesNewSection(T),
}
...
match status {
    OutdatedAndWrong => replace() && link(create_issue || create_pr);
    ExistsImprovements => link(issues || prs || tutorials || code);
    MotivatesNewSection => link(issues || prs || tutorials || code);
    _ => empty_label_at_top;
}
```

## (optional) motivate usage with context

## show simple api usage

## (optional) work through each part

## (optional) cite other examples/references