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
| `,` `;` | separate the elements of an array |
| `(` `)` | group a nested value |
| `"` | starts a quoted string |
| `[` `]` | enclose an array of five or more dimensions |

`[` and `]` are special only inside the `[ ]` of an [array of five or more dimensions](arrays.md#arrays-of-five-or-more-dimensions).

## Escapes

Inside quotes, `"` is written `\"` and `\` is written `\\`. A newline is not escaped:

```json
{"greet":"Hello\nWorld","quote":"say \"hi\""}
```

```mison
greet="Hello
World" quote="say \"hi\""
```
