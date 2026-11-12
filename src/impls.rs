//! `Encode` and `Decode` for the standard library's types.

use crate::{Decode, Encode, Error, Reader, Writer};
use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasher;
use std::ops::RangeInclusive;

macro_rules! int {
    ($($T:ty)*) => {$(
        impl Encode for $T {
            const HEIGHT: usize = 0;

            fn write(&self, w: &mut Writer) {
                w.literal(itoa::Buffer::new().format(*self));
            }
        }

        impl Decode for $T {
            fn read(r: &mut Reader) -> Result<Self, Error> {
                r.int()
            }
        }

        impl Encode for RangeInclusive<$T> {
            const HEIGHT: usize = 0;

            fn write(&self, w: &mut Writer) {
                let (mut start, mut end) = (itoa::Buffer::new(), itoa::Buffer::new());
                w.range(start.format(*self.start()), end.format(*self.end()));
            }
        }

        impl Decode for RangeInclusive<$T> {
            fn read(r: &mut Reader) -> Result<Self, Error> {
                let (start, end) = r.range()?;
                match (start.parse(), end.parse()) {
                    (Ok(start), Ok(end)) => Ok(start..=end),
                    _ => Err(r.error("range end out of range")),
                }
            }
        }
    )*};
}

int!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

macro_rules! float {
    ($($T:ty)*) => {$(
        impl Encode for $T {
            const HEIGHT: usize = 0;

            fn write(&self, w: &mut Writer) {
                let mut buffer = ryu::Buffer::new();
                let text = buffer.format(*self);
                w.literal(text.strip_suffix(".0").unwrap_or(text));
            }
        }

        impl Decode for $T {
            fn read(r: &mut Reader) -> Result<Self, Error> {
                r.float()
            }
        }
    )*};
}

float!(f32 f64);

impl Encode for bool {
    const HEIGHT: usize = 0;

    fn write(&self, w: &mut Writer) {
        w.literal(if *self { "true" } else { "false" });
    }
}

impl Decode for bool {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        r.boolean()
    }
}

impl Encode for str {
    const HEIGHT: usize = 0;

    fn write(&self, w: &mut Writer) {
        w.string(self);
    }
}

impl Encode for String {
    const HEIGHT: usize = 0;

    fn write(&self, w: &mut Writer) {
        w.string(self);
    }
}

impl Decode for String {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        Ok(r.string()?.into_owned())
    }
}

impl<T: Encode> Encode for Option<T> {
    const HEIGHT: usize = T::HEIGHT;

    fn write(&self, w: &mut Writer) {
        if let Some(x) = self {
            x.write(w);
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        if r.is_nil() { Ok(None) } else { T::read(r).map(Some) }
    }
}

impl<T: Encode + ?Sized> Encode for &T {
    const HEIGHT: usize = T::HEIGHT;

    fn write(&self, w: &mut Writer) {
        (**self).write(w);
    }
}

impl<T: Encode + ?Sized> Encode for Box<T> {
    const HEIGHT: usize = T::HEIGHT;

    fn write(&self, w: &mut Writer) {
        (**self).write(w);
    }
}

impl<T: Decode> Decode for Box<T> {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        T::read(r).map(Box::new)
    }
}

impl<T: Encode> Encode for [T] {
    const HEIGHT: usize = T::HEIGHT + 1;

    fn write(&self, w: &mut Writer) {
        w.seq(Self::HEIGHT, self.len(), |s| self.iter().for_each(|x| s.element(x)));
    }
}

impl<T: Encode> Encode for Vec<T> {
    const HEIGHT: usize = T::HEIGHT + 1;

