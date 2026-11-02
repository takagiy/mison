# mison

MISON (minimal and simple object notation) is a JSON-like notation that is token-efficient yet human-friendly, with a [serde](https://serde.rs) implementation in this crate. It writes the data JSON writes with fewer characters, for readers that pay per character, such as language models reading a tool's output.

```text
JSON:  [{"ok":true,"file":"a.rs","replaced":[[2,2]]}]
MISON: ok=true file=a.rs replaced=2-2
```

MISON leaves out what a reader can work out, so a text does not say what type each value has. It reads back only through the type it was written from: the type is the schema. See [SPEC.md](SPEC.md) for the rules.

## Usage

```rust
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Report {
    ok: bool,
    file: String,
    replaced: Vec<RangeInclusive<usize>>,
}

let report = Report { ok: true, file: "a.rs".into(), replaced: vec![2..=2] };
let text = mison::to_string(&report)?;
assert_eq!(text, "ok=true file=a.rs replaced=2-2");
assert_eq!(mison::from_str::<Report>(&text)?, report);
# Ok::<(), mison::Error>(())
```

## License

MIT
