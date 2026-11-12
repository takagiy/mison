# Test lists

Each item is one behavior, checked by one test with Given/When/Then comments; `[x]` marks the ones that have it. The lists are split by layer, and each uses that layer's words.

| Layer | List | Words |
|---|---|---|
| Writing MISON text (`Writer`) | [writer.md](writer.md) | string, array, height, separator, element, record, field, key, variant, label, contents, nil, top level, bracket literal |
| Reading MISON text (`Reader`) | [reader.md](reader.md) | text, reads as, separator, element, field, label, contents, nil, error |
| Rust types (`Encode` and `Decode` implementations) | [types.md](types.md) | Rust type, value, height, written as, reads back |
| Derived types (`#[derive(Encode, Decode)]`) | [derive.md](derive.md) | struct, field, enum, variant, record, case |
| Entry points (`to_string`, `from_str`, …) | [api.md](api.md) | value, text, bytes, writer, reader, error |
