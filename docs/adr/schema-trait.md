# Read and write through mison's own Schema trait instead of serde

## Context

An array or tuple separates its elements by the height of its type ([evaluation-semantics.md](evaluation-semantics.md)), so both sides need the height before they see the elements.

- Writing: as a field value of type `Vec<Option<Vec<u32>>>`, `[None, None]` is written `a=;`, while of type `Vec<Option<u32>>` it is `a=,`. The values alone cannot tell the two apart.
- Reading: as `Vec<Vec<u32>>`, the text `1,2` is `[[1,2]]`, not `[[1],[2]]`.

serde hands a Serializer only values, so nil and empty arrays carry no height. A Deserializer gets one hint at a time from the type, and learns an array's height only after reading its elements. Getting the height with serde therefore needs a second mechanism next to `Serialize` and `Deserialize`, and that mechanism has to agree with serde on every name and shape.

## Decision

- mison defines a trait `mison::Schema`. It has a constant `HEIGHT`, the type's height as the book defines it, and methods to write a value and to read one.
- Users get it with `#[derive(mison::Schema)]`, which comes from a proc-macro crate `mison-derive` in the same workspace and is re-exported by `mison`. The derive takes no attributes: field and variant names are the Rust identifiers.
- mison implements `Schema` for `bool`, the integer and float types, `String`, `&str`, `Vec`, arrays, `Option`, `Box`, tuples up to about 12 elements, `HashMap`, `BTreeMap` and `RangeInclusive`.
- Reading and writing are split into a core, `Writer` and `Reader`, which speaks only MISON terms (height, record, variant, nil, scalar), and the `Schema` layer, which calls it. The core stays `pub(crate)` until a second entry point needs it; entry points for serde or facet can then be added as cargo features.
- Numbers are formatted with itoa and ryu.
- Recursive types are out of scope, as the book's types have none.
- 0.2.0 removes the serde implementation and the serde dependency.

## Consequences

- Gained: the height is a compile-time constant, known for nil and empty arrays, with no schema tree, no tracing and no dummy values.
- Gained: a value can be written from its first element on, without first building a tree as the serde writer did.
- Gained: ranges and variants are types of the trait itself, not mapped from another data model.
- Gained: no name or shape has to agree between two mechanisms.
- Lost: a breaking change. Types need `#[derive(mison::Schema)]` instead of serde's derives, and a type from another crate that implements only serde has to be wrapped in a local newtype.
- Lost: no `rename`, `skip` or `flatten`. A field whose name in the text differs from its Rust name cannot be written.
- Lost: mison maintains its own derive, the std implementations, and a second crate to publish.
- Lost: recursive types cannot be read or written.

## Alternatives not taken

- Keeping serde and taking the height from the values needs nothing from users, but nil and empty arrays have no height, so such values are written with a separator their type does not read back.
- Keeping serde and tracing `Deserialize` with a probe Deserializer, as serde-reflection does, always agrees with serde. But writing then needs `Deserialize` too, types that check their values (`NonZeroU32`) fail on dummy values, and an enum has to be traced once per variant.
- Keeping serde and taking the schema from schemars' `JsonSchema` follows serde's attributes, so names agree. But users add a third derive, JSON Schema does not tell MISON's types apart (a range is an object with `start` and `end`), and a hand-written `Serialize` can drift from its schema.
- facet was the only reflection crate surveyed that gives a type's shape statically (`SHAPE`) and builds values generically (`Peek`, `Partial`), which fits reading by type. It was not taken: it is before 1.0 with breaking changes every few weeks, needs Rust 1.90 or later, brings many dependencies, and its serializer interface does not pass the type to `begin_seq`, so mison would walk `Shape` itself anyway. Once the height turned out to be one constant per type, its machinery was not needed. bevy_reflect needs `'static + Send + Sync` and has no `rename`; schemars, specta and ts-rs only describe types and cannot build values.
- Changing the spec to separate by depth from the outside fits serde's one-hint-at-a-time model, but then `1,2,3` and `1,2;3,4` cannot both be written: one of the two common shapes gets the less readable separator.
