# Leave out a dynamic `Value` type until a user needs one

## Context

serde_json has `serde_json::Value`, which holds any JSON value and is both built by hand (`json!`) and read from text. A MISON `Value` could only be written, never read, since reading needs a type. Its remaining uses are building output without declaring types, and holding a range in such output, which `serde_json::Value` cannot.

## Decision

The first version has no `Value` type. Output whose shape varies is written from structs with `Option` fields, and ranges from `Range` or `RangeInclusive` fields.

## Consequences

- Gained: a smaller library, and output whose shape is written down as a type, which is what reading it back needs anyway.
- Lost: a program has to declare a struct for output it would rather build ad hoc; adding `Value` later is possible without breaking anything.

## Alternatives not taken

- A write-only `Value` with a `Range` variant, which was planned at first, has no user yet; it can be added when the first one, edo, finds structs too heavy for its reports.
