# Entry points

- [x] `to_string` writes a value at the top level
- [x] `to_writer` writes the same text to an `io::Write`
- [x] `from_str` reads a value from the whole text
- [x] `from_slice` reads from UTF-8 bytes; bytes that are not UTF-8 are an error
- [x] `from_reader` reads everything from an `io::Read`, and an I/O failure is an error
