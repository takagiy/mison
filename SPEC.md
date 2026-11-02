# MISON specification

MISON (minimal and simple object notation) writes the data JSON writes, with fewer characters. It is meant for readers that pay for every character they read, such as language models reading a tool's output.

MISON gets shorter by leaving out what the reader can work out: the quotes around plain strings, the braces around objects, the colons after keys. The cost is that a MISON text does not say what type each value has. `age=20` may hold the number 20 or the string `"20"`.

## The type is the schema

A MISON text is read back through the type it was written from. In Rust, that is the type passed to `mison::from_str`; serde tells the reader, field by field, whether to expect a number, a string, an array or an object, and the reader parses the text accordingly.

So reading needs a type. Reading into a value whose type is decided by the text itself, such as `serde_json::Value` or an enum marked `#[serde(untagged)]`, is an error. Writing always works.

## Values that are the same

MISON keeps data, not Rust values: some different Rust values are the same data, and read back as one of them. There is one absent value, called nil, and:

1. `Some(x)` is the same as `x`.
2. `None` is nil.
3. An empty array and an empty object are nil.
4. An array whose only element is nil is nil, at any depth: `[None]`, `[vec![]]` and `[[None]]` are nil.

So `Some(None)`, `Some(vec![])` and `None` are all nil, and an `Option` holding any of them reads back as `None`. A program that needs to tell "not set" from "set to an empty list" has to say so in another field.

An array of two or more elements keeps its length: `[None, None]` is an array holding nil twice, not nil.

## Values

### Strings

A string is written as is, unless it would be misread. It is written in double quotes when it is empty or holds any of these characters:

| Character | Why it would be misread |
|---|---|
| whitespace (space, tab, newline, ...) | separates the fields of an object and the elements of a top-level array |
| `=` | separates a key from its value |
| `,` | separates array elements |
| `(` `)` | group a nested value |
| `"` | starts a quoted string |

Inside quotes, `"` is written `\"` and `\` is written `\\`. Nothing else is escaped: a newline is written as a newline.

`-` and `:` do not need quotes. The type tells a range from a string, so file names and dates such as `a-b.rs` and `2026-11-02T23:06:00+09:00` are written as they are.

### Numbers

An integer is written as is: `20`, `-3`.

A float is written in the shortest form that reads back as the same value. A float equal to an integer is written without a fraction (`1`, not `1.0`), since the type says it is a float. Not-a-number and the infinities are written `NaN`, `inf` and `-inf`.

### Booleans

`true` and `false`.

### Ranges

A range is written as its two ends joined by `-`: `1-2`, `-3--1`. Rust's `Range` and `RangeInclusive` are both written this way; whether the end is included is up to the program reading it.

A range binds more tightly than `,`, so an array of ranges needs no parentheses: `1-2,3-4`.

Any other pair, such as an array `[1, 2]`, is an array and is written `1,2`.

### Objects

An object is written as its fields, each `key=value`, separated by a space:

```
name=John email=john@example.com age=20
```

Keys follow the same quoting rules as strings: `"key=key"=value`.

An object that is the value of a field is written in parentheses:

```
child=(name=foo age=10) parent=(name=bar age=42)
```

A field whose value is missing (`None`) is left out.

### Arrays

An array is written as its elements separated by `,`: `1,2,3`.

An array with a single element is written as that element, however deeply it is nested: `[[1,2,3]]` is written `1,2,3`, and `[[[1]]]` is written `1`. The type says how many arrays to wrap the value back in.

The elements of an array with two or more elements are written in parentheses when they are arrays or objects, and are not unwrapped:

| Value | Written |
|---|---|
| `[[1,2,3],[4,5,6]]` | `(1,2,3),(4,5,6)` |
| `[[1],[2]]` | `(1),(2)` |
| `[[1,2]]` | `1,2` |

Without this rule, `[[1],[2]]` and `[[1,2]]` would both be written `1,2`, and not even their type could tell them apart.

An element that is nil (`None`, or an empty array or object) is written as nothing: `1,,3`.

### Empty arrays and objects

nil is written as nothing: a field holding an empty array is written `array=`, and an element that is nil is written as nothing between its separators. A field holding `None` is left out instead, since serde reads a missing field as `None` but would reject a missing array.
### Enums

A variant without a value is written as its name. A variant with a value is written like an object with one field, named after the variant:

| Value | JSON | MISON |
|---|---|---|
| `Mode::Replace` | `"Replace"` | `Replace` |
| `Shape::Circle(2)` | `{"Circle":2}` | `Circle=2` |
| `Shape::Rect(1, 2)` | `{"Rect":[1,2]}` | `Rect=1,2` |
| `Shape::Rect { w: 1, h: 2 }` | `{"Rect":{"w":1,"h":2}}` | `Rect=(w=1 h=2)` |

As the value of a field, a variant with a value is in parentheses like any object: `shape=(Circle=2)`.

### The top level

An object at the top level is written on one line, without parentheses.

An array with two or more elements at the top level is written one element per line, without parentheses:

```
foo=1 bar=1,1
foo=2 bar=2,2
```

Only newlines outside quotes separate the lines, so a string holding a newline may span lines.

## Examples

| JSON | MISON |
|---|---|
| `{"name":"John","email":"john@example.com","age":20}` | `name=John email=john@example.com age=20` |
| `{"key=key":"value"}` | `"key=key"=value` |
| `{"array":[1,2,3]}` | `array=1,2,3` |
| `{"array":[]}` | `array=` |
| `{"array":[[1,2,3],[4,5,6]]}` | `array=(1,2,3),(4,5,6)` |
| `{"array":[[1,2,3]]}` | `array=1,2,3` |
| `{"array":[[[[1]]]]}` | `array=1` |
| `{"range":1..=2}` (a range, not an array) | `range=1-2` |
| `{"ranges":[1..=2,3..=4]}` | `ranges=1-2,3-4` |
| `{"child":{"name":"foo","age":10},"parent":{"name":"bar","age":42}}` | `child=(name=foo age=10) parent=(name=bar age=42)` |
| `{"greet":"Hello World"}` | `greet="Hello World"` |
| `{"greet":"Hello\nWorld"}` | `greet="Hello`<br>`World"` |
| `[{"foo":1,"bar":[1,1]},{"foo":2,"bar":[2,2]}]` | `foo=1 bar=1,1`<br>`foo=2 bar=2,2` |
