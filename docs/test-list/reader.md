# Reading MISON text

## Literals

- [x] An unquoted string reads up to the next special character
- [x] A quoted string reads with `\"` and `\\` unescaped and newlines kept
- [x] A `\` before any other character inside quotes is an error
- [x] Inside a bracket literal, an unquoted string also ends at `[` and `]`
- [x] An integer reads from `0` or digits with an optional `-`; `01`, `+1` and `-0` are errors
- [x] A float reads from an integer, a fraction or an exponent, and from `NaN`, `inf` and `-inf`; `.5`, `1.` and `infinity` are errors
- [x] `true` and `false` read as booleans
- [x] `a-b` reads as a range; a negative end reads only in parentheses

## Nil

- [x] Nothing before a separator, a space, a newline, `)` or the end reads as nil

## Arrays and tuples

- [x] Elements are split by the separator of the array's height
- [x] At the top level, an array of height 1 or 2 splits its elements by newlines
- [x] A text with no separator of the array's height reads as one element
- [x] Nothing reads as an array with no elements
- [x] A separator of a lower array at the start begins the first element, so the array is not nil
- [x] Nothing between `,` or before `]` in a bracket literal reads as nil
- [x] A newline followed by `----` or `)` does not separate the elements of an array of height 3
- [x] `\r\n` reads as a newline
- [x] A field value in parentheses, with a newline after `(` and before `)`, reads as a matrix
- [x] Contents with a newline after `(` and before `)` read as a matrix
- [x] An array of height 5 or more reads only from a bracket literal

## Records

- [x] Fields separated by spaces read in the order they are written
- [x] A record that is a field value reads only in parentheses
- [x] A record element of an array separated by `,` or `;` reads in parentheses
- [x] A record at the top level, in a row or as contents reads without parentheses
- [x] The `\r` of `\r\n` does not separate fields

## Variants

- [x] A label followed by parentheses reads the contents between them
- [x] Nothing between the parentheses reads as nil contents

## Top level

- [x] Text left after the value is an error
- [x] An error says at which byte reading stopped
