# Define reading as a type-directed, big-step evaluation with no error value

## Context

The book's formal definition (Syntax, Types and values, Semantics) states what a MISON text means under a type. Writing it down forced choices the prose chapters left open: what happens when a text does not fit its type, which spellings of a value are allowed, and how nil, ranges and records behave at the edges. This record keeps those choices and why, so the book can state only the rules.

## Decision

Evaluation is defined by big-step rules over one judgment `ρ ⊢ e : τ ⇓ v`, whose environment `ρ ∈ {top, inner}` says whether the expression is the whole text. An array or tuple of height up to 4 separates its elements by its height, and a higher one is a bracket literal ([bracket-arrays.md](bracket-arrays.md)), so parentheses mark only a record or map used as an element or a field value, and blocks of lines in a field.

| Issue | Decision | Why |
|---|---|---|
| A text that does not fit its type | No derivation; there is no error value | Keeps the rules minimal. "Does not fit" and "no rule applies" are the same thing |
| `-0` | Not an `Int` literal | Makes the integer interpretation 𝒩 a bijection |
| Values of `float` | ℚ ∪ {+∞, −∞, NaN}, written `inf`, `-inf` and `NaN` | Evaluation is exact on paper; rounding is left to implementations. The infinities and NaN are floating-point values that programs hold |
| The empty string | Only `""` | Leaves the empty expression ε free to mean nil |
| Nested nullable `τ??` | Ruled out by the type grammar (`?` attaches only to a non-null σ) | nil has exactly one spelling, ε, so `τ??` could not be told from `τ?` |
| `range` with `a > b` | Evaluates to the empty set | A `range` always evaluates when the type fits |
| E-Lift and ε | E-Lift requires `e ≠ ε` | Keeps evaluation deterministic |
| An array of a single nil | Not a value: `⟦array⁺ τ⟧` leaves out `⟨nil⟩`, and E-Arr and E-BArr require `v₁ ≠ nil` when `n = 1` | `[nil]` is nil (the Nil chapter). Leaving out `⟨nil⟩` at every level also leaves out `⟨⟨nil⟩⟩` and deeper |
| Nullable record fields | Cannot be omitted; written `a=` | The type alone fixes the form of the expression |
| Duplicate labels | No derivation | JSON implementations disagree on them |
| Unneeded parentheses | Not allowed | Keeps one way to write each value |
| Separators | By the height of the array or tuple: `,` for 1, `;` for 2, a newline for 3, a `----` line for 4; higher ones are bracket literals | Each level that occurs in practice has its own separator, so values stay apart: `[[[1],[2]]]` is `1;2` while `[[[1]],[[2]]]` puts `1` and `2` on separate lines, and `[[nil,nil]]` is `,` while `[nil,nil]` is `;` |
| An array with one element | E-Arr with `n = 1`: the element alone, with no separator | The separator comes from the type, so a one-element array needs no rule of its own. The array's own separator never appears in a text of its element type, so `n = 1` and `n ≥ 2` cannot both apply |
| Parentheses | Around an element or field value whose text is a record or map, unless the array has one element or its elements are separated by newlines, and around a field value that holds a newline (a block) | Arrays no longer need them: their separators already say where each element ends |
| The top level | In `top`, arrays and tuples of height 1 and 2 also separate their elements by newlines; from height 3 on, `top` and `inner` agree. Sub-expressions are evaluated in `inner` | Keeps one record per line, as edo writes its reports, without mistaking the top-level newline for the height-3 separator |
| Whitespace around commas | Not allowed | Keeps one way to write each value |
| A trailing newline | Not part of the grammar; `1⏎` is the lines `1` and ε | Removes the ambiguity between a trailing newline and a last empty line without extra rules |

## Consequences

- Gained: evaluation is deterministic. For any `(e, τ)` there is at most one `v`, which follows by induction on expressions and types:
  - each scalar type has one rule, and the interpretations 𝒩, 𝒟 and 𝒮 are functions;
  - E-Nil and E-Lift are told apart by whether the expression is ε;
  - E-Arr, E-Tup and E-Rec are told apart by the form of the type, and E-Arr and E-Tup from E-BArr and E-BTup by the height and the environment, and the syntax splits an expression into its elements in exactly one way;
  - E-Arr with `n = 1` and with `n ≥ 2` are told apart by the separator: an array's separator does not occur in a text of its element type;
  - in `top`, heights 1 and 2 are separated by newlines too, while a one-element array evaluates its element in `inner`, where no newline separates heights below 3.
- Gained: the book can state the rules without exceptions or error cases.
- Lost: a printer (value to text) needs canonical spellings, because evaluation is not injective. 𝒩 is a bijection and a non-empty `range` has one literal, but the empty range has many, and 𝒟 (`1`, `1.0`, `10e-1`) and 𝒮 (`abc`, `"abc"`) map several literals to one value.
- Lost: a line that is just the string `----`, inside an array of height 4, reads as a separator; a writer must quote it.
- Lost: `;` is a special character, so a bare string cannot hold it.
- Lost: the library writes the earlier notation (parentheses around array elements, no `;`, no bracket literal) and has to change.
- Lost: the library reads more than the specification allows. It skips one trailing newline, accepts newlines inside parentheses as field separators, and skips extra whitespace around record fields. These texts have no derivation in the specification.

## Alternatives not taken

- Counting the separators, as Julia does with `;`, `;;` and `;;;`, would cover every height, but a run of separators already means empty elements in MISON (`1,,2`), so nil between outer elements would need a spelling of its own.
- Never writing a one-element array of composite values as its element would remove every collision, but would add parentheses to common texts such as `teams=(Ann,Ben)`.
- Parenthesizing a one-element array only when writing it as its element would collide (E-Wrap) keeps the old notation, but needs a condition on the shape of the text in both E-One and the writer.
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
