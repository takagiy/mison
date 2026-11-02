# Reading

## The type as the schema

- [x] Every value written by `to_string` reads back as the same value through its type
- [x] Text reads as a number or a string by the type of the field it is read into
- [x] Reading into a type that asks the text what it holds (`deserialize_any`) is an error

## Strings

- [x] An unquoted string reads up to the next separator
- [x] A quoted string reads with `\"` and `\\` unescaped and newlines kept

## Numbers

- [x] An integer reads into a float type
- [x] `NaN`, `inf` and `-inf` read into a float type

## Ranges

- [x] `a-b` reads into a `RangeInclusive` and a `Range` with those ends
- [x] A range whose ends are negative reads with its signs

## Objects

- [x] A struct field left out reads as `None` when the field is an `Option`
- [x] A field written in parentheses reads into a struct field

## Arrays

- [x] A value written without brackets reads into as many arrays as the type nests
- [x] `1,2` reads into a `Vec<Vec<u32>>` as one inner array
- [x] Nothing reads as `None` when the type is an `Option`
- [x] Nothing reads as an empty array or object when the type is an array or object
- [x] An array whose only element is nil reads back as an empty array
- [x] `Some(vec![])` and `Some(None)` read back as `None`

## Top level

- [x] Each line at the top level reads as one element of a top-level array
- [x] A newline inside quotes does not start a new top-level element
