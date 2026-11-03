# Arrays

An array, or a tuple, is a sequence of values. Its elements are separated by `,`.

```json
{"ingredients":["spam","ham","eggs"]}
```

```mison
ingredients=spam,ham,eggs
```

## Arrays at the top level

A top-level array separates its elements with newlines instead of `,`:

```json
[{"name":"Ash","buddy":"Pikachu"},{"name":"Dawn","buddy":"Piplup"},{"name":"Brock","buddy":"Croagunk"}]
```

```mison
name=Ash buddy=Pikachu
name=Dawn buddy=Piplup
name=Brock buddy=Croagunk
```

## Nested arrays

A nested array is written in parentheses:

```json
{"matrix":[[1,2,3],[4,5,6]]}
```

```mison
matrix=(1,2,3),(4,5,6)
```

## Arrays with one element

An array with a single element is written as that element, however deeply it is nested. The reader knows how many arrays to wrap the value back in.

| JSON | MISON |
|---|---|
| `{"teams":[["Ann","Ben"]]}` | `teams=Ann,Ben` |
| `{"teams":[[["Ann"]]]}` | `teams=Ann` |
| `{"teams":[["Ann"],["Ben"]]}` | `teams=(Ann),(Ben)` |

The last row shows why elements of longer arrays keep their parentheses: without them, `[["Ann"],["Ben"]]` and `[["Ann","Ben"]]` would both be written `Ann,Ben`, and not even the reader's shape could tell them apart.
