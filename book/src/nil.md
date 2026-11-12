# Nil

Nil is the absence of a value. It is written as nothing.

```json
null
```

```mison

```

```json
{"name":null}
```

```mison
name=
```

```json
{"answers":[0,null,42]}
```

```mison
answers=0,,42
```

## Values that hold nothing are nil

MISON has one way to say "nothing". These values are all nil:

- an empty array,
- an empty map,
- an array whose only element is nil.

```json
{"empty":[],"none":{},"void":[[[[[[[[[[]]]]]]]]]]}
```

```mison
empty= none= void=
```
