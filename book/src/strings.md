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
""
```

```mison
""
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

`[` and `]` are special only inside an [array of five or more dimensions](arrays.md#arrays-of-five-or-more-dimensions).

## Escapes

Inside quotes, `"` is written `\"` and `\` is written `\\`:

```json
"say \"hi\""
```

```mison
"say \"hi\""
```

A newline need not be escaped:

```json
"Hello\nWorld"
```

```mison
"Hello
World"
```
