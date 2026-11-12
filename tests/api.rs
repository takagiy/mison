use std::io::{self, Read};

#[test]
fn to_string_writes_at_top_level() {
    // Given: an array of two numbers / When: written with to_string / Then: they are on separate lines
    assert_eq!(mison::to_string(&vec![1, 2]), "1\n2");
}

#[test]
fn to_writer_writes_same_text() {
    // Given: an array of two numbers / When: written with to_writer / Then: the bytes are to_string's text
    let mut out = Vec::new();
    mison::to_writer(&mut out, &vec![1, 2]).unwrap();
    assert_eq!(out, b"1\n2");
}

#[test]
fn from_str_reads_whole_text() {
    // Given: two lines, and a line with text after its value / When: read as an array of numbers
    // Then: the first reads, and the second is an error
    assert_eq!(mison::from_str::<Vec<i32>>("1\n2"), Ok(vec![1, 2]));
    assert!(mison::from_str::<Vec<i32>>("1 2").is_err());
}

#[test]
fn from_slice_reads_utf8() {
    // Given: UTF-8 bytes and bytes that are not UTF-8 / When: read with from_slice / Then: only UTF-8 reads
    assert_eq!(mison::from_slice::<String>("é".as_bytes()), Ok("é".into()));
    assert!(mison::from_slice::<String>(&[0xff]).is_err());
}

/// A reader that always fails.
struct Broken;

impl Read for Broken {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("broken"))
    }
}

#[test]
fn from_reader_reads_everything() {
    // Given: a reader of two lines, and a reader that fails / When: read with from_reader
    // Then: the first reads, and the failure is an error
    assert_eq!(mison::from_reader::<_, Vec<i32>>(&b"1\n2"[..]), Ok(vec![1, 2]));
    assert!(mison::from_reader::<_, Vec<i32>>(Broken).is_err());
}
