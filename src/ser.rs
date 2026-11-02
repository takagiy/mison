//! Writing: serde turns a value into a [`Node`] tree, which is then laid out as text. How a value is laid
//! out depends on where it sits (the top level, a field, an element of a longer array), so the tree is built
//! first and written second.

use serde::ser::{self, Serialize};

use crate::Error;

/// Writes `value` as MISON.
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String, Error> {
    Ok(top(&value.serialize(Builder)?))
}

/// A value as serde described it, before it is laid out.
enum Node {
    /// `None`: a field holding it is left out, and it is nothing anywhere else.
    None,
    /// A scalar as written: a number, a boolean, a string (in quotes when it needs them), a variant's name.
    /// Unit is the empty atom.
    Atom(String),
    /// A struct serde names `Range` or `RangeInclusive`, by its ends as written.
    Range(String, String),
    Array(Vec<Node>),
    /// Fields, keys as written.
    Object(Vec<(String, Node)>),
}

// Layout. A value is laid out by where it sits: at the top level, as a field's value (or the only element
// of an array, which is written in its place), or as an element of an array with two or more elements.

fn top(node: &Node) -> String {
    match node {
        Node::Array(items) if items.len() > 1 => items.iter().map(line).collect::<Vec<_>>().join("\n"),
        _ => line(node),
    }
}

/// A value on a line of its own: an object needs no parentheses there.
fn line(node: &Node) -> String {
    match node {
        Node::Object(fields) => fields_text(fields),
        Node::Array(items) if items.len() == 1 => line(&items[0]),
        _ => value(node),
    }
}

fn value(node: &Node) -> String {
    match node {
        Node::None => String::new(),
        Node::Atom(text) => text.clone(),
        Node::Range(start, end) => format!("{start}-{end}"),
        Node::Object(fields) => {
            let text = fields_text(fields);
            if text.is_empty() { text } else { format!("({text})") }
        }
        Node::Array(items) => match items.as_slice() {
            [] => String::new(),
            [only] => value(only),
            _ => items.iter().map(element).collect::<Vec<_>>().join(","),
        },
    }
}

/// An element of an array with two or more elements: an array is grouped and not unwrapped.
fn element(node: &Node) -> String {
    match node {
        Node::Array(items) if !items.is_empty() => format!("({})", value(node)),
        _ => value(node),
    }
}

fn fields_text(fields: &[(String, Node)]) -> String {
    let present = fields.iter().filter(|(_, v)| !matches!(v, Node::None));
    present.map(|(key, v)| format!("{key}={}", value(v))).collect::<Vec<_>>().join(" ")
}

