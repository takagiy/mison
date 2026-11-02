//! Reading: the text is parsed into a [`Syntax`] tree without types, then serde reads the tree through the
//! type it asks for, which decides how each part of the tree is taken.

use std::cell::Cell;

use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};

use crate::Error;

/// Reads `text` as MISON through the type `T`, which decides how each part of the text is taken.
pub fn from_str<T: de::DeserializeOwned>(text: &str) -> Result<T, Error> {
    let syntax = top(text)?;
    T::deserialize(De { node: &syntax, slot: Slot::Line })
}

fn error(message: impl Into<String>) -> Error {
    Error(message.into())
}

/// MISON's syntax, as far as it can be read without types.
#[derive(Debug)]
enum Syntax {
    /// Nothing between separators: nil.
    Nothing,
    /// A token outside quotes: a number, a boolean, a string, a range, a variant's name.
    Bare(String),
    /// A string in quotes, unescaped.
    Quoted(String),
    /// Two or more values separated by `,`.
    List(Vec<Syntax>),
    /// `(...)`.
    Group(Box<Syntax>),
    /// `key=value` fields separated by whitespace, keys unquoted.
    Fields(Vec<(String, Syntax)>),
    /// Two or more top-level lines.
    Lines(Vec<Syntax>),
}

// Parsing.

/// Splits `text` at the characters `at` matches, outside quotes and parentheses.
fn split(text: &str, at: impl Fn(char) -> bool) -> Result<Vec<&str>, Error> {
    let (mut parts, mut start, mut depth, mut quoted, mut escaped) = (vec![], 0, 0usize, false, false);
    for (i, c) in text.char_indices() {
        if quoted {
            match c {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => quoted = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => quoted = true,
            '(' => depth += 1,
            ')' => depth = depth.checked_sub(1).ok_or_else(|| error(format!("unmatched ) in {text:?}")))?,
            c if depth == 0 && at(c) => {
                parts.push(&text[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    if quoted || depth > 0 {
        return Err(error(format!("unclosed quote or parenthesis in {text:?}")));
    }
    parts.push(&text[start..]);
    Ok(parts)
}

/// The inside of `text` when all of it is one group.
fn group(text: &str) -> Result<Option<&str>, Error> {
    let Some(inner) = text.strip_prefix('(').and_then(|t| t.strip_suffix(')')) else { return Ok(None) };
    // `(a),(b)` starts and ends with parentheses but is two groups: its inside does not balance on its own.
    let balanced = split(inner, |_| false).is_ok();
    Ok(balanced.then_some(inner))
}

/// The unescaped string when all of `text` is one quoted string.
fn quoted(text: &str) -> Option<String> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => out.push(chars.next()?),
            '"' => return None,
            c => out.push(c),
        }
    }
    Some(out)
}

fn top(text: &str) -> Result<Syntax, Error> {
    let text = text.strip_suffix('\n').unwrap_or(text);
    let lines = split(text, |c| c == '\n')?;
    if lines.len() < 2 {
        return slot(text);
    }
    lines
        .into_iter()
        .map(|line| slot(line.strip_suffix('\r').unwrap_or(line)))
        .collect::<Result<_, _>>()
        .map(Syntax::Lines)
}

/// A line, or the inside of a group: fields when it has an `=`, otherwise a value.
fn slot(text: &str) -> Result<Syntax, Error> {
    if split(text, |c| c == '=')?.len() < 2 {
        return value(text);
    }
    let mut fields = vec![];
    for token in split(text, char::is_whitespace)?.into_iter().filter(|t| !t.is_empty()) {
        let key = split(token, |c| c == '=')?[0];
        let rest = token.get(key.len() + 1..).ok_or_else(|| error(format!("expected key=value, found {token:?}")))?;
        fields.push((quoted(key).unwrap_or_else(|| key.into()), value(rest)?));
    }
    Ok(Syntax::Fields(fields))
}

fn value(text: &str) -> Result<Syntax, Error> {
    let parts = split(text, |c| c == ',')?;
    if parts.len() < 2 {
        return element(text);
    }
    parts.into_iter().map(element).collect::<Result<_, _>>().map(Syntax::List)
}

fn element(text: &str) -> Result<Syntax, Error> {
    if text.is_empty() {
        return Ok(Syntax::Nothing);
    }
    if let Some(inner) = group(text)? {
        return Ok(Syntax::Group(Box::new(slot(inner)?)));
    }
    Ok(quoted(text).map_or_else(|| Syntax::Bare(text.into()), Syntax::Quoted))
}

// Reading through types.

/// Where a value sits, which decides what a group around it means.
#[derive(Clone, Copy, PartialEq)]
enum Slot {
    /// A top-level line: an object there has no parentheses.
    Line,
    /// A field's value, or the only element of an array written in its place: a group there around an
    /// array's text is that array's one element.
    Field,
    /// An element of an array of two or more: a group there only marks where the element starts and ends.
    Element,
}

#[derive(Clone, Copy)]
struct De<'s> {
    node: &'s Syntax,
    slot: Slot,
}

impl<'s> De<'s> {
    /// The text of a scalar.
    fn text(self) -> Result<&'s str, Error> {
        match self.node {
            Syntax::Bare(s) | Syntax::Quoted(s) => Ok(s),
            Syntax::Group(inner) => De { node: inner, slot: Slot::Field }.text(),
            other => Err(error(format!("expected a single value, found {other:?}"))),
        }
    }

    fn parse<T: std::str::FromStr>(self, what: &str) -> Result<T, Error> {
        let text = self.text()?;
        text.parse().map_err(|_| error(format!("expected {what}, found {text:?}")))
    }

    /// The elements of an array, each with where it sits, and the list they came from when they were
    /// written separated by `,`.
    fn items(self) -> (Vec<De<'s>>, Option<&'s Syntax>) {
        match (self.node, self.slot) {
            (Syntax::Nothing, _) => (vec![], None),
            (list @ Syntax::List(items), _) => {
                (items.iter().map(|node| De { node, slot: Slot::Element }).collect(), Some(list))
            }
            (Syntax::Lines(items), _) => (items.iter().map(|node| De { node, slot: Slot::Line }).collect(), None),
            (Syntax::Group(inner), Slot::Element) => De { node: inner, slot: Slot::Field }.items(),
            (Syntax::Group(inner), _) => (vec![De { node: inner, slot: Slot::Field }], None),
            (node, slot) => (vec![De { node, slot: if slot == Slot::Line { Slot::Line } else { Slot::Field } }], None),
        }
    }

    /// The fields of an object.
    fn fields(self) -> Result<&'s [(String, Syntax)], Error> {
        match self.node {
            Syntax::Fields(fields) => Ok(fields),
            Syntax::Group(inner) => De { node: inner, slot: Slot::Field }.fields(),
            Syntax::Nothing => Ok(&[]),
            other => Err(error(format!("expected an object, found {other:?}"))),
        }
    }
}

macro_rules! numbers {
    ($($method:ident $visit:ident $type:ty),*) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            visitor.$visit(self.parse::<$type>(stringify!($type))?)
        }
    )*};
}

impl<'de> de::Deserializer<'de> for De<'_> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Error> {
        Err(error("MISON is read through a type, and this type asks the text what it holds"))
    }

    numbers!(
        deserialize_i8 visit_i8 i8, deserialize_i16 visit_i16 i16, deserialize_i32 visit_i32 i32,
        deserialize_i64 visit_i64 i64, deserialize_i128 visit_i128 i128, deserialize_u8 visit_u8 u8,
        deserialize_u16 visit_u16 u16, deserialize_u32 visit_u32 u32, deserialize_u64 visit_u64 u64,
        deserialize_u128 visit_u128 u128, deserialize_f32 visit_f32 f32, deserialize_f64 visit_f64 f64,
        deserialize_bool visit_bool bool, deserialize_char visit_char char
    );

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_str(self.text()?)
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_str(visitor)
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_str(visitor)
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.node {
            Syntax::Nothing => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }
    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.node {
            Syntax::Nothing => visitor.visit_unit(),
            other => Err(error(format!("expected nothing, found {other:?}"))),
        }
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(self, _: &'static str, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_unit(visitor)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _: &'static str, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        let (items, list) = self.items();
        visitor.visit_seq(Items { items: items.into_iter(), list, done: false })
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, _: usize, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_map(Fields { fields: self.fields()?.iter(), value: None })
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        let is_range = matches!(name, "Range" | "RangeInclusive") && fields == ["start", "end"];
        if !is_range {
            return self.deserialize_map(visitor);
        }
        // A range is its ends joined by `-`; a leading `-` is the start's sign.
        let text = self.text()?;
        let at = text.char_indices().skip(1).find(|&(_, c)| c == '-').map(|(i, _)| i);
        let at = at.ok_or_else(|| error(format!("expected a range, found {text:?}")))?;
        let ends = [
            ("start".to_string(), Syntax::Bare(text[..at].into())),
            ("end".to_string(), Syntax::Bare(text[at + 1..].into())),
        ];
        visitor.visit_map(Fields { fields: ends.iter(), value: None })
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        if let Ok(name) = self.text() {
            return visitor.visit_enum(name.into_deserializer());
        }
        match self.fields()? {
            [(name, value)] => visitor.visit_enum(Variant { name, value: De { node: value, slot: Slot::Field } }),
            _ => Err(error("expected a variant: a name, or one name=value")),
        }
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }
}

