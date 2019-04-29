# Iterate Through a Slice Rather than a Vec!

It's noticeably faster to iterate over a slice rather than a `vec!`.

* `.iter.map(|x| x.0.into()).collect`