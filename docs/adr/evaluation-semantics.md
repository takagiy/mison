# Define reading as a type-directed, big-step evaluation with no error value

## Context

The book's formal definition (Syntax, Types and values, Semantics) states what a MISON text means under a type. Writing it down forced choices the prose chapters left open: what happens when a text does not fit its type, which spellings of a value are allowed, and how nil, ranges and records behave at the edges. This record keeps those choices and why, so the book can state only the rules.

## Decision

Evaluation is a relation `e : τ ⇓ v` defined by big-step rules, one judgment for every position. Where a rule needs parentheses around an element or field value, it decides from the element's type (`composite-type(τ)`), not from the shape of the expression.

| Issue | Decision | Why |
|---|---|---|
| A text that does not fit its type | No derivation; there is no error value | Keeps the rules minimal. "Does not fit" and "no rule applies" are the same thing |
| `-0` | Not an `Int` literal | Makes the integer interpretation 𝒩 a bijection |
| Values of `float` | ℚ | Evaluation is exact on paper; rounding is left to implementations |
| The empty string | Only `""` | Leaves the empty expression ε free to mean nil |
| Nested nullable `τ??` | Ruled out by the type grammar (`?` attaches only to a non-null σ) | nil has exactly one spelling, ε, so `τ??` could not be told from `τ?` |
| `range` with `a > b` | Evaluates to the empty set | A `range` always evaluates when the type fits |
| E-Lift and ε | E-Lift requires `e ≠ ε` | Keeps evaluation deterministic; as a result `⟨nil⟩` cannot be written under `array τ?` |
| Nullable record fields | Cannot be omitted; written `a=` | The type alone fixes the form of the expression |
| Duplicate labels | No derivation | JSON implementations disagree on them |
| Unneeded parentheses | Not allowed | Keeps one way to write each value |
| Whitespace around commas | Not allowed | Keeps one way to write each value |
| A trailing newline | Not part of the grammar; `1⏎` is the lines `1` and ε | Removes the ambiguity between a trailing newline and a last empty line without extra rules |

## Consequences

- Gained: evaluation is deterministic. For any `(e, τ)` there is at most one `v`, which follows by induction on expressions and types:
  - each scalar type has one rule, and the interpretations 𝒩, 𝒟 and 𝒮 are functions;
  - E-Nil and E-Lift are told apart by whether the expression is ε;
  - E-Arr, E-Tup and E-Rec are told apart by the form of the type, and the syntax splits an expression into its elements in exactly one way;
  - E-Lines and E-Lines-Tup apply only to a text of two or more lines, the other rules only to a single line.
- Gained: the book can state the rules without exceptions or error cases.
- Lost: a printer (value to text) needs canonical spellings, because evaluation is not injective. 𝒩 is a bijection and a non-empty `range` has one literal, but the empty range has many, and 𝒟 (`1`, `1.0`, `10e-1`) and 𝒮 (`abc`, `"abc"`) map several literals to one value.
- Lost: the library reads more than the specification allows. It skips one trailing newline, accepts newlines inside parentheses as field separators, and skips extra whitespace around record fields. These texts have no derivation in the specification.

## Alternatives not taken

- Error values in the result (`e : τ ⇓ error`) would let the rules report why a text fails, but every rule would need error cases, and none of them would change what a valid text means.
- Deciding parentheses from the expression's shape (parenthesize lists and records) would remove types from that rule, but `[["Ann"],["Ben"]]` would then be written `Ann,Ben`, the same as `[["Ann","Ben"]]`.
- A trailing newline as part of the top level (`Toplevel ::= … Newline?`) matches the library, but makes `1⏎` both one line and two, so it needs a separate top-level judgment to stay deterministic.

## References

Consulted during the design. The rules themselves are MISON's own; no earlier design of the same kind was found.

- Ye & Oliveira, Type-Directed Operational Semantics for Gradual Typing: semantics in which type annotations drive evaluation
- CSCI 3155, CU Boulder, Operational Semantics: big-step judgments, and syntax versus values
- Kahn, Hoffmann & Li, Big-Stop Semantics: big-step semantics cannot express divergence
- UNSW COMP3161 lecture notes: the shape of a determinism proof
- RFC 8259 and Errata 7600: the JSON number grammar, and object order and duplicate names
- RFC 8785 Errata 7920: handling of `-0`
- A YAML ↔ JSON converter (Qiita): YAML's Norway problem
- Nested nullables in Kotlin (gist); Temper, Nullable types and flat option equivalence: nested nullables
- @httpland/range-parser: reversed ranges in HTTP Range
- Rust by Example (Japanese), Repetition; Data.List.NonEmpty: repetition notation and non-empty lists
- Cornell CS3110 Recitation 2: the `*` notation for tuple types
