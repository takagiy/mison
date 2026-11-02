# Read MISON back only through the type it was written from

## Context

MISON exists to be shorter than JSON, and most of what it saves is type information: the quotes that tell a string from a number, the braces that tell an object from its fields, the brackets around a single-element array. Without them, a text such as `age=20` or `array=1` does not say what it holds.

## Decision

The type a program reads into is the schema. `mison::from_str::<T>` lets serde ask for each value as the type `T` declares it, and parses the text as that kind of value. MISON is a format that does not describe itself, like bincode.

Reading into a type that asks the text what it holds, through serde's `deserialize_any` (`serde_json::Value`, `#[serde(untagged)]` enums), is an error. Writing works for any value serde can serialize.

## Consequences

- Gained: every rule can drop what the type already says. Plain strings need no quotes, a float equal to an integer needs no fraction, a single-element array needs no brackets, and `-` in a file name needs no quotes even though `-` also writes a range.
- Lost: a MISON text cannot be read without the type it came from, so it cannot be explored like JSON or read by a program written in another language unless that program declares the same shape. Untagged enums can be written but not read.

## Alternatives not taken

- A separate schema language, like JSON Schema, would let programs without serde read MISON, but is a second thing to design, write and keep in step with the types.
- Writing only, with no reader, would be enough for language models reading a tool's output, but would leave the claim that MISON reads back unchecked; the reader is what keeps the rules unambiguous.
