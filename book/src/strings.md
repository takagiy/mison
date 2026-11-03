# Strings

A string is a sequence of characters representing text. It is written as is.

```json
"alice@example.com"
```

```mison
alice@example.com
```

An empty string is written `""`.

```json
{"nickname":""}
```

```mison
nickname=""
```

## Quoted strings

A string that holds a special character is written in double quotes.

```json
"Hello, World"
```

```mison
"Hello, World"
```

The special characters are:

| Character | What it means outside quotes |
|---|---|
| whitespace (space, tab, newline, ...) | separates fields and lines |
| `=` | separates a key from its value |
| `,` | separates the elements of an array |
| `(` `)` | group a nested value |
| `"` | starts a quoted string |

## Escapes

Inside quotes, `"` is written `\"` and `\` is written `\\`. A newline is not escaped:

```json
{"greet":"Hello\nWorld","quote":"say \"hi\""}
```

```mison
greet="Hello
World" quote="say \"hi\""
```
