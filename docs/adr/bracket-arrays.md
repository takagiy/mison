# Write arrays of height 5 or more as bracket literals

## Context

Arrays and tuples separate their elements by height: `,` for 1, `;` for 2, a newline for 3 and a `----` line for 4. Heights 5 and above shared the `----` line with height 4, so some of their values collided, and the rules were deterministic only up to height 4. Such arrays are rare, so the notation for them does not need to be short.

## Decision

- An array or tuple of height 5 or more is written as a bracket literal, `[item,…,item]`, at the top level or as a field value. A field value is not parenthesized: `blocks=[[[[[1,2]]]],[[[[3,4]]]]]`.
- Inside the literal, every array or tuple is a bracket literal, whatever its height, and is written in full: a one-element array keeps its brackets.
- Elements are separated by `,` at every level, with no whitespace or newlines.
- nil is written as nothing, and so is an empty array, which is nil; `[]` never occurs.
- A record element is written in parentheses. Its field values are written as usual, matrices with newlines included.
- Within the literal, records included, `[` and `]` are special characters, so a string holding them is quoted. No new escapes are added.
- E-One is merged into E-Arr, which now allows one element (`n = 1`, `v₁ ≠ nil`). Since the separator comes from the type, a one-element array is the element with no separator, and needed no rule of its own.

## Consequences

- Gained: every height has one spelling, and evaluation is deterministic for every type.
- Gained: the literal reads like JSON, and `[` marks where it starts.
- Gained: one rule fewer for arrays, since E-One is gone.
- Lost: the grammar has a second set of special characters, and the semantics a third environment, `brk`.
- Lost: the literal spends brackets on every level, including one-element arrays.
- Lost: a record inside the literal can hold a matrix, so the literal is not always one line.
- Lost: the library has no bracket literal yet.

## Alternatives not taken

- More dashes for higher heights (`-----` for 5, `------` for 6) keeps the separator scheme, but the reader has to count dashes to know the height.
- Parentheses instead of brackets need no new special characters, but `(` already opens a record or a block in a field value.
- Making `[` and `]` special everywhere keeps one string rule, but every string such as `[INFO]` or a Markdown link would need quotes to support a rare case.
- Writing the empty array as `[]` looks like JSON, but gives nil a second spelling.
- Allowing whitespace and newlines inside the literal helps hand-written text, but gives one value many spellings and needs a rule for spaces around bare strings.
- Unwrapping one-element arrays inside the literal saves brackets, but the brackets then no longer show the height.
- Writing a record's field arrays inside the literal as bracket literals too keeps the literal on one line, but makes fields there differ from fields elsewhere.
