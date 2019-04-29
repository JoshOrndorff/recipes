# Iterate Through a Slice Rather than a Vec!

It's noticeably faster to iterate over a slice rather than a `vec!`.

* `.iter.map(|x| x.0.into()).collect`

### MISC NOTES

* MAPS USE BLAKE2
* STORAGE VALUES USE TWOX

BLAKE2 IS 6X SLOWER THAN TWOX
but if you have keys in your map, that can be manipulated from the outside; an attacker could try to create hash collisions.
Moreover, for the map, you can set the hasher you want to use => look up the correct hash for your type in the metadata.