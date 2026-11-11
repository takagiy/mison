# Introduction

## Overview

MISON (minimal and simple object notation) is a JSON-like notation for structured data that is easy for people to read, and so for language models too.

```json
[{"ok":true,"file":"a.rs","range":[[2,2]]}]
```

```mison
ok=true file=a.rs range=2-2
```

## Use cases

Its main use is passing structured data to a language model in fewer tokens.

Exchanges between programs with no language model in between, such as RPC or API requests and responses, are not intended.

## Types and parsing

A program parsing MISON already knows the type of the data it expects. So MISON leaves out the quotes and brackets the type can fill back in, and writes the same data in fewer characters than JSON. For example, `age=20` reads as the number 20 if the type is a number, and as the string `"20"` if it is a string.
