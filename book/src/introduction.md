# Introduction

## Overview

MISON (minimal and simple object notation) is a JSON-like notation for structured data that is token-efficient yet human-friendly. It is easy to read even for people seeing it for the first time, and so for language models too.

```json
[{"ok":true,"file":"a.rs","ranges":[{"start":10,"end":15},{"start":20,"end":25}]}]
```

```mison
ok=true file=a.rs ranges=10-15,20-25
```

## Use cases

Its main use is passing structured data to a language model.

Exchanges between programs with no language model in between, such as RPC or API requests and responses, are not intended.

## Types and parsing

A program parsing MISON uses the type of the data it expects as a schema. So MISON leaves out the quotes and brackets the type can fill back in, and writes the same data in fewer characters than JSON. For example, `age=20` reads as the number 20 if the type is a number, and as the string `"20"` if it is a string.
