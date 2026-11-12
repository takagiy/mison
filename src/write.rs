//! Writing MISON text, in MISON's own terms: strings, arrays, records and variants.

use std::mem::replace;

/// A value that MISON writes, with its type's height as the book defines it.
pub trait Encode {
    /// The height of the type: how deeply its arrays and tuples nest, which decides their separators.
    const HEIGHT: usize;

    /// Writes the value. A nil value writes nothing.
    fn write(&self, w: &mut Writer);
}

/// Where a value is written, which decides what goes around it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    /// The top level.
    Top,
    /// Inside the text, with nothing around it.
    Inner,
    /// A field value, where a record or a matrix goes in parentheses.
    Field,
    /// The contents of a case, inside the case's parentheses.
    Contents,
    /// An element of an array separated by `,` or `;`, where a record goes in parentheses.
    Paren,
    /// An element of a bracket literal, where a record goes in parentheses.
    Brk,
}

/// Builds MISON text from strings, arrays, records and variants.
pub struct Writer {
    out: String,
    place: Place,
    /// Inside a bracket literal, where `[` and `]` are special characters.
    brk: bool,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Writer { out: String::new(), place: Place::Top, brk: false }
    }

    pub(crate) fn finish(self) -> String {
        self.out
    }

    /// Writes a literal as is, such as a number or `true`.
    pub fn literal(&mut self, text: &str) {
        self.out.push_str(text);
    }

    /// Writes a string, in quotes when it needs them.
    pub fn string(&mut self, s: &str) {
        let special = |c: char| c.is_whitespace() || ",;()=\"".contains(c) || self.brk && "[]".contains(c);
        if !s.is_empty() && s != "----" && !s.chars().any(special) {
            return self.out.push_str(s);
        }
        self.out.push('"');
        for c in s.chars() {
            if c == '"' || c == '\\' {
                self.out.push('\\');
            }
            self.out.push(c);
        }
        self.out.push('"');
    }

    /// Writes a range from the texts of its ends.
    pub fn range(&mut self, start: &str, end: &str) {
        self.bound(start);
        self.out.push('-');
        self.bound(end);
    }

    fn bound(&mut self, n: &str) {
        if n.starts_with('-') {
            self.out.push('(');
            self.out.push_str(n);
            self.out.push(')');
        } else {
            self.out.push_str(n);
        }
    }

    /// Writes an array or tuple of `height` with `len` elements, which `f` writes in order.
    pub fn seq(&mut self, height: usize, len: usize, f: impl FnOnce(&mut SeqWriter)) {
        if len == 0 {
            return;
        }
        let (open, sep, close, place) = if self.place == Place::Brk || height >= 5 {
            ("[", ",", "]", Place::Brk)
        } else if len == 1 {
            ("", "", "", if self.place == Place::Top { Place::Inner } else { self.place })
        } else {
            match (height, self.place) {
                (1 | 2, Place::Top) => ("", "\n", "", Place::Inner),
                (1, _) => ("", ",", "", Place::Paren),
                (2, _) => ("", ";", "", Place::Paren),
                (h, place) => {
                    let sep = if h == 3 { "\n" } else { "\n----\n" };
                    match place {
                        Place::Field => ("(\n", sep, "\n)", Place::Inner),
                        Place::Contents => ("\n", sep, "\n", Place::Inner),
                        _ => ("", sep, "", Place::Inner),
                    }
                }
            }
        };
        let start = self.out.len();
        let brk = self.brk;
        self.brk |= place == Place::Brk;
        self.out.push_str(open);
        let mut seq = SeqWriter { w: self, sep, place, n: 0 };
        f(&mut seq);
        debug_assert_eq!(seq.n, len, "elements written, against the length given");
        self.out.push_str(close);
        self.brk = brk;
        // A bracket literal whose one element is nil is nil.
        if len == 1 && self.out.len() == start + open.len() + close.len() {
            self.out.truncate(start);
        }
    }

    /// Writes a record, whose fields `f` writes in order. A record without fields is nil.
    pub fn record(&mut self, f: impl FnOnce(&mut RecordWriter)) {
        let paren = matches!(self.place, Place::Field | Place::Paren | Place::Brk);
        let mut record = RecordWriter { w: self, paren, n: 0 };
        f(&mut record);
        if paren && record.n > 0 {
            self.out.push(')');
        }
    }

    /// Writes a case without contents: its label.
    pub fn unit_variant(&mut self, label: &str) {
        self.string(label);
    }

    /// Writes a case with contents: its label, and the contents, which `f` writes, in parentheses.
    pub fn variant(&mut self, label: &str, f: impl FnOnce(&mut Writer)) {
        self.string(label);
        self.out.push('(');
        self.within(Place::Contents, f);
        self.out.push(')');
    }

    fn within(&mut self, place: Place, f: impl FnOnce(&mut Self)) {
        let outer = replace(&mut self.place, place);
        f(self);
        self.place = outer;
    }
}

/// The elements of an array or tuple being written.
pub struct SeqWriter<'w> {
    w: &'w mut Writer,
    sep: &'static str,
    place: Place,
    n: usize,
}

impl SeqWriter<'_> {
    /// Writes the next element.
    pub fn element<T: Encode + ?Sized>(&mut self, x: &T) {
        if self.n > 0 {
            self.w.out.push_str(self.sep);
        }
        self.n += 1;
        self.w.within(self.place, |w| x.write(w));
    }
}

/// The fields of a record being written.
pub struct RecordWriter<'w> {
    w: &'w mut Writer,
    /// The record goes in parentheses.
    paren: bool,
    n: usize,
}

impl RecordWriter<'_> {
    /// Writes the next field.
    pub fn field<T: Encode + ?Sized>(&mut self, key: &str, value: &T) {
        self.w.out.push_str(if self.n > 0 {
            " "
        } else if self.paren {
            "("
        } else {
            ""
        });
        self.n += 1;
        self.w.string(key);
        self.w.out.push('=');
        self.w.within(Place::Field, |w| value.write(w));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use V::*;

    /// A value of any shape, whose arrays are given their heights.
    enum V {
        S(&'static str),
        Nil,
        Arr(usize, Vec<V>),
        Rec(Vec<(&'static str, V)>),
        Case(&'static str, Option<Box<V>>),
    }

    impl Encode for V {
        const HEIGHT: usize = 0;

        fn write(&self, w: &mut Writer) {
            match self {
                S(s) => w.string(s),
                Nil => {}
                Arr(height, xs) => w.seq(*height, xs.len(), |s| xs.iter().for_each(|x| s.element(x))),
                Rec(fields) => w.record(|r| fields.iter().for_each(|(k, v)| r.field(k, v))),
                Case(label, None) => w.unit_variant(label),
                Case(label, Some(contents)) => w.variant(label, |w| contents.write(w)),
            }
        }
    }

    fn written(f: impl FnOnce(&mut Writer)) -> String {
        let mut w = Writer::new();
        f(&mut w);
        w.finish()
    }

    /// The text of `v` at the top level.
    fn top(v: V) -> String {
        written(|w| v.write(w))
    }

    /// The text of `v` inside the text, not at the top level.
    fn inner(v: V) -> String {
        written(|w| w.within(Place::Inner, |w| v.write(w)))
    }

    /// The array of a and b, of `height`.
    fn ab(height: usize) -> V {
        Arr(height, vec![S("a"), S("b")])
    }

    fn case(label: &'static str, contents: V) -> V {
        Case(label, Some(Box::new(contents)))
    }

    // Arrays and tuples

    #[test]
    fn arrays_of_height_1_and_2_are_separated_by_comma_and_semicolon() {
        // Given: two elements / When: written as arrays of height 1 and 2 / Then: they are separated by , and ;
        assert_eq!(inner(ab(1)), "a,b");
        assert_eq!(inner(ab(2)), "a;b");
    }

    #[test]
    fn arrays_of_height_3_and_4_are_separated_by_lines() {
        // Given: two elements / When: written as arrays of height 3 and 4
        // Then: they are separated by a newline and by a ---- line
        assert_eq!(inner(ab(3)), "a\nb");
        assert_eq!(inner(ab(4)), "a\n----\nb");
    }

    #[test]
    fn top_level_arrays_of_height_1_and_2_are_separated_by_newlines() {
        // Given: two elements / When: written at the top level as arrays of height 1 and 2
        // Then: they are separated by newlines
        assert_eq!(top(ab(1)), "a\nb");
        assert_eq!(top(ab(2)), "a\nb");
    }

    #[test]
    fn one_element_array_is_written_as_its_element() {
        // Given: an array of height 2 whose one element is an array of height 1 / When: written at the top level
        // Then: it is the element, separated as an inner array
        assert_eq!(top(Arr(2, vec![ab(1)])), "a,b");
    }

    #[test]
    fn empty_array_and_array_of_one_nil_are_nothing() {
        // Given: an array with no elements, and one whose only element is nil / When: written / Then: both are empty
        assert_eq!(inner(Arr(1, vec![])), "");
        assert_eq!(inner(Arr(1, vec![Nil])), "");
    }

    #[test]
    fn nil_element_is_nothing() {
        // Given: an array with a nil element between two others / When: written / Then: the nil is empty
        assert_eq!(inner(Arr(1, vec![S("a"), Nil, S("b")])), "a,,b");
    }

    #[test]
    fn record_element_of_comma_array_is_in_parentheses() {
        // Given: an array of two records / When: written separated by , / Then: each record is in parentheses
        let r = || Rec(vec![("x", S("a"))]);
        assert_eq!(inner(Arr(1, vec![r(), r()])), "(x=a),(x=a)");
    }

    #[test]
    fn record_element_of_line_array_has_no_parentheses() {
        // Given: an array of two records / When: written at the top level, separated by newlines
        // Then: the records have no parentheses
        let r = || Rec(vec![("x", S("a"))]);
        assert_eq!(top(Arr(1, vec![r(), r()])), "x=a\nx=a");
    }

    #[test]
    fn array_of_height_5_is_bracket_literal() {
        // Given: an array of height 5 / When: written at the top level and as a field value
        // Then: it is a bracket literal, with its elements separated by ,
        assert_eq!(top(ab(5)), "[a,b]");
        assert_eq!(top(Rec(vec![("m", ab(5))])), "m=[a,b]");
    }

    #[test]
    fn arrays_inside_bracket_literal_are_bracket_literals() {
        // Given: an array of height 5 holding a one-element array and a two-element array / When: written
        // Then: both inner arrays are bracket literals
        assert_eq!(top(Arr(5, vec![Arr(1, vec![S("a")]), ab(1)])), "[[a],[a,b]]");
    }

    #[test]
    fn one_element_array_of_nil_inside_bracket_literal_is_nothing() {
        // Given: bracket literals holding an array whose only element is nil / When: written
        // Then: that array is empty, and a bracket literal of only it is empty too
        assert_eq!(top(Arr(5, vec![S("a"), Arr(1, vec![Nil])])), "[a,]");
        assert_eq!(top(Arr(5, vec![Arr(1, vec![Nil])])), "");
    }

    #[test]
    fn record_inside_bracket_literal_is_in_parentheses() {
        // Given: an array of height 5 whose one element is a record with an array field / When: written
        // Then: the record is in parentheses, and its field is written as usual
        assert_eq!(top(Arr(5, vec![Rec(vec![("x", ab(1))])])), "[(x=a,b)]");
    }

    // Records

    #[test]
    fn record_is_fields_separated_by_space() {
        // Given: a record of two fields / When: written / Then: it is key=value pairs separated by a space
        assert_eq!(top(Rec(vec![("x", S("a")), ("y", S("b"))])), "x=a y=b");
    }

    #[test]
    fn key_needing_quotes_is_quoted() {
        // Given: a key holding a space / When: the record is written / Then: the key is in quotes
        assert_eq!(top(Rec(vec![("a b", S("a"))])), r#""a b"=a"#);
    }

    #[test]
    fn nil_field_value_is_nothing() {
        // Given: a field whose value is nil / When: written / Then: nothing follows =
        assert_eq!(top(Rec(vec![("x", Nil), ("y", S("b"))])), "x= y=b");
    }

    #[test]
    fn record_without_fields_is_nothing() {
        // Given: a record without fields, at the top level and as a field value / When: written
        // Then: it is empty, without parentheses
        assert_eq!(top(Rec(vec![])), "");
        assert_eq!(top(Rec(vec![("x", Rec(vec![]))])), "x=");
    }

    #[test]
    fn record_field_value_is_in_parentheses() {
        // Given: a field whose value is a record / When: written / Then: the value is in parentheses
        assert_eq!(top(Rec(vec![("x", Rec(vec![("y", S("a"))]))])), "x=(y=a)");
    }

    #[test]
    fn matrix_field_value_is_in_parentheses_on_its_own_lines() {
        // Given: a field whose value is an array of height 3 / When: written
        // Then: the value is in parentheses, with a newline after ( and before )
        assert_eq!(top(Rec(vec![("m", ab(3))])), "m=(\na\nb\n)");
    }

    #[test]
    fn low_array_field_value_has_no_parentheses() {
        // Given: fields whose values are arrays of height 1 and 2 / When: written / Then: no parentheses
        assert_eq!(top(Rec(vec![("x", ab(1)), ("y", ab(2))])), "x=a,b y=a;b");
    }

    // Variants

    #[test]
    fn case_without_contents_is_its_label() {
        // Given: a case without contents / When: written / Then: it is its label
        assert_eq!(top(Case("ok", None)), "ok");
    }

    #[test]
    fn case_with_contents_is_label_and_contents_in_parentheses() {
        // Given: a case with a string as its contents / When: written / Then: the contents follow in parentheses
        assert_eq!(top(case("err", S("not found"))), r#"err("not found")"#);
    }

    #[test]
    fn record_contents_have_no_parentheses_of_their_own() {
        // Given: a case whose contents are a record / When: written / Then: only the case's parentheses remain
        let r = Rec(vec![("width", S("a")), ("height", S("b"))]);
        assert_eq!(top(case("rect", r)), "rect(width=a height=b)");
    }

    #[test]
    fn matrix_contents_are_on_their_own_lines() {
        // Given: a case whose contents are an array of height 3 / When: written
        // Then: a newline follows ( and precedes )
        assert_eq!(top(case("m", ab(3))), "m(\na\nb\n)");
    }

    #[test]
    fn nil_contents_are_empty_parentheses() {
        // Given: a case whose contents are nil / When: written / Then: it is the label and ()
        assert_eq!(top(case("v", Nil)), "v()");
    }

    // Strings

    #[test]
    fn plain_string_is_written_without_quotes() {
        // Given: a string of characters that are not special / When: it is written / Then: it has no quotes
        assert_eq!(written(|w| w.string("john@example.com")), "john@example.com");
    }

    #[test]
    fn empty_string_is_written_as_quotes() {
        // Given: an empty string / When: it is written / Then: it is ""
        assert_eq!(written(|w| w.string("")), r#""""#);
    }

    #[test]
    fn string_with_special_character_is_written_in_quotes() {
        // Given: strings holding whitespace, ',', ';', '(', ')' or '=' / When: written / Then: each is quoted
        for s in ["a b", "a\tb", "a,b", "a;b", "a(b", "a)b", "a=b"] {
            assert_eq!(written(|w| w.string(s)), format!(r#""{s}""#));
        }
    }

    #[test]
    fn quote_and_backslash_are_escaped_and_newline_is_kept() {
        // Given: a string holding ", \ and a newline / When: it is written / Then: " and \ get a \ before them
        assert_eq!(written(|w| w.string("say \"\\\"\nok")), "\"say \\\"\\\\\\\"\nok\"");
    }

    #[test]
    fn dashes_are_written_in_quotes() {
        // Given: the string ---- / When: it is written / Then: it is quoted, so it is not a separator
        assert_eq!(written(|w| w.string("----")), r#""----""#);
    }

    #[test]
    fn brackets_are_quoted_only_inside_bracket_literal() {
        // Given: a string holding [ and ] / When: written outside and inside a bracket literal
        // Then: it is quoted only inside
        assert_eq!(top(S("[INFO]")), "[INFO]");
        assert_eq!(top(Arr(5, vec![S("[INFO]")])), r#"["[INFO]"]"#);
    }
}
