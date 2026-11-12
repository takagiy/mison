# Variants

A variant is a value that can be of several kinds. It is written as a combination of a tag for its kind and the data it contains:

```json
{"type":"rect","width":1,"height":2}
```

```mison
rect(width=1 height=2)
```

```json
{"type":"circle","radius":1}
```

```mison
circle(radius=1)
```

The contents can be any value, not only a record:

```json
{"result":{"type":"err","value":"not found"}}
```

```mison
result=err("not found")
```

A kind without contents is written as its tag alone:

```json
{"result":{"type":"ok"}}
```

```mison
result=ok
```
