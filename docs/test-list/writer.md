# Writing MISON text

## Strings

- [x] A string of characters that are not special is written without quotes
- [x] An empty string is written `""`
- [x] A string holding whitespace, `,`, `;`, `(`, `)`, `=` or `"` is written in quotes
- [x] Inside quotes, `"` and `\` are written with a `\` before them, and a newline is written as is
- [x] The string `----` is written in quotes, so it cannot be read as a separator
- [x] Inside a bracket literal, a string holding `[` or `]` is written in quotes; outside one, it is not

## Arrays and tuples

- [x] An array of height 1 separates its elements with `,`, and one of height 2 with `;`
- [x] An array of height 3 separates its elements with a newline, and one of height 4 with a `----` line
- [x] At the top level, an array of height 1 or 2 separates its elements with newlines
- [x] An array with one element is written as that element
- [x] An array with no elements, or whose only element is nil, is nil and written as nothing
- [x] A nil element is written as nothing
- [x] A record element of an array separated by `,` or `;` is written in parentheses
- [x] A record element of an array separated by newlines is written without parentheses
- [x] An array of height 5 or more is written as a bracket literal, with its elements separated by `,`
- [x] Inside a bracket literal, every array is a bracket literal, a one-element array included
- [x] Inside a bracket literal, an array whose only element is nil is nil and written as nothing
- [x] Inside a bracket literal, a record element is written in parentheses

## Records

- [x] A record is written as `key=value` fields separated by a space
- [x] A key that needs quotes is written in quotes
- [x] A nil field value is written as nothing after `=`
- [x] A record without fields is nil and written as nothing, without parentheses
- [x] A record that is a field value is written in parentheses
- [x] A matrix that is a field value is written in parentheses, with a newline after `(` and before `)`
- [x] An array of height 1 or 2 that is a field value is written without parentheses

## Variants

- [x] A case without contents is written as its label
- [x] A case with contents is written as its label and the contents in parentheses
- [x] Record contents are written without parentheses of their own
- [x] Matrix contents are written with a newline after `(` and before `)`
- [x] Nil contents are written as `label()`
