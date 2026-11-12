# Rust types

## Heights

- [x] `Vec<T>` and `[T; N]` have the height of `T` plus one
- [x] `Option<T>` and `Box<T>` have the height of `T`
- [x] A tuple has the largest height of its elements plus one
- [x] Scalars, strings, ranges and maps have height 0
- [x] `Vec<Option<Vec<u32>>>` holding only `None`s is written with the separator of its type's height, not of its values

## Scalars

- [x] Every integer type is written in decimal and reads back; a number out of the type's range is an error
- [x] `f64` and `f32` are written in the shortest form that reads back as the same value, without `.0`
- [x] NaN and the infinities are written `NaN`, `inf` and `-inf` and read back
- [x] `bool` is written `true` or `false` and reads back
- [x] `String` is written as a MISON string and reads back
- [x] A number reads into `String` as the string of its digits

## Ranges

- [x] `RangeInclusive` of an integer type is written `start-end` and reads back
- [x] A negative end is written in parentheses

## Nil

- [x] `None` is written as nothing and reads back from nothing
- [x] `Some(x)` is written as `x`
- [x] An empty `Vec` and an empty map are written as nothing and read back as empty
- [x] `Some(vec![])` reads back as `None`

## Arrays and tuples

- [x] `[T; N]` reads back only from `N` elements
- [x] A tuple reads back only from as many elements as it has

## Maps

- [x] `HashMap<String, V>` and `BTreeMap<String, V>` are written as records and read back
- [x] A key written twice is an error

## Round trip

- [x] A value nesting every kind of type reads back as the same value
