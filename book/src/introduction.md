# Introduction

MISON (minimal and simple object notation) is a JSON-like notation that is token-efficient yet human-friendly. It writes the data JSON writes with fewer characters, for readers that pay for every character they read, such as language models reading a tool's output.

```json
[{"ok":true,"file":"a.rs","replaced":[[2,2]]}]
```

```mison
ok=true file=a.rs replaced=2-2
```

MISON gets shorter by leaving out what the reader can work out: the quotes around plain strings, the braces around records, the colons after keys, and nesting that holds a single value.

The cost is that a MISON text does not say what type each value has. `age=20` may hold the number 20 or the string `"20"`. A program reading MISON already knows the shape of the data it expects, and that shape tells it how to read each value. [Reading MISON](reading.md) explains this after the values are introduced.

Each chapter on a value shows the same data in JSON and in MISON.