    fn write(&self, w: &mut Writer) {
        self.as_slice().write(w);
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        let mut v = Vec::new();
        r.seq(Self::HEIGHT, |r| {
            v.push(T::read(r)?);
            Ok(())
        })?;
        Ok(v)
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    const HEIGHT: usize = T::HEIGHT + 1;

    fn write(&self, w: &mut Writer) {
        self.as_slice().write(w);
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn read(r: &mut Reader) -> Result<Self, Error> {
        let v = Vec::<T>::read(r)?;
        v.try_into().map_err(|v: Vec<T>| r.error(format_args!("expected {N} elements, not {}", v.len())))
    }
}

/// The largest of `heights`, in a constant.
const fn max<const N: usize>(heights: [usize; N]) -> usize {
    let (mut max, mut i) = (0, 0);
    while i < N {
        if heights[i] > max {
            max = heights[i];
        }
        i += 1;
    }
    max
}

macro_rules! tuple {
    ($n:literal: $($T:ident $i:tt),+) => {
        impl<$($T: Encode),+> Encode for ($($T,)+) {
            const HEIGHT: usize = max([$($T::HEIGHT),+]) + 1;

            fn write(&self, w: &mut Writer) {
                w.seq(Self::HEIGHT, $n, |s| { $(s.element(&self.$i);)+ });
            }
        }

        impl<$($T: Decode),+> Decode for ($($T,)+) {
            fn read(r: &mut Reader) -> Result<Self, Error> {
                let mut slots = ($(None::<$T>,)+);
                let mut n = 0;
                r.seq(Self::HEIGHT, |r| {
                    match n {
                        $($i => slots.$i = Some($T::read(r)?),)+
                        _ => return Err(r.error(concat!("expected ", $n, " elements"))),
                    }
                    n += 1;
                    Ok(())
                })?;
                Ok(($(slots.$i.ok_or_else(|| r.error(concat!("expected ", $n, " elements")))?,)+))
            }
        }
    };
}

tuple!(2: A 0, B 1);
tuple!(3: A 0, B 1, C 2);
tuple!(4: A 0, B 1, C 2, D 3);
tuple!(5: A 0, B 1, C 2, D 3, E 4);
tuple!(6: A 0, B 1, C 2, D 3, E 4, F 5);
tuple!(7: A 0, B 1, C 2, D 3, E 4, F 5, G 6);
tuple!(8: A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
tuple!(9: A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
tuple!(10: A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
tuple!(11: A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
tuple!(12: A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);

macro_rules! map {
    ($($M:ident $(, $S:ident: $bound:path)?);*) => {$(
        impl<V: Encode $(, $S: $bound)?> Encode for $M<String, V $(, $S)?> {
            const HEIGHT: usize = 0;

            fn write(&self, w: &mut Writer) {
                w.record(|r| self.iter().for_each(|(k, v)| r.field(k, v)));
            }
        }

        impl<V: Decode $(, $S: $bound + Default)?> Decode for $M<String, V $(, $S)?> {
            fn read(r: &mut Reader) -> Result<Self, Error> {
                let mut map = Self::default();
                if r.is_nil() {
                    return Ok(map);
                }
                r.record(|k, r| {
                    if map.contains_key(&*k) {
                        return Err(r.error(format_args!("key `{k}` written twice")));
                    }
                    let v = V::read(r)?;
                    map.insert(k.into_owned(), v);
                    Ok(())
                })?;
                Ok(map)
            }
        }
    )*};
}

map!(HashMap, S: BuildHasher; BTreeMap);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_str, to_string};
    use std::fmt::Debug;

    /// Writes `value`, and checks that the text reads back as it.
    fn round_trip<T: Decode + PartialEq + Debug>(value: T) -> String {
        let text = to_string(&value);
        assert_eq!(from_str::<T>(&text).as_ref(), Ok(&value), "{text:?}");
        text
    }

    fn map<V>(entries: impl IntoIterator<Item = (&'static str, V)>) -> BTreeMap<String, V> {
        entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    }

    // Heights

    #[test]
    fn arrays_add_one_to_height() {
        // Given: arrays nested one and two deep / When: their heights are taken / Then: each level adds one
        assert_eq!(<Vec<u8>>::HEIGHT, 1);
        assert_eq!(<Vec<Vec<u8>>>::HEIGHT, 2);
        assert_eq!(<[[u8; 2]; 3]>::HEIGHT, 2);
    }

    #[test]
    fn option_and_box_keep_height() {
        // Given: Option and Box around an array / When: their heights are taken / Then: they are the array's
        assert_eq!(<Option<Vec<u8>>>::HEIGHT, 1);
        assert_eq!(<Box<Vec<u8>>>::HEIGHT, 1);
    }

    #[test]
    fn tuple_height_is_largest_element_plus_one() {
        // Given: a tuple of a number and an array / When: its height is taken / Then: it is the array's plus one
        assert_eq!(<(u8, Vec<u8>)>::HEIGHT, 2);
    }

    #[test]
    fn scalars_strings_ranges_and_maps_have_height_0() {
        // Given: a scalar, a string, a range and a map / When: their heights are taken / Then: each is 0
        assert_eq!(i32::HEIGHT + f64::HEIGHT + bool::HEIGHT + String::HEIGHT, 0);
        assert_eq!(<RangeInclusive<i32>>::HEIGHT + <BTreeMap<String, Vec<u8>>>::HEIGHT, 0);
    }

    #[test]
    fn separator_comes_from_type_not_values() {
        // Given: arrays of two Nones whose types have heights 2 and 1 / When: written as a field value
        // Then: they are separated by ; and , by their types
        assert_eq!(to_string(&map([("a", vec![None::<Vec<u32>>, None])])), "a=;");
        assert_eq!(to_string(&map([("a", vec![None::<u32>, None])])), "a=,");
    }

    // Scalars

    #[test]
    fn integers_round_trip_and_reject_out_of_range() {
        // Given: the bounds of every integer type / When: written and read / Then: they read back
        // and a number outside the type's range is an error
        macro_rules! bounds {
            ($($T:ty)*) => {$( round_trip(<$T>::MIN); round_trip(<$T>::MAX); )*};
        }
        bounds!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
        assert!(from_str::<u8>("256").is_err());
        assert!(from_str::<u32>("-1").is_err());
    }

    #[test]
    fn floats_are_shortest_without_fraction_zero() {
        // Given: floats with and without a fraction / When: written / Then: the shortest form reads back, no .0
        assert_eq!(round_trip(1.0f64), "1");
        assert_eq!(round_trip(1.5f64), "1.5");
        assert_eq!(round_trip(0.1f64), "0.1");
        assert_eq!(round_trip(1e300f64), "1e300");
        assert_eq!(round_trip(0.1f32), "0.1");
    }

    #[test]
    fn nan_and_infinities() {
        // Given: NaN and the infinities / When: written and read / Then: they are NaN, inf and -inf and read back
        assert_eq!(to_string(&f64::NAN), "NaN");
        assert!(from_str::<f64>("NaN").unwrap().is_nan());
        assert_eq!(round_trip(f64::INFINITY), "inf");
        assert_eq!(round_trip(f32::NEG_INFINITY), "-inf");
    }

    #[test]
    fn booleans_round_trip() {
        // Given: true and false / When: written / Then: they are true and false and read back
        assert_eq!(round_trip(true), "true");
        assert_eq!(round_trip(false), "false");
    }

    #[test]
    fn strings_round_trip() {
        // Given: strings that need and need not quotes / When: written / Then: they read back
        assert_eq!(round_trip("a.rs".to_string()), "a.rs");
        assert_eq!(round_trip("say \"hi\"\n".to_string()), "\"say \\\"hi\\\"\n\"");
    }

    #[test]
    fn number_reads_into_string() {
        // Given: the text 20 / When: read as a String / Then: it is the string "20"
        assert_eq!(from_str::<String>("20"), Ok("20".into()));
    }

    // Ranges

    #[test]
    fn ranges_are_ends_joined_by_dash() {
        // Given: a range of non-negative ends / When: written / Then: it is start-end and reads back
        assert_eq!(round_trip(2u32..=10), "2-10");
    }

    #[test]
    fn negative_range_end_is_in_parentheses() {
        // Given: a range of negative ends / When: written / Then: each end is in parentheses
        assert_eq!(round_trip(-3i64..=-1), "(-3)-(-1)");
    }

    // Nil

    #[test]
    fn none_is_nothing() {
        // Given: None / When: written / Then: it is nothing and reads back
        assert_eq!(round_trip(None::<i32>), "");
    }

    #[test]
    fn some_is_its_value() {
        // Given: Some(5) / When: written / Then: it is 5
        assert_eq!(round_trip(Some(5)), "5");
    }

    #[test]
    fn empty_vec_and_map_are_nothing() {
        // Given: an empty Vec and an empty map / When: written / Then: both are nothing and read back empty
        assert_eq!(round_trip(Vec::<i32>::new()), "");
        assert_eq!(round_trip(BTreeMap::<String, i32>::new()), "");
    }

    #[test]
    fn some_empty_vec_reads_back_as_none() {
        // Given: Some(vec![]) / When: written and read / Then: it reads back as None
        assert_eq!(from_str::<Option<Vec<i32>>>(&to_string(&Some(Vec::<i32>::new()))), Ok(None));
    }

    // Arrays and tuples

    #[test]
    fn fixed_array_needs_its_length() {
        // Given: texts of two, three and one elements / When: read as [i32; 2] / Then: only two read
        assert_eq!(from_str::<[i32; 2]>("1\n2"), Ok([1, 2]));
        assert!(from_str::<[i32; 2]>("1\n2\n3").is_err());
        assert!(from_str::<[i32; 2]>("1").is_err());
    }

    #[test]
    fn tuple_needs_its_length() {
        // Given: texts of two, three and one elements / When: read as (i32, i32) / Then: only two read
        assert_eq!(round_trip((1, 2)), "1\n2");
        assert!(from_str::<(i32, i32)>("1\n2\n3").is_err());
        assert!(from_str::<(i32, i32)>("1").is_err());
    }

    // Maps

    #[test]
    fn maps_are_records() {
        // Given: a HashMap and a BTreeMap / When: written / Then: they are records and read back
        assert_eq!(round_trip(HashMap::from([("a".to_string(), 1)])), "a=1");
        assert_eq!(round_trip(map([("a", 1), ("b", 2)])), "a=1 b=2");
    }

    #[test]
    fn key_written_twice_is_error() {
        // Given: a record with the key a twice / When: read as a map / Then: it is an error
        assert!(from_str::<BTreeMap<String, i32>>("a=1 a=2").is_err());
    }

    // Round trip

    #[test]
    fn nested_values_round_trip() {
        // Given: values nesting arrays of every height, tuples, records, nil and ranges / When: written
        // Then: each reads back as the same value
        round_trip(vec![map([("x", vec![1, 2])]), map([("x", vec![])])]);
        round_trip(map([("m", vec![vec![vec![1, 2], vec![3]], vec![vec![4]]])]));
        round_trip(map([("m", vec![vec![vec![vec![1, 2]], vec![vec![3]]], vec![vec![vec![4, 5]]]])]));
        round_trip(vec![vec![vec![vec![vec![map([("a", vec![1, 2])])]]]], vec![]]);
        round_trip(map([("t", (Some("a b".to_string()), vec![1..=2, -1..=0], None::<bool>))]));
        round_trip(vec![vec![None, Some(1.5)], vec![Some(f64::INFINITY)]]);
        round_trip(map([("r", map([("s", map([("u", [0u8, 1])]))]))]));
        round_trip((vec![vec![1, 2], vec![3]], Box::new(map([("k", "----".to_string())]))));
    }
}
