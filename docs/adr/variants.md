# Write a variant as its name with its contents in parentheses, and map Rust enums to it in one way only

## Context

MISON had no type for "one of several cases", which Rust programs use for results, statuses and shapes. serde offers four ways to represent an enum: externally tagged (the name outside the contents), internally tagged (the name as a field of the contents), adjacently tagged (the name and the contents as two fields) and untagged (no name). The last three need the reader to look at the contents before it knows the case. Formats that read by type, like bincode and postcard, reject them, and so does serde-reflection.

## Decision

- The book adds the type `variant {c₁,…,cₙ}` with `c ::= l | l : τ` and distinct labels. Its values are `(l, v)` for a case with contents and `l` for a case without. Variant types that differ only in the order of their cases are the same type.
- A case with contents is written `name(contents)`, the contents written as a field value would be: a record without its own parentheses (`rect(width=1 height=2)`), a matrix between newlines. A case without contents is the bare name (`ok`).
- A variant has height 0, like a record.
- There is no general unit type: a case without contents is the only value that is just a name.
- `#[derive(mison::Schema)]` maps each enum variant to a case: a unit variant to a case without contents, `V(T)` to `V : T`, a tuple variant to a case whose contents are a tuple, and a struct variant to a case whose contents are a record. This is the externally tagged form, and the only one offered.

## Consequences

- Gained: enums with data can be written and read.
- Gained: the name comes first, so the reader knows the case before it reads the contents, as reading by type needs.
- Gained: each value has one spelling.
- Lost: the JSON shape with the name inside the object (`{"type":"rect","width":1,"height":2}`) has no direct counterpart; its MISON form is `rect(width=1 height=2)`.
- Lost: every case is named in the text, even when the contents alone would tell the cases apart.

## Alternatives not taken

- The name as a field of a record (`type=rect width=1 height=2`) looks like the JSON many APIs use, but works only when the contents are a record, and the reader has to find that field before it can read the others.
- The name and the contents as two fields (`type=err value="not found"`) works for any contents, but writes two field names for every value.
- No name, choosing the case by the form of the contents, is the shortest, but cannot tell apart two cases whose contents have the same form, and the reader has to try every case.
- Separating the fields in the parentheses with commas, like the arguments of a call (`box(width=1, height=2)`), would give records a second spelling.
- A general unit type usable outside variants has few uses and adds notation.