/// A string as written: in quotes when it is empty or holds a character MISON gives a meaning to.
fn string(s: &str) -> String {
    let plain = !s.is_empty() && !s.chars().any(|c| c.is_whitespace() || "=,()\"".contains(c));
    if plain {
        return s.into();
    }
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// A float in the shortest form that reads back as the same value, without a fraction when it is whole.
fn float(v: f64, debug: String) -> String {
    if v.is_nan() {
        return "NaN".into();
    }
    if v.is_infinite() {
        return if v > 0.0 { "inf".into() } else { "-inf".into() };
    }
    debug.strip_suffix(".0").map_or(debug.clone(), String::from)
}

// Building the tree.

struct Builder;

impl ser::Serializer for Builder {
    type Ok = Node;
    type Error = Error;
    type SerializeSeq = Seq;
    type SerializeTuple = Seq;
    type SerializeTupleStruct = Seq;
    type SerializeTupleVariant = Seq;
    type SerializeMap = Map;
    type SerializeStruct = Struct;
    type SerializeStructVariant = Struct;

    fn serialize_bool(self, v: bool) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_i8(self, v: i8) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_i16(self, v: i16) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_i32(self, v: i32) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_i64(self, v: i64) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_i128(self, v: i128) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_u8(self, v: u8) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_u16(self, v: u16) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_u32(self, v: u32) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_u64(self, v: u64) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_u128(self, v: u128) -> Result<Node, Error> {
        Ok(Node::Atom(v.to_string()))
    }
    fn serialize_f32(self, v: f32) -> Result<Node, Error> {
        Ok(Node::Atom(float(v.into(), format!("{v:?}"))))
    }
    fn serialize_f64(self, v: f64) -> Result<Node, Error> {
        Ok(Node::Atom(float(v, format!("{v:?}"))))
    }
    fn serialize_char(self, v: char) -> Result<Node, Error> {
        Ok(Node::Atom(string(&v.to_string())))
    }
    fn serialize_str(self, v: &str) -> Result<Node, Error> {
        Ok(Node::Atom(string(v)))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Node, Error> {
        Ok(Node::Array(v.iter().map(|b| Node::Atom(b.to_string())).collect()))
    }
    fn serialize_none(self) -> Result<Node, Error> {
        Ok(Node::None)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Node, Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Node, Error> {
        Ok(Node::Atom(String::new()))
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Node, Error> {
        Ok(Node::Atom(String::new()))
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Node, Error> {
        Ok(Node::Atom(string(variant)))
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<Node, Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Node, Error> {
        Ok(Node::Object(vec![(string(variant), value.serialize(Builder)?)]))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Seq, Error> {
        Ok(Seq { items: Vec::with_capacity(len.unwrap_or(0)), variant: None })
    }
    fn serialize_tuple(self, len: usize) -> Result<Seq, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Seq, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, variant: &'static str, len: usize) -> Result<Seq, Error> {
        Ok(Seq { items: Vec::with_capacity(len), variant: Some(string(variant)) })
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Map, Error> {
        Ok(Map { fields: vec![], key: None })
    }
    fn serialize_struct(self, name: &'static str, _: usize) -> Result<Struct, Error> {
        Ok(Struct { fields: vec![], range: matches!(name, "Range" | "RangeInclusive"), variant: None })
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Struct, Error> {
        Ok(Struct { fields: vec![], range: false, variant: Some(string(variant)) })
    }
}

/// A variant with a value is an object of one field named after the variant.
fn variant(name: Option<String>, node: Node) -> Node {
    match name {
        Some(name) => Node::Object(vec![(name, node)]),
        None => node,
    }
}

struct Seq {
    items: Vec<Node>,
    variant: Option<String>,
}

impl Seq {
    fn push<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.items.push(value.serialize(Builder)?);
        Ok(())
    }
    fn done(self) -> Result<Node, Error> {
        Ok(variant(self.variant, Node::Array(self.items)))
    }
}

impl ser::SerializeSeq for Seq {
    type Ok = Node;
    type Error = Error;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

impl ser::SerializeTuple for Seq {
    type Ok = Node;
    type Error = Error;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

impl ser::SerializeTupleStruct for Seq {
    type Ok = Node;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

impl ser::SerializeTupleVariant for Seq {
    type Ok = Node;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

struct Map {
    fields: Vec<(String, Node)>,
    key: Option<String>,
}

impl ser::SerializeMap for Map {
    type Ok = Node;
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        match key.serialize(Builder)? {
            Node::Atom(key) => self.key = Some(key),
            _ => return Err(Error("a map key must be a string, a number, a boolean or a unit variant".into())),
        }
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        let key = self.key.take().ok_or_else(|| Error("a map value came before its key".into()))?;
        self.fields.push((key, value.serialize(Builder)?));
        Ok(())
    }
    fn end(self) -> Result<Node, Error> {
        Ok(Node::Object(self.fields))
    }
}

struct Struct {
    fields: Vec<(String, Node)>,
    range: bool,
    variant: Option<String>,
}

impl Struct {
    fn push<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error> {
        self.fields.push((string(key), value.serialize(Builder)?));
        Ok(())
    }
    fn done(self) -> Result<Node, Error> {
        let node = match <[_; 2]>::try_from(self.fields) {
            Ok([(start, Node::Atom(a)), (end, Node::Atom(b))]) if self.range && start == "start" && end == "end" => {
                Node::Range(a, b)
            }
            Ok(fields) => Node::Object(fields.into()),
            Err(fields) => Node::Object(fields),
        };
        Ok(variant(self.variant, node))
    }
}

impl ser::SerializeStruct for Struct {
    type Ok = Node;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error> {
        self.push(key, value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

impl ser::SerializeStructVariant for Struct {
    type Ok = Node;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error> {
        self.push(key, value)
    }
    fn end(self) -> Result<Node, Error> {
        self.done()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::collections::BTreeMap;

    fn write(value: impl Serialize) -> String {
        to_string(&value).unwrap()
    }

    #[derive(Serialize)]
    struct Field<T> {
        x: T,
    }

    fn field<T: Serialize>(x: T) -> String {
        write(Field { x })
    }

    // Strings

    #[test]
    fn plain_string_is_written_without_quotes() {
        // Given / When: a string of plain characters / Then: it is written without quotes
        assert_eq!(field("john@example.com"), "x=john@example.com");
    }

    #[test]
    fn empty_string_is_written_as_quotes() {
        // Given / When: an empty string / Then: it is written as ""
        assert_eq!(field(""), r#"x="""#);
    }

    #[test]
    fn string_with_whitespace_is_written_in_quotes() {
        // Given / When: a string holding a space / Then: it is written in quotes
        assert_eq!(field("Hello World"), r#"x="Hello World""#);
    }

    #[test]
    fn string_with_separator_or_bracket_is_written_in_quotes() {
        // Given / When: strings holding =, ',', (, ) and " / Then: each is written in quotes
        for s in ["a=b", "a,b", "a(b", "a)b"] {
            assert_eq!(field(s), format!(r#"x="{s}""#));
        }
        assert_eq!(field("a\"b"), r#"x="a\"b""#);
    }

    #[test]
    fn quote_and_backslash_inside_quotes_are_escaped() {
        // Given / When: a quoted string holding " and \ / Then: each is written with a \ before it
        assert_eq!(field(r#"say "\""#), r#"x="say \"\\\"""#);
    }

    #[test]
    fn newline_inside_quotes_is_written_as_newline() {
        // Given / When: a string holding a newline / Then: the newline is written as is, in quotes
        assert_eq!(field("Hello\nWorld"), "x=\"Hello\nWorld\"");
    }

    #[test]
    fn string_with_dash_or_colon_is_written_without_quotes() {
        // Given / When: a file name and a datetime / Then: they are written without quotes
        assert_eq!(field("a-b.rs"), "x=a-b.rs");
        assert_eq!(field("2026-11-02T23:06:00+09:00"), "x=2026-11-02T23:06:00+09:00");
    }

    // Numbers and booleans

    #[test]
    fn integer_is_written_as_is() {
        // Given / When: integers / Then: they are written as is
        assert_eq!(field(20), "x=20");
        assert_eq!(field(-3), "x=-3");
    }

    #[test]
    fn float_equal_to_integer_is_written_without_fraction() {
        // Given / When: the float 1.0 / Then: it is written 1
        assert_eq!(field(1.0), "x=1");
    }

    #[test]
    fn float_is_written_in_shortest_form() {
        // Given / When: floats / Then: each is written in its shortest form that reads back the same
        assert_eq!(field(0.1), "x=0.1");
        assert_eq!(field(1e100), "x=1e100");
        assert_eq!(field(1.5f32), "x=1.5");
    }

    #[test]
    fn nan_and_infinities_are_written_by_name() {
        // Given / When: NaN and the infinities / Then: they are written NaN, inf and -inf
        assert_eq!(field(f64::NAN), "x=NaN");
        assert_eq!(field(f64::INFINITY), "x=inf");
        assert_eq!(field(f64::NEG_INFINITY), "x=-inf");
    }

    #[test]
    fn boolean_is_written_true_or_false() {
        // Given / When: booleans / Then: they are written true and false
        assert_eq!(field(true), "x=true");
        assert_eq!(field(false), "x=false");
    }

    // Ranges

    #[test]
    fn range_inclusive_is_written_as_ends_joined_by_dash() {
        // Given / When: an inclusive range / Then: it is written as its ends joined by -
        assert_eq!(field(1..=2), "x=1-2");
        assert_eq!(field(-3..=-1), "x=-3--1");
    }

    #[test]
    fn range_is_written_as_ends_joined_by_dash() {
        // Given / When: a half-open range / Then: it is written as its ends joined by -
        assert_eq!(field(1..3), "x=1-3");
    }

    #[test]
    fn array_of_ranges_is_written_without_parentheses() {
        // Given / When: an array of ranges / Then: it is written without parentheses
        assert_eq!(field(vec![1..=2, 3..=4]), "x=1-2,3-4");
    }

    // Objects

    #[derive(Serialize)]
    struct Person {
        name: &'static str,
        age: u32,
    }

    #[test]
    fn struct_is_written_as_fields_separated_by_spaces() {
        // Given / When: a struct / Then: it is written as key=value fields separated by spaces
        assert_eq!(write(Person { name: "John", age: 20 }), "name=John age=20");
    }

    #[test]
    fn map_is_written_like_struct() {
        // Given / When: a map / Then: it is written like a struct
        assert_eq!(write(BTreeMap::from([("a", 1), ("b", 2)])), "a=1 b=2");
    }

    #[test]
    fn key_needing_quotes_is_written_in_quotes() {
        // Given / When: a map whose key holds = / Then: the key is written in quotes
        assert_eq!(write(BTreeMap::from([("key=key", "value")])), r#""key=key"=value"#);
    }

    #[test]
    fn struct_as_field_value_is_written_in_parentheses() {
        // Given / When: a struct holding structs / Then: the inner structs are written in parentheses
        #[derive(Serialize)]
        struct Family {
            child: Person,
            parent: Person,
        }
        let family = Family { child: Person { name: "foo", age: 10 }, parent: Person { name: "bar", age: 42 } };
        assert_eq!(write(family), "child=(name=foo age=10) parent=(name=bar age=42)");
    }

    #[test]
    fn none_field_is_left_out() {
        // Given / When: a struct whose field is None / Then: the field is left out
        #[derive(Serialize)]
        struct Note {
            a: Option<u32>,
            b: u32,
        }
        assert_eq!(write(Note { a: None, b: 1 }), "b=1");
    }

    // Arrays

    #[test]
    fn array_is_written_as_elements_separated_by_commas() {
        // Given / When: an array / Then: its elements are separated by ,
        assert_eq!(field(vec![1, 2, 3]), "x=1,2,3");
    }

    #[test]
    fn single_element_array_is_written_as_its_element() {
        // Given / When: an array holding one array / Then: it is written as the inner array
        assert_eq!(field(vec![vec![1, 2, 3]]), "x=1,2,3");
    }

    #[test]
    fn nested_single_element_arrays_are_written_as_innermost_value() {
        // Given / When: single-element arrays nested four deep / Then: only the innermost value is written
        assert_eq!(field(vec![vec![vec![vec![1]]]]), "x=1");
    }

    #[test]
    fn array_and_object_elements_of_longer_array_are_written_in_parentheses() {
        // Given / When: arrays of arrays and of structs / Then: their elements are written in parentheses
        assert_eq!(field(vec![vec![1, 2, 3], vec![4, 5, 6]]), "x=(1,2,3),(4,5,6)");
        assert_eq!(field(vec![vec![1], vec![2]]), "x=(1),(2)");
        assert_eq!(
            field(vec![Person { name: "a", age: 1 }, Person { name: "b", age: 2 }]),
            "x=(name=a age=1),(name=b age=2)"
        );
    }

    #[test]
    fn none_element_is_written_as_nothing() {
        // Given / When: an array with None in the middle / Then: it is written as nothing
        assert_eq!(field(vec![Some(1), None, Some(3)]), "x=1,,3");
    }

    #[test]
    fn empty_array_field_is_written_as_nothing() {
        // Given / When: a field holding an empty array / Then: nothing is written after =
        assert_eq!(field(Vec::<u32>::new()), "x=");
    }

    #[test]
    fn empty_array_element_of_longer_array_is_written_as_nothing() {
        // Given / When: an empty array beside another / Then: it is written as nothing
        assert_eq!(field(vec![vec![], vec![1]]), "x=,(1)");
    }

    #[test]
    fn array_whose_only_element_is_nil_is_written_as_nothing() {
        // Given / When: arrays whose only element is nil, nested to any depth / Then: they are written as nothing
        assert_eq!(field(vec![None::<u32>]), "x=");
        assert_eq!(field(vec![Vec::<u32>::new()]), "x=");
        assert_eq!(field(vec![vec![None::<u32>]]), "x=");
    }

    #[test]
    fn some_is_written_as_its_value() {
        // Given / When: Some holding a value / Then: it is written as the value
        assert_eq!(field(Some(Some(5))), "x=5");
    }

    // Enums

    #[derive(Serialize)]
    enum Shape {
        Dot,
        Circle(u32),
        Line(u32, u32),
        Rect { w: u32, h: u32 },
    }

    #[test]
    fn variant_without_value_is_written_as_name() {
        // Given / When: a unit variant / Then: it is written as its name
        assert_eq!(field(Shape::Dot), "x=Dot");
    }

    #[test]
    fn variant_with_one_value_is_written_as_name_and_value() {
        // Given / When: a newtype variant at the top level / Then: it is written Name=value
        assert_eq!(write(Shape::Circle(2)), "Circle=2");
    }

    #[test]
    fn variant_with_several_values_is_written_as_name_and_array() {
        // Given / When: a tuple variant at the top level / Then: it is written Name= and the values as an array
        assert_eq!(write(Shape::Line(1, 2)), "Line=1,2");
    }

    #[test]
    fn variant_with_fields_is_written_as_name_and_fields_in_parentheses() {
        // Given / When: a struct variant at the top level / Then: it is written Name= and its fields in parentheses
        assert_eq!(write(Shape::Rect { w: 1, h: 2 }), "Rect=(w=1 h=2)");
    }

    #[test]
    fn variant_with_value_as_field_value_is_written_in_parentheses() {
        // Given / When: a newtype variant as a field's value / Then: it is written in parentheses
        assert_eq!(field(Shape::Circle(2)), "x=(Circle=2)");
    }

    // Top level

    #[test]
    fn struct_at_top_level_is_written_on_one_line_without_parentheses() {
        // Given / When: a struct at the top level / Then: it is one line without parentheses
        assert_eq!(write(Person { name: "John", age: 20 }), "name=John age=20");
    }

    #[test]
    fn longer_array_at_top_level_is_written_one_element_per_line() {
        // Given / When: an array of two structs at the top level / Then: each is a line
        #[derive(Serialize)]
        struct Row {
            foo: u32,
            bar: Vec<u32>,
        }
        let rows = vec![Row { foo: 1, bar: vec![1, 1] }, Row { foo: 2, bar: vec![2, 2] }];
        assert_eq!(write(rows), "foo=1 bar=1,1\nfoo=2 bar=2,2");
    }

    #[test]
    fn elements_of_top_level_array_are_written_without_parentheses() {
        // Given / When: an array of arrays at the top level / Then: each element is a line without parentheses
        assert_eq!(write(vec![vec![1, 2], vec![3, 4]]), "1,2\n3,4");
    }
}
