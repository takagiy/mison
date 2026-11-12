# Write every range as its two ends joined by `-`, including the end

## Context

Line ranges and other spans are common in tool output, and `1-2` is shorter and easier to read than `1,2` or `(start=1 end=2)`. Rust holds them in `Range`, which leaves out the end, and `RangeInclusive`, which includes it.

## Decision

A range is written `start-end` and includes its end. A negative end is written in parentheses: `(-3)-(-1)`. Every other pair, including a two-element array, is written as an array.

mison implements `Schema` for `RangeInclusive` only. A program holding a `Range` `a..b` writes it as `a..=b-1`.

## Consequences

- Gained: ranges are short, and programs keep using a standard type; nothing in their data depends on MISON.
- Gained: the text alone says which integers a range covers.
- Lost: a program holding `Range` values converts them before writing and after reading.

## Alternatives not taken

- Leaving whether the end is included to the program reading the range lets `Range` and `RangeInclusive` both be written as they are, but the text alone no longer says which integers a range covers.
- Writing a `Range` `a..b` as `a-(b-1)` keeps `Range` usable, but works only for integers, and an empty range such as `0..0` needs a negative end in the text.
- Keeping the sign of a negative end as is (`-3--1`) needs no parentheses, but the separator and the sign run together.
- A `mison::Range` type would make the meaning explicit, but would make programs depend on MISON in their data types.
- Writing every two-element array of numbers as a range needs no types at all, but turns genuine pairs, such as coordinates, into ranges.
- A second notation for half-open ranges, such as `1..3`, says more, but adds a notation for a case that `RangeInclusive` already covers.
