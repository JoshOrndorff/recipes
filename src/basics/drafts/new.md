# New Storage Items

**an objective should be limiting `clone` invocations**

* `put_ref` and `insert_ref` from 3041
* 3071 use of `append` and `len` instead of the `mutate`
* `swap_remove` from the current `collective`
* `.retain` on `vec` is worth emphasizing as useful when removing a specific element
* **using `into_iter()` instead of `iter()` whenever you are going to consume the iterator**

* should we have a `mutate_ref` or does that make no sense whatsoever