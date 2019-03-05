# Security
> add relevant links to these sections and other sections to prevent collisions

**Best Practices**
* [Condition-Oriented Programming](./declarative.md)
    * consider abstracting into it's own folder? depends how many patterns
* [Preventing Collisions in Key-Value Map](#collisions)
* [Checking for Underflow/Overflow](#flow)

**Economic Security**
* []()

## Best Practices

* [Preventing Collisions for Key-Value Maps](#collisions)
* [Checking for Underflow/Overflow](#flow)

### Preventing Collisions for Key-Value Maps <a name = "collisions"></a>

Often times we may intend for keys to be unique identifiers that map to a specific storage item. In this case, it is necessary to check for collisions before adding new entries. Before adding a new item to the mapping, we can check if the unique id already has an associated storage item.

```
ensure!(!<Value<T>>::exists(new_id), "This new id already exists");
```

### Checking for Underflow/Overflow <a name = "flow"></a>

Before [adding an item to a list](#LINKTOSTORAGE), we should check that we can successfully increment the existing object. Here's how that would look:

```
let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;

```

`checked_sub` may be used in the event that we are decrementing something and must check for underflow.

## Economic Security

> Gautam's note, Shawn's note