//! Reading MISON text, in MISON's own terms: strings, arrays, records and variants.

use crate::{Encode, Error};
use std::borrow::Cow;
use std::mem::replace;
use std::str::FromStr;

/// A type that MISON reads back, by its height and the text.
pub trait Decode: Encode + Sized {
    fn read(r: &mut Reader) -> Result<Self, Error>;
}

/// Where an expression is, which decides what ends it and whether a record there is in parentheses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// The whole text.
    Top,
    /// A line: an element of an array separated by newlines.
    Row,
    /// A field value.
    Field,
    /// The contents of a variant.
    Contents,
    /// An element of an array separated by `,` or `;`. `paren` if a record here must be in parentheses.
    Item { paren: bool },
    /// An element of a bracket literal.
    Brk,
}

/// The separator between the elements of an array.
#[derive(Clone, Copy)]
enum Sep {
    Char(char),
    Newline,
    Dashes,
}

/// Reads values out of MISON text: strings, arrays, records and variants.
pub struct Reader<'a> {
    text: &'a str,
    i: usize,
    place: Place,
    /// Inside a bracket literal, where `[` and `]` are special characters.
    brk: bool,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        Reader { text, i: 0, place: Place::Top, brk: false }
    }

    pub(crate) fn finish(&self) -> Result<(), Error> {
        if self.i < self.text.len() { Err(self.error("unexpected text after the value")) } else { Ok(()) }
    }

    /// An error at the byte being read.
    pub fn error(&self, message: impl std::fmt::Display) -> Error {
        Error(format!("{message} at byte {}", self.i))
    }

    fn rest(&self) -> &'a str {
        &self.text[self.i..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn eat(&mut self, c: char) -> bool {
        let ate = self.peek() == Some(c);
        if ate {
            self.i += c.len_utf8();
        }
        ate
    }

    fn expect(&mut self, c: char) -> Result<(), Error> {
        if self.eat(c) { Ok(()) } else { Err(self.error(format_args!("expected `{c}`"))) }
    }

    fn special(&self, c: char) -> bool {
        c.is_whitespace() || ",;()=\"".contains(c) || self.brk && "[]".contains(c)
    }

    /// Whether the value here is nil: nothing is written before what follows it.
    pub fn is_nil(&self) -> bool {
        match self.peek() {
            None => true,
            // contents that start with a newline are a matrix
            Some('\n' | '\r') if self.place == Place::Contents => false,
            Some(c) => c.is_whitespace() || ",;)".contains(c) || self.brk && c == ']',
        }
    }

    /// Reads the characters up to the next special one: the text of a literal.
    pub fn token(&mut self) -> Result<&'a str, Error> {
        let rest = self.rest();
        let end = rest.find(|c| self.special(c)).unwrap_or(rest.len());
        if end == 0 {
            return Err(self.error("expected a value"));
        }
        self.i += end;
        Ok(&rest[..end])
    }

    /// Reads a token that `valid` accepts, as a `T`.
    fn literal<T: FromStr>(&mut self, what: &str, valid: impl Fn(&str) -> bool) -> Result<T, Error> {
        let start = self.i;
        let parsed = self.token().ok().filter(|t| valid(t)).and_then(|t| t.parse().ok());
        parsed.ok_or_else(|| {
            self.i = start;
            self.error(format_args!("expected {what}"))
        })
    }

    pub fn int<T: FromStr>(&mut self) -> Result<T, Error> {
        self.literal("an integer in range", is_int)
    }

    pub fn float<T: FromStr>(&mut self) -> Result<T, Error> {
        self.literal("a float", is_float)
    }

    pub fn boolean(&mut self) -> Result<bool, Error> {
        self.literal("`true` or `false`", |t| t == "true" || t == "false")
    }

    /// Reads a string, quoted or not.
    pub fn string(&mut self) -> Result<Cow<'a, str>, Error> {
        if !self.eat('"') {
            return self.token().map(Cow::Borrowed);
        }
        let mut s = String::new();
        let mut chars = self.rest().char_indices();
        loop {
            match chars.next() {
                None => {
                    self.i = self.text.len();
                    return Err(self.error("expected `\"`"));
                }
                Some((j, '"')) => {
                    self.i += j + 1;
                    return Ok(Cow::Owned(s));
                }
                Some((j, '\\')) => match chars.next() {
                    Some((_, c @ ('"' | '\\'))) => s.push(c),
                    _ => {
                        self.i += j;
                        return Err(self.error("expected `\"` or `\\` after `\\`"));
                    }
                },
                Some((_, c)) => s.push(c),
            }
        }
    }

    /// Reads a range, as the texts of its ends without parentheses.
    pub fn range(&mut self) -> Result<(&'a str, &'a str), Error> {
        let start = self.bound()?;
        self.expect('-')?;
        Ok((start, self.bound()?))
    }

    fn bound(&mut self) -> Result<&'a str, Error> {
        let paren = self.eat('(');
        let rest = self.rest();
        let end = if paren { rest.find(')') } else { rest.find(|c: char| !c.is_ascii_digit()) };
        let n = &rest[..end.unwrap_or(rest.len())];
        if !is_int(n) || n.starts_with('-') != paren {
            return Err(self.error(if paren { "expected a negative integer" } else { "expected a range end" }));
        }
        self.i += n.len() + paren as usize;
        Ok(n)
    }

    /// Reads an array or tuple of `height`, calling `item` for each element; returns how many there were.
    pub fn seq(&mut self, height: usize, mut item: impl FnMut(&mut Self) -> Result<(), Error>) -> Result<usize, Error> {
        if self.place == Place::Brk || height >= 5 {
            if self.is_nil() {
                return Ok(0);
            }
            self.expect('[')?;
            let brk = replace(&mut self.brk, true);
            let mut n = 0;
            loop {
                self.within(Place::Brk, &mut item)?;
                n += 1;
                if !self.eat(',') {
                    break;
                }
            }
            self.brk = brk;
            self.expect(']')?;
            return Ok(n);
        }
        if height >= 3 && !matches!(self.place, Place::Top | Place::Row) {
            // a matrix is on lines of its own: in parentheses as a field value, directly as contents
            let paren = self.place == Place::Field;
            let rest = self.rest();
            let open = match self.place {
                Place::Field => rest.starts_with('(') && newline_len(&rest[1..]) > 0,
                Place::Contents => newline_len(rest) > 0,
                _ => false,
            };
            if !open {
                return if self.is_empty(height) { Ok(0) } else { item(self).map(|()| 1) };
            }
            self.i += paren as usize;
            self.i += newline_len(self.rest());
            let n = self.within(Place::Row, |r| r.elements(height, item))?;
            match newline_len(self.rest()) {
                0 => return Err(self.error("expected a newline")),
                len => self.i += len,
            }
            if paren {
                self.expect(')')?;
            }
            return Ok(n);
        }
        self.elements(height, item)
    }

    /// The separator between the elements of an array of `height` here.
    fn sep(&self, height: usize) -> Sep {
        match height {
            1 | 2 if self.place == Place::Top => Sep::Newline,
            1 => Sep::Char(','),
            2 => Sep::Char(';'),
            3 => Sep::Newline,
            _ => Sep::Dashes,
        }
    }

    /// Whether an array of `height` here is nil: nothing before what follows it, not even a separator of its own
    /// or of a lower array, which would start its first element.
    fn is_empty(&self, height: usize) -> bool {
        let rest = self.rest();
        self.is_nil()
            && self.sep_len(self.sep(height)) == 0
            && !(height >= 2 && rest.starts_with(',')
                || height >= 3 && rest.starts_with(';')
                || height >= 4 && self.sep_len(Sep::Newline) > 0)
    }

    fn elements(
        &mut self,
        height: usize,
        mut item: impl FnMut(&mut Self) -> Result<(), Error>,
    ) -> Result<usize, Error> {
        let sep = self.sep(height);
        if self.is_empty(height) {
            return Ok(0);
        }
        let place = match sep {
            Sep::Char(_) => Place::Item { paren: self.paren() },
            _ => Place::Row,
        };
        let mut n = 0;
        loop {
            self.within(place, &mut item)?;
            n += 1;
            match self.sep_len(sep) {
                0 => return Ok(n),
                len => self.i += len,
            }
        }
    }

    /// The length of the separator `sep` here, or 0 if there is none.
    fn sep_len(&self, sep: Sep) -> usize {
        let rest = self.rest();
        let nl = newline_len(rest);
        match sep {
            Sep::Char(c) => rest.starts_with(c) as usize,
            // a newline before ) closes a matrix, and one before a ---- line belongs to that line
            Sep::Newline if nl > 0 && !rest[nl..].starts_with(')') && dashes_len(&rest[nl..]) == 0 => nl,
            Sep::Dashes if nl > 0 && dashes_len(&rest[nl..]) > 0 => nl + dashes_len(&rest[nl..]),
            _ => 0,
        }
    }

    /// Whether a record here must be in parentheses.
    fn paren(&self) -> bool {
        match self.place {
            Place::Field | Place::Brk => true,
            Place::Item { paren } => paren,
            _ => false,
        }
    }

    /// Reads a record, calling `field` with each key, to read its value.
    pub fn record(&mut self, mut field: impl FnMut(Cow<'a, str>, &mut Self) -> Result<(), Error>) -> Result<(), Error> {
        let paren = self.paren() || matches!(self.place, Place::Item { .. }) && self.peek() == Some('(');
        if paren {
            self.expect('(')?;
        }
        loop {
            let key = self.string()?;
            self.expect('=')?;
            self.within(Place::Field, |r| field(key, r))?;
            if !self.spaces() {
                break;
            }
        }
        if paren {
            self.expect(')')?;
        }
        Ok(())
    }

    /// Skips the spaces between fields; whether there were any.
    fn spaces(&mut self) -> bool {
        let start = self.i;
        while let Some(c) = self.peek() {
            if !c.is_whitespace() || newline_len(self.rest()) > 0 {
                break;
            }
            self.i += c.len_utf8();
        }
        self.i > start
    }

    /// Reads the label of a variant.
    pub fn label(&mut self) -> Result<Cow<'a, str>, Error> {
        self.string()
    }

    /// Whether contents follow the label just read.
    pub fn has_contents(&self) -> bool {
        self.peek() == Some('(')
    }

    /// Reads the contents of a variant, in parentheses after its label.
    pub fn contents<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, Error>) -> Result<T, Error> {
        self.expect('(')?;
        let value = self.within(Place::Contents, f)?;
        self.expect(')')?;
        Ok(value)
    }

    fn within<T>(&mut self, place: Place, f: impl FnOnce(&mut Self) -> T) -> T {
        let outer = replace(&mut self.place, place);
        let value = f(self);
        self.place = outer;
        value
    }
}

fn newline_len(s: &str) -> usize {
    if s.starts_with('\n') {
        1
    } else if s.starts_with("\r\n") {
        2
    } else {
        0
    }
}

/// The length of a `----` line's dashes and the newline after them, or 0.
fn dashes_len(s: &str) -> usize {
    match s.strip_prefix("----").map(newline_len) {
        Some(nl) if nl > 0 => 4 + nl,
        _ => 0,
    }
}

/// `0`, or digits not starting with 0 after an optional `-`.
fn is_int(t: &str) -> bool {
    let digits = t.strip_prefix('-').unwrap_or(t);
    t == "0" || digits.starts_with(|c: char| ('1'..='9').contains(&c)) && digits.bytes().all(|b| b.is_ascii_digit())
}

/// An integer part, then an optional fraction and exponent; or `NaN`, `inf` or `-inf`.
fn is_float(t: &str) -> bool {
    if matches!(t, "NaN" | "inf" | "-inf") {
        return true;
    }
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let (mantissa, exponent) = match t.find(['e', 'E']) {
        Some(e) => (&t[..e], Some(&t[e + 1..])),
        None => (t, None),
    };
    let (int, fraction) = match mantissa.split_once('.') {
        Some((int, fraction)) => (int, Some(fraction)),
        None => (mantissa, None),
    };
    let int = int.strip_prefix('-').unwrap_or(int);
    (int == "0" || is_int(int))
        && fraction.is_none_or(digits)
        && exponent.is_none_or(|e| digits(e.strip_prefix(['+', '-']).unwrap_or(e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read<T>(text: &str, f: impl FnOnce(&mut Reader) -> Result<T, Error>) -> Result<T, Error> {
        let mut r = Reader::new(text);
        let value = f(&mut r)?;
        r.finish()?;
        Ok(value)
    }

    /// A string, or nil.
    fn atom(r: &mut Reader) -> Result<Option<String>, Error> {
        if r.is_nil() { Ok(None) } else { Ok(Some(r.string()?.into_owned())) }
    }

    /// An array of `height` whose elements are read by `item`.
    fn array<T>(
        height: usize,
        mut item: impl FnMut(&mut Reader) -> Result<T, Error>,
    ) -> impl FnMut(&mut Reader) -> Result<Vec<T>, Error> {
        move |r| {
            let mut v = vec![];
            r.seq(height, |r| {
                v.push(item(r)?);
                Ok(())
            })?;
            Ok(v)
        }
    }

    fn s(x: &str) -> Option<String> {
        Some(x.to_string())
    }

    // Literals

    #[test]
    fn unquoted_string_reads_up_to_special_character() {
        // Given: an unquoted string followed by , / When: a string is read / Then: it stops before ,
        assert_eq!(
            read("a-b.rs,", |r| {
                let x = r.string()?;
                r.expect(',')?;
                Ok(x.into_owned())
            }),
            Ok("a-b.rs".into())
        );
    }

    #[test]
    fn quoted_string_is_unescaped() {
        // Given: a quoted string holding \", \\ and a newline / When: read / Then: the escapes are undone
        assert_eq!(read("\"say \\\"\\\\\\\"\nok\"", |r| Ok(r.string()?.into_owned())), Ok("say \"\\\"\nok".into()));
    }

    #[test]
    fn other_escape_is_error() {
        // Given: \n written as two characters inside quotes / When: read / Then: it is an error
        assert!(read(r#""a\nb""#, |r| Ok(r.string()?.into_owned())).is_err());
    }

    #[test]
    fn unquoted_string_ends_at_bracket_inside_bracket_literal() {
        // Given: [a] as a bracket literal and as a plain string / When: read
        // Then: inside the literal the string ends at ], outside it does not
        assert_eq!(read("[a]", array(5, atom)), Ok(vec![s("a")]));
        assert_eq!(read("[a]", |r| Ok(r.string()?.into_owned())), Ok("[a]".into()));
    }

    #[test]
    fn integer_literal() {
        // Given: integers written in and out of the grammar / When: read / Then: only 0 and -?[1-9][0-9]* read
        for (text, n) in [("0", 0), ("20", 20), ("-3", -3)] {
            assert_eq!(read(text, |r| r.int::<i32>()), Ok(n));
        }
        for text in ["01", "+1", "-0", "1.0", ""] {
            assert!(read(text, |r| r.int::<i32>()).is_err(), "{text}");
        }
    }

    #[test]
    fn float_literal() {
        // Given: floats written in and out of the grammar / When: read / Then: only the grammar's forms read
        for (text, x) in [("1", 1.0), ("-1.5", -1.5), ("2e3", 2e3), ("1.5E-2", 1.5e-2), ("inf", f64::INFINITY)] {
            assert_eq!(read(text, |r| r.float::<f64>()), Ok(x));
        }
        assert!(read("NaN", |r| r.float::<f64>()).unwrap().is_nan());
        assert_eq!(read("-inf", |r| r.float::<f64>()), Ok(f64::NEG_INFINITY));
        for text in [".5", "1.", "infinity", "nan", "+1", "01"] {
            assert!(read(text, |r| r.float::<f64>()).is_err(), "{text}");
        }
    }

    #[test]
    fn boolean_literal() {
        // Given: true, false and True / When: read as booleans / Then: only true and false read
        assert_eq!(read("true", |r| r.boolean()), Ok(true));
        assert_eq!(read("false", |r| r.boolean()), Ok(false));
        assert!(read("True", |r| r.boolean()).is_err());
    }

    #[test]
    fn range_literal() {
        // Given: ranges with non-negative and parenthesized negative ends / When: read
        // Then: their ends read, and a negative end without parentheses is an error
        let range = |text: &str| read(text, |r| r.range().map(|(a, b)| format!("{a} {b}")));
        assert_eq!(range("2-10"), Ok("2 10".into()));
        assert_eq!(range("(-3)-(-1)"), Ok("-3 -1".into()));
        assert!(range("-3--1").is_err());
        assert!(range("01-2").is_err());
    }

    // Nil

    #[test]
    fn nothing_before_terminator_is_nil() {
        // Given: nothing before ,, a space, a newline, ) and the end / When: checked for nil / Then: each is nil
        for text in [",", " ", "\n", ")", ""] {
            assert!(Reader::new(text).is_nil(), "{text:?}");
        }
        assert!(!Reader::new("a").is_nil());
    }

    // Arrays and tuples

    #[test]
    fn elements_are_split_by_height_separator() {
        // Given: arrays of height 1 to 4 written in a field / When: read
        // Then: elements split at , ; a newline and a ---- line
        let field = |height: usize, text: &str| {
            read(&format!("x={text}"), |r| {
                let mut v = vec![];
                r.record(|_, r| {
                    v = array(height, atom)(r)?;
                    Ok(())
                })?;
                Ok(v)
            })
        };
        let ab = Ok(vec![s("a"), s("b")]);
        assert_eq!(field(1, "a,b"), ab);
        assert_eq!(field(2, "a;b"), ab);
        assert_eq!(field(3, "(\na\nb\n)"), ab);
        assert_eq!(field(4, "(\na\n----\nb\n)"), ab);
    }

    #[test]
    fn top_level_low_arrays_split_by_newline() {
        // Given: a top-level text of two lines / When: read as arrays of height 1 and 2 / Then: each line is an element
        assert_eq!(read("a\nb", array(1, atom)), Ok(vec![s("a"), s("b")]));
        assert_eq!(read("a,b\nc", array(2, array(1, atom))), Ok(vec![vec![s("a"), s("b")], vec![s("c")]]));
    }

    #[test]
    fn text_without_separator_is_one_element() {
        // Given: a,b / When: read as an array of height 2 / Then: it is one element, an array of height 1
        assert_eq!(
            read("x=a,b", |r| {
                let mut v = vec![];
                r.record(|_, r| {
                    v = array(2, array(1, atom))(r)?;
                    Ok(())
                })?;
                Ok(v)
            }),
            Ok(vec![vec![s("a"), s("b")]])
        );
    }

    #[test]
    fn nothing_is_empty_array() {
        // Given: an empty text / When: read as an array / Then: it has no elements
        assert_eq!(read("", array(1, atom)), Ok(vec![]));
    }

    #[test]
    fn lower_separator_starts_first_element() {
        // Given: a top-level line that starts with , / When: read as an array of height 2
        // Then: it is not empty: its first element starts with nil
        assert_eq!(read(",a\nb", array(2, array(1, atom))), Ok(vec![vec![None, s("a")], vec![s("b")]]));
    }

    #[test]
    fn nothing_in_bracket_literal_is_nil() {
        // Given: a bracket literal whose second element is nothing / When: read / Then: that element is empty
        assert_eq!(read("[[a],]", array(5, array(1, atom))), Ok(vec![vec![s("a")], vec![]]));
    }

    #[test]
    fn newline_before_dashes_or_parenthesis_does_not_separate_height_3() {
        // Given: an array of height 4 in a field, whose elements have newlines / When: read
        // Then: the ---- line and the closing ) end the elements of height 3
        let v = read("x=(\na\nb\n----\nc\n)", |r| {
            let mut v = vec![];
            r.record(|_, r| {
                v = array(4, array(3, atom))(r)?;
                Ok(())
            })?;
            Ok(v)
        });
        assert_eq!(v, Ok(vec![vec![s("a"), s("b")], vec![s("c")]]));
    }

    #[test]
    fn crlf_is_newline() {
        // Given: two lines separated by \r\n / When: read at the top level / Then: they are two elements
        assert_eq!(read("a\r\nb", array(1, atom)), Ok(vec![s("a"), s("b")]));
    }

    #[test]
    fn matrix_contents_are_on_their_own_lines() {
        // Given: contents with a newline after ( and before ) / When: read as an array of height 3
        // Then: each line is an element
        let v = read("m(\na\nb\n)", |r| {
            r.label()?;
            r.contents(array(3, atom))
        });
        assert_eq!(v, Ok(vec![s("a"), s("b")]));
    }

    #[test]
    fn high_array_reads_only_from_bracket_literal() {
        // Given: an array of height 5 as a bracket literal and as plain elements / When: read
        // Then: only the bracket literal reads
        assert_eq!(read("[a,,b]", array(5, atom)), Ok(vec![s("a"), None, s("b")]));
        assert!(read("a,b", array(5, atom)).is_err());
    }

    // Records

    #[test]
    fn fields_read_in_written_order() {
        // Given: two fields separated by spaces / When: read / Then: each key and value reads in order
        let mut fields = vec![];
        read("b=1  a=2", |r| {
            r.record(|k, r| {
                fields.push((k.into_owned(), r.token()?.to_string()));
                Ok(())
            })
        })
        .unwrap();
        assert_eq!(fields, [("b".to_string(), "1".to_string()), ("a".into(), "2".into())]);
    }

    /// A record of string fields, or nil.
    fn rec(r: &mut Reader) -> Result<Vec<(String, Option<String>)>, Error> {
        let mut fields = vec![];
        r.record(|k, r| {
            fields.push((k.into_owned(), atom(r)?));
            Ok(())
        })?;
        Ok(fields)
    }

    #[test]
    fn record_field_value_reads_only_in_parentheses() {
        // Given: a record field value with and without parentheses / When: read / Then: only the first reads
        let field = |text: &str| {
            read(text, |r| {
                let mut v = vec![];
                r.record(|_, r| {
                    v = rec(r)?;
                    Ok(())
                })?;
                Ok(v)
            })
        };
        assert_eq!(field("x=(y=a)"), Ok(vec![("y".to_string(), s("a"))]));
        assert!(field("x=y=a").is_err());
    }

    #[test]
    fn record_element_of_comma_array_reads_in_parentheses() {
        // Given: two records in parentheses separated by , / When: read as an array of height 1 in a field
        // Then: each is a record
        let v = read("x=(y=a),(y=b)", |r| {
            let mut v = vec![];
            r.record(|_, r| {
                v = array(1, rec)(r)?;
                Ok(())
            })?;
            Ok(v)
        });
        assert_eq!(v, Ok(vec![vec![("y".to_string(), s("a"))], vec![("y".to_string(), s("b"))]]));
    }

    #[test]
    fn record_reads_without_parentheses_at_top_in_row_and_as_contents() {
        // Given: records at the top level, as rows and as contents / When: read / Then: none needs parentheses
        assert_eq!(read("y=a", rec), Ok(vec![("y".to_string(), s("a"))]));
        assert_eq!(read("y=a\ny=b", array(1, rec)).map(|v| v.len()), Ok(2));
        assert_eq!(
            read("v(y=a)", |r| {
                r.label()?;
                r.contents(rec)
            }),
            Ok(vec![("y".to_string(), s("a"))])
        );
    }

    #[test]
    fn carriage_return_before_line_feed_does_not_separate_fields() {
        // Given: two records on lines ended by \r\n / When: read / Then: the \r ends the line, not a field
        assert_eq!(read("y=a\r\ny=b", array(1, rec)).map(|v| v.len()), Ok(2));
    }

    // Variants

    #[test]
    fn label_with_parentheses_reads_contents() {
        // Given: a label followed by a string in parentheses / When: read / Then: the label and contents read
        assert_eq!(
            read(r#"err("not found")"#, |r| Ok((r.label()?.into_owned(), r.contents(atom)?))),
            Ok(("err".to_string(), s("not found")))
        );
    }

    #[test]
    fn empty_parentheses_are_nil_contents() {
        // Given: a label followed by () / When: read / Then: the contents are nil
        assert_eq!(
            read("v()", |r| {
                r.label()?;
                r.contents(atom)
            }),
            Ok(None)
        );
    }

    // Top level

    #[test]
    fn text_after_value_is_error() {
        // Given: a string followed by more text / When: read as one string / Then: it is an error
        assert!(read("a b", |r| Ok(r.string()?.into_owned())).is_err());
    }

    #[test]
    fn error_names_byte_offset() {
        // Given: a text with , after a field / When: read as a record / Then: the error names the byte of ,
        let e = read("x=a,b", |r| {
            r.record(|_, r| {
                r.string()?;
                Ok(())
            })
        })
        .unwrap_err();
        assert!(e.to_string().ends_with("at byte 3"), "{e}");
    }
}