struct Items<'s> {
    items: std::vec::IntoIter<De<'s>>,
    /// The `,` list the items came from, until the first item is read.
    list: Option<&'s Syntax>,
    /// The first item took the whole list.
    done: bool,
}

impl<'de> de::SeqAccess<'de> for Items<'_> {
    type Error = Error;
    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Error> {
        let Some(item) = self.items.next().filter(|_| !self.done) else { return Ok(None) };
        let Some(list) = self.list.take() else { return seed.deserialize(item).map(Some) };
        let took_list = Cell::new(false);
        let value = seed.deserialize(First { item, list, took_list: &took_list })?;
        self.done = took_list.get();
        Ok(Some(value))
    }
}

/// The first item of a `,` list. An array or object element of a list of two or more is written in
/// parentheses, so when the type asks this item for one and it has none, the list is not elements at all:
/// it is the text of an array's only element, written in its place, and the item takes the whole list.
struct First<'a, 's> {
    item: De<'s>,
    list: &'s Syntax,
    took_list: &'a Cell<bool>,
}

impl<'s> First<'_, 's> {
    fn compound(self) -> De<'s> {
        if !matches!(self.item.node, Syntax::Bare(_) | Syntax::Quoted(_)) {
            return self.item;
        }
        self.took_list.set(true);
        De { node: self.list, slot: Slot::Field }
    }
}

macro_rules! to_item {
    ($($method:ident),*) => {$(
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            self.item.$method(visitor)
        }
    )*};
}

impl<'de> de::Deserializer<'de> for First<'_, '_> {
    type Error = Error;

    to_item!(
        deserialize_any,
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_identifier,
        deserialize_unit,
        deserialize_ignored_any
    );

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.compound().deserialize_bytes(visitor)
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.compound().deserialize_byte_buf(visitor)
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.item.node {
            Syntax::Nothing => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value, Error> {
        self.item.deserialize_unit_struct(name, visitor)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _: &'static str, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.compound().deserialize_seq(visitor)
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        self.compound().deserialize_tuple(len, visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.compound().deserialize_tuple_struct(name, len, visitor)
    }
    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.compound().deserialize_map(visitor)
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        // A range is written bare even as an element, so it never takes the list.
        if matches!(name, "Range" | "RangeInclusive") {
            return self.item.deserialize_struct(name, fields, visitor);
        }
        self.compound().deserialize_struct(name, fields, visitor)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.item.deserialize_enum(name, variants, visitor)
    }
}
struct Fields<'s> {
    fields: std::slice::Iter<'s, (String, Syntax)>,
    value: Option<&'s Syntax>,
}

impl<'de> de::MapAccess<'de> for Fields<'_> {
    type Error = Error;
    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Error> {
        let Some((key, value)) = self.fields.next() else { return Ok(None) };
        self.value = Some(value);
        // A key reads like a bare value, so a map's keys may be numbers as well as strings.
        let key = Syntax::Bare(key.clone());
        seed.deserialize(De { node: &key, slot: Slot::Field }).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Error> {
        let node = self.value.take().ok_or_else(|| error("a value was asked for before its key"))?;
        seed.deserialize(De { node, slot: Slot::Field })
    }
}

struct Variant<'s> {
    name: &'s str,
    value: De<'s>,
}

impl<'de, 's> de::EnumAccess<'de> for Variant<'s> {
    type Error = Error;
    type Variant = De<'s>;
    fn variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<(T::Value, De<'s>), Error> {
        let name: de::value::StrDeserializer<Error> = self.name.into_deserializer();
        Ok((seed.deserialize(name)?, self.value))
    }
}

