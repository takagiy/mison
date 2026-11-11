# Records

A record is a set of named fields, each a key and its value. A field is written `key=value`, and the fields are separated by a space.

```json
{"name":"bob","email":"bob@example.com"}
```

```mison
name=bob email=bob@example.com
```

A key is quoted by the same rules as a [string](strings.md):

```json
{"first name":"Alice","last name":"Smith"}
```

```mison
"first name"=Alice "last name"=Smith
```

## Nested records

A nested record is written in parentheses to avoid ambiguity:

```json
{"from":{"city":"Tokyo","airport":"HND"},"to":{"city":"Paris","airport":"CDG"}}
```

```mison
from=(city=Tokyo airport=HND) to=(city=Paris airport=CDG)
```
