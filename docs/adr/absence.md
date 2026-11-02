# Treat `None`, `Some` around a value, and empty arrays and objects as one absent value

## Context

MISON writes `None`, an empty array and an empty object as nothing, to keep tool output short. Rust can nest them in ways that nothing cannot tell apart, even with the type: `Some(None)` and `None` in an `Option<Option<u32>>`, or `Some(vec![])` and `None` in an `Option<Vec<u32>>` at the top level, where there is no key to leave out. Settling each case with its own notation made the rules longer at every such case.

## Decision

MISON keeps data rather than Rust values, and some different Rust values are the same data. There is one absent value, called nil, and:

1. `Some(x)` is the same as `x`.
2. `None` is nil.
3. An empty array and an empty object are nil.
4. An array whose only element is nil is nil, at any depth.

A text reads back as one value of the data it was written from: an `Option` holding nil reads back as `None`, and `[None]` as an empty array. An array of two or more elements keeps its length, even when its elements are nil.

## Consequences

- Gained: nil is always written as nothing, and parentheses only ever mark where an array or object starts and ends. Every value reads back as the same data, with no exceptions to list.
- Lost: a value can read back as a different Rust value. `Some(vec![])` and `Some(None)` read back as `None`, and `[None]` and `[[None]]` as empty arrays, so data that tells "not set" from "set to nothing", or one absent element from none, cannot be written in MISON without a field saying which.

## Alternatives not taken

- A notation for `None`, such as `~`, keeps every Rust value apart and makes nil elements easier to spot, but adds a second way to write absence and makes `~` need quotes as a string.
- Writing an array whose only element is nil as `()`, and grouping that once more one level up (`(())`), keeps `[None]` apart from `[]`, but adds a notation used nowhere else for a case whose single position rarely means anything.
- Treating every array whose elements are all nil as nil drops the parentheses rule as well, but loses the length of arrays such as `[None, None]`, whose positions often mean something.
- Rejecting values that would read back differently keeps every round trip exact, but makes ordinary data unwritable.