impl<'de> de::VariantAccess<'de> for De<'_> {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Error> {
        seed.deserialize(self)
    }
    fn tuple_variant<V: Visitor<'de>>(self, _: usize, visitor: V) -> Result<V::Value, Error> {
        de::Deserializer::deserialize_seq(self, visitor)
    }
    fn struct_variant<V: Visitor<'de>>(self, _: &'static [&'static str], visitor: V) -> Result<V::Value, Error> {
        de::Deserializer::deserialize_map(self, visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_string;
    use serde::{Deserialize, Serialize, de::DeserializeOwned};
    use std::collections::BTreeMap;
    use std::fmt::Debug;
    use std::ops::{Range, RangeInclusive};

    /// Writes `value` and reads it back through its type.
    fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: T) {
        let text = to_string(&value).unwrap();
        assert_eq!(from_str::<T>(&text).unwrap(), value, "{text:?}");
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Field<T> {
        x: T,
    }

    fn read<T: DeserializeOwned>(text: &str) -> T {
        from_str::<Field<T>>(text).unwrap().x
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Shape {
        Dot,
        Circle(u32),
        Line(u32, u32),
        Rect { w: u32, h: u32 },
    }

    // The type as the schema

    #[test]
    fn written_values_read_back_through_their_types() {
        // Given / When: values of many shapes, written and read back / Then: each reads back the same
        round_trip(Person { name: "John".into(), age: 20 });
        round_trip(Field { x: "Hello \"big\"\nWorld\\".to_string() });
        round_trip(Field { x: String::new() });
        round_trip(Field { x: vec![vec![1, 2, 3], vec![4, 5, 6]] });
        round_trip(Field { x: vec![vec![1], vec![2]] });
        round_trip(Field { x: vec![vec![vec![vec![1]]]] });
        round_trip(Field { x: vec![Some(1), None, Some(3)] });
        round_trip(Field { x: vec![None::<u32>, None] });
        round_trip(Field { x: vec![vec![], vec![1]] });
        round_trip(Field { x: vec![1..=2, 3..=4] });
        round_trip(Field { x: vec![Person { name: "a".into(), age: 1 }, Person { name: "b b".into(), age: 2 }] });
        round_trip(BTreeMap::from([("key=key".to_string(), 1), ("b".to_string(), 2)]));
        round_trip(BTreeMap::from([(1, "one".to_string())]));
        round_trip(vec![Shape::Dot, Shape::Circle(2), Shape::Line(1, 2), Shape::Rect { w: 1, h: 2 }]);
        round_trip(Field { x: Shape::Rect { w: 1, h: 2 } });
        round_trip(Field { x: (1.5, -2, true, 'c') });
        round_trip(vec![Person { name: "a".into(), age: 1 }, Person { name: "b".into(), age: 2 }]);
        round_trip(vec![vec![1, 2], vec![3, 4]]);
        round_trip(Field { x: Vec::<u32>::new() });
        round_trip(Field { x: () });
        round_trip(Field { x: vec![vec![1, 2]] });
        round_trip(Field { x: vec![vec![vec![1, 2]]] });
        round_trip(Field { x: vec![Person { name: "a".into(), age: 1 }] });
        round_trip(vec![Person { name: "a".into(), age: 1 }]);
    }

    #[test]
    fn text_reads_as_number_or_string_by_type() {
        // Given / When: x=20 read into a number and into a string / Then: 20 and "20"
        assert_eq!(read::<u32>("x=20"), 20);
        assert_eq!(read::<String>("x=20"), "20");
    }

    #[test]
    fn reading_into_type_that_asks_text_is_error() {
        // Given: an untagged enum, which asks the text what it holds
        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        #[allow(dead_code)]
        enum Either {
            Number(u32),
            Text(String),
        }
        // When / Then: reading into it is an error
        assert!(from_str::<Either>("1").is_err());
    }

    // Strings

    #[test]
    fn unquoted_string_reads_up_to_next_separator() {
        // Given / When: two fields of unquoted strings / Then: each reads up to the space
        let person: Person = from_str("name=John age=20").unwrap();
        assert_eq!(person, Person { name: "John".into(), age: 20 });
    }

    #[test]
    fn quoted_string_reads_unescaped_with_newlines() {
        // Given / When: a quoted string with \", \\ and a newline / Then: it reads unescaped, newline kept
        assert_eq!(read::<String>("x=\"a \\\"b\\\\\nc\""), "a \"b\\\nc");
    }

    // Numbers

    #[test]
    fn integer_reads_into_float() {
        // Given / When: x=1 read into a float / Then: 1.0
        assert_eq!(read::<f64>("x=1"), 1.0);
    }

    #[test]
    fn nan_and_infinities_read_into_float() {
        // Given / When: NaN, inf and -inf / Then: they read as NaN and the infinities
        assert!(read::<f64>("x=NaN").is_nan());
        assert_eq!(read::<f64>("x=inf"), f64::INFINITY);
        assert_eq!(read::<f64>("x=-inf"), f64::NEG_INFINITY);
    }

    // Ranges

    #[test]
    fn range_reads_into_both_range_types() {
        // Given / When: x=1-3 read into RangeInclusive and Range / Then: both have ends 1 and 3
        assert_eq!(read::<RangeInclusive<u32>>("x=1-3"), 1..=3);
        assert_eq!(read::<Range<u32>>("x=1-3"), 1..3);
    }

    #[test]
    fn range_with_negative_ends_reads_with_signs() {
        // Given / When: x=-3--1 / Then: the ends are -3 and -1
        assert_eq!(read::<RangeInclusive<i32>>("x=-3--1"), -3..=-1);
    }

    // Objects

    #[test]
    fn left_out_field_reads_as_none() {
        // Given / When: a text without the Option field / Then: the field reads as None
        #[derive(Deserialize, PartialEq, Debug)]
        struct Note {
            a: Option<u32>,
            b: u32,
        }
        assert_eq!(from_str::<Note>("b=1").unwrap(), Note { a: None, b: 1 });
    }

    #[test]
    fn field_in_parentheses_reads_into_struct_field() {
        // Given / When: a struct field written in parentheses / Then: it reads into the struct
        assert_eq!(read::<Person>("x=(name=foo age=10)"), Person { name: "foo".into(), age: 10 });
    }

    // Arrays

    #[test]
    fn value_without_brackets_reads_into_nested_arrays() {
        // Given / When: x=1 read into arrays three deep / Then: [[[1]]]
        assert_eq!(read::<Vec<Vec<Vec<u32>>>>("x=1"), vec![vec![vec![1]]]);
    }

    #[test]
    fn elements_without_parentheses_read_as_one_inner_array() {
        // Given / When: x=1,2 read into Vec<Vec<u32>> / Then: one inner array [1,2]
        assert_eq!(read::<Vec<Vec<u32>>>("x=1,2"), vec![vec![1, 2]]);
    }

    #[test]
    fn nothing_reads_as_none_for_option() {
        // Given / When: nothing between separators read into Option elements / Then: None
        assert_eq!(read::<Vec<Option<u32>>>("x=1,,3"), vec![Some(1), None, Some(3)]);
    }

    #[test]
    fn nothing_reads_as_empty_array_for_array() {
        // Given / When: nothing between separators read into array elements / Then: an empty array
        assert_eq!(read::<Vec<Vec<u32>>>("x=,(1)"), vec![vec![], vec![1]]);
    }

    #[test]
    fn array_whose_only_element_is_nil_reads_back_as_empty_array() {
        // Given / When: [None] and [[None]] written and read back / Then: both read as empty arrays
        let one = to_string(&Field { x: vec![None::<u32>] }).unwrap();
        assert_eq!(from_str::<Field<Vec<Option<u32>>>>(&one).unwrap().x, vec![]);
        let nested = to_string(&Field { x: vec![vec![None::<u32>]] }).unwrap();
        assert_eq!(from_str::<Field<Vec<Vec<Option<u32>>>>>(&nested).unwrap().x, Vec::<Vec<Option<u32>>>::new());
    }

    #[test]
    fn some_of_nothing_reads_back_as_none() {
        // Given / When: Some(vec![]) and Some(None) written and read back / Then: both read as None
        let empty = to_string(&Field { x: Some(Vec::<u32>::new()) }).unwrap();
        assert_eq!(from_str::<Field<Option<Vec<u32>>>>(&empty).unwrap().x, None);
        let nested = to_string(&Field { x: Some(None::<u32>) }).unwrap();
        assert_eq!(from_str::<Field<Option<Option<u32>>>>(&nested).unwrap().x, None);
    }

    // Top level

    #[test]
    fn each_top_level_line_reads_as_element() {
        // Given / When: two lines read into an array of structs / Then: one struct per line
        let people: Vec<Person> = from_str("name=a age=1\nname=b age=2").unwrap();
        assert_eq!(people, vec![Person { name: "a".into(), age: 1 }, Person { name: "b".into(), age: 2 }]);
    }

    #[test]
    fn newline_inside_quotes_does_not_start_element() {
        // Given / When: one line whose quoted string holds a newline / Then: it reads as one element
        let notes: Vec<String> = from_str("\"a\nb\"").unwrap();
        assert_eq!(notes, vec!["a\nb".to_string()]);
    }
}
