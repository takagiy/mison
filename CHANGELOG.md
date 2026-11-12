# Changelog

## 0.2.0

Breaking: mison no longer uses serde.

- Types are written and read through mison's own traits `Encode` and `Decode`, derived with `#[derive(mison::Encode, mison::Decode)]` from the new crate `mison-derive`.
- A struct with named fields is a record. An enum is a variant: `Point`, `Circle(1.5)`, `Line(1,2)`, `Rect(width=1 height=2)`.
- Ranges are `RangeInclusive` only, written `start-end` with both ends included. A negative end is in parentheses: `(-3)-(-1)`.
- `to_string` returns `String`, as writing cannot fail.
- The serde dependency is removed. Numbers are written with itoa and ryu.
