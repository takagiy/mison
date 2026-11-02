# Writing

## Strings

- [x] A string of plain characters is written without quotes
- [x] An empty string is written as `""`
- [x] A string holding whitespace is written in quotes
- [x] A string holding `=`, `,`, `(`, `)` or `"` is written in quotes
- [x] Inside quotes, `"` and `\` are written with a `\` before them
- [x] Inside quotes, a newline is written as a newline
- [x] A string holding `-` or `:` is written without quotes

## Numbers and booleans

- [x] An integer is written as is
- [x] A float equal to an integer is written without a fraction
- [x] A float is written in the shortest form that reads back as the same value
- [x] Not-a-number and the infinities are written `NaN`, `inf` and `-inf`
- [x] A boolean is written `true` or `false`

## Ranges

- [x] A `RangeInclusive` is written as its ends joined by `-`
- [x] A `Range` is written as its ends joined by `-`
- [x] An array of ranges is written without parentheses

## Objects

- [x] A struct is written as `key=value` fields separated by spaces
- [x] A map is written like a struct
- [x] A key holding a character that needs quotes is written in quotes
- [x] A struct that is a field's value is written in parentheses
- [x] A field holding `None` is left out

## Arrays

- [x] An array is written as its elements separated by `,`
- [x] An array with one element is written as that element
- [x] Nested single-element arrays are written as the innermost value
- [x] Array and object elements of an array with two or more elements are written in parentheses
- [x] A `None` element is written as nothing
- [x] An empty array that is a field's value is written as nothing
- [x] An empty array that is an element of a longer array is written as nothing
- [x] An array whose only element is nil is written as nothing, at any depth
- [x] `Some(x)` is written as `x`

## Enums

- [x] A variant without a value is written as its name
- [x] A variant with one value is written as `Name=value`
- [x] A variant with several values is written as `Name=` and the values as an array
- [x] A variant with fields is written as `Name=` and the fields in parentheses
- [x] A variant with a value that is a field's value is written in parentheses

## Top level

- [x] A struct at the top level is written on one line without parentheses
- [x] An array with two or more elements at the top level is written one element per line
- [x] Elements of a top-level array are written without parentheses
