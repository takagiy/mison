use mison::{Decode, Encode, from_str, to_string};
use std::fmt::Debug;
use std::ops::RangeInclusive;

/// Writes `value`, and checks that the text reads back as it.
fn round_trip<T: Decode + PartialEq + Debug>(value: T) -> String {
    let text = to_string(&value);
    assert_eq!(from_str::<T>(&text).as_ref(), Ok(&value), "{text:?}");
    text
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Report {
    ok: bool,
    file: String,
    ranges: Vec<RangeInclusive<usize>>,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Person {
    name: Option<String>,
    age: u32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Wrap<T> {
    x: T,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Shape {
    Point,
    Circle(f64),
    Line(i32, i32),
    Rect { width: u32, height: u32 },
}

/// Field names that the derived code also uses for its own variables.
#[derive(Encode, Decode, PartialEq, Debug)]
enum Names {
    V { w: u8, r: u8, key: u8, x: u8 },
}
// Structs

#[test]
fn struct_is_record_keyed_by_field_names() {
    // Given: a struct with three fields / When: written / Then: it is a record keyed by the field names
    let report = Report { ok: true, file: "a.rs".into(), ranges: vec![10..=15, 20..=25] };
    assert_eq!(round_trip(report), "ok=true file=a.rs ranges=10-15,20-25");
}

#[test]
fn fields_read_in_any_order() {
    // Given: the fields of a struct in another order / When: read / Then: each field gets its value
    assert_eq!(from_str("age=3 name=Ann"), Ok(Person { name: Some("Ann".into()), age: 3 }));
}

#[test]
fn missing_unknown_or_repeated_key_is_error() {
    // Given: records missing a key, with an unknown key and with a key twice / When: read / Then: each is an error
    for text in ["name=Ann", "name=Ann age=3 x=1", "name=Ann age=3 age=4"] {
        assert!(from_str::<Person>(text).is_err(), "{text}");
    }
}

#[test]
fn none_field_is_written_empty_and_cannot_be_left_out() {
    // Given: a struct whose Option field is None / When: written and read
    // Then: it is written key=, and leaving the key out is an error
    assert_eq!(round_trip(Person { name: None, age: 3 }), "name= age=3");
    assert!(from_str::<Person>("age=3").is_err());
}

#[test]
fn generic_struct_implements_traits_when_its_parameters_do() {
    // Given: a generic struct holding an array / When: written / Then: it is a record and reads back
    assert_eq!(round_trip(Wrap { x: vec![1, 2] }), "x=1,2");
    assert_eq!(round_trip(Wrap { x: Wrap { x: 1 } }), "x=(x=1)");
}

// Enums

#[test]
fn unit_variant_is_its_name() {
    // Given: a unit variant / When: written / Then: it is the variant's name
    assert_eq!(round_trip(Shape::Point), "Point");
}

#[test]
fn variant_with_one_field_has_it_as_contents() {
    // Given: a variant with one unnamed field / When: written / Then: the field is the contents
    assert_eq!(round_trip(Shape::Circle(1.5)), "Circle(1.5)");
}

#[test]
fn variant_with_several_fields_has_tuple_as_contents() {
    // Given: a variant with two unnamed fields / When: written / Then: the contents are a tuple of them
    assert_eq!(round_trip(Shape::Line(1, -2)), "Line(1,-2)");
}

#[test]
fn variant_with_named_fields_has_record_as_contents() {
    // Given: a variant with named fields / When: written / Then: the contents are a record of them
    assert_eq!(round_trip(Shape::Rect { width: 1, height: 2 }), "Rect(width=1 height=2)");
}

#[test]
fn field_names_do_not_clash_with_derived_code() {
    // Given: a variant whose fields are named w, r, key and x / When: written / Then: it reads back
    assert_eq!(round_trip(Names::V { w: 1, r: 2, key: 3, x: 4 }), "V(w=1 r=2 key=3 x=4)");
}
#[test]
fn unknown_name_is_error() {
    // Given: a name that is not a variant of the enum / When: read / Then: it is an error
    assert!(from_str::<Shape>("Square(1)").is_err());
}

#[test]
fn variants_in_arrays_and_fields() {
    // Given: variants as array elements and as a field value / When: written / Then: they read back
    assert_eq!(
        round_trip(Wrap { x: vec![Shape::Point, Shape::Rect { width: 1, height: 2 }] }),
        "x=Point,Rect(width=1 height=2)"
    );
    assert_eq!(round_trip(vec![Shape::Circle(1.0), Shape::Line(1, 2)]), "Circle(1)\nLine(1,2)");
}
