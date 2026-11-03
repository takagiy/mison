# Nil

Nil is the absence of a value. It is written as nothing.

| JSON | MISON |
|---|---|
| `null` | (empty text) |
| `{"name":null}` | `name=` |
| `{"answers":[0,null,42]}` | `answers=0,,42` |

## Values that hold nothing are nil

MISON has one way to say "nothing". These values are all nil:

- an empty array,
- an empty record,
- an array whose only element is nil, at any depth.

```json
{"empty":[],"none":{},"void":[[[[[[[[[[]]]]]]]]]]}
```

```mison
empty= none= void=
```
