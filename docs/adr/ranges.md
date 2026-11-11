# Write every range as its two ends joined by `-`, and leave whether the end is included to the reader

## Context

Line ranges and other spans are common in tool output, and `1-2` is shorter and easier to read than `1,2` or `(start=1 end=2)`. serde has no range type of its own: Rust's `Range` and `RangeInclusive` reach a serializer as structs named `Range` and `RangeInclusive` with fields `start` and `end`.

## Decision

A struct that serde names `Range` or `RangeInclusive` is written `start-end` and read back from it. There is one notation: whether `end` is included is not written, and is up to the program reading the value, which knows which of the two types it reads into. Every other pair, including a two-element array, is written as an array. A negative end is written in parentheses: `(-3)-(-1)`.

## Consequences

- Gained: ranges are short, and programs keep using the standard types; nothing in their data depends on MISON.
- Lost: a struct of another crate that happens to be named `Range` with fields `start` and `end` is written as a range too. A reader of the text cannot tell half-open from closed ranges.

## Alternatives not taken

- Keeping the sign of a negative end as is (`-3--1`) needs no parentheses, but the separator and the sign run together.
- A `mison::Range` type would make the meaning explicit, but would make programs depend on MISON in their data types.
- Writing every two-element array of numbers as a range needs no types at all, but turns genuine pairs, such as coordinates, into ranges.
- A second notation for half-open ranges, such as `1..3`, says more, but the program reading already knows which range type it holds.
