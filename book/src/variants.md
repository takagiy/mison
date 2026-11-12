# Variants

A variant is one of several cases, chosen by name. A case with contents is written as its name followed by the contents in parentheses:

```json
{"type":"rect","w":1,"h":2}
```

```mison
rect(w=1 h=2)
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

A case without contents is written as its name alone:

```json
{"result":{"type":"ok"}}
```

```mison
result=ok
```
