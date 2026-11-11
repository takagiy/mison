# mison

MISON (minimal and simple object notation) is a simpler JSON-like notation for structured data. It is easy to read even for people seeing it for the first time, and so for language models too. This crate implements it for [serde](https://serde.rs).

```text
JSON:  [{"ok":true,"file":"a.rs","range":[[2,2]]}]
MISON: ok=true file=a.rs range=2-2
```

Its main use is passing structured data to a language model. Exchanges between programs with no language model in between, such as RPC or API requests and responses, are not intended.

A program parsing MISON uses the type of the data it expects as a schema. So MISON leaves out the quotes and brackets the type can fill back in, and writes the same data in fewer characters than JSON. The [book](https://github.com/takagiy/mison/tree/master/book) defines the notation.

## Usage

```rust
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Report {
    ok: bool,
    file: String,
    range: Vec<RangeInclusive<usize>>,
}

let report = Report { ok: true, file: "a.rs".into(), range: vec![2..=2] };
let text = mison::to_string(&report)?;
assert_eq!(text, "ok=true file=a.rs range=2-2");
assert_eq!(mison::from_str::<Report>(&text)?, report);
# Ok::<(), mison::Error>(())
```

## License

MIT
