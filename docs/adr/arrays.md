# Unwrap single-element arrays, but parenthesize array and object elements of longer arrays

## Context

Tool output is full of arrays that almost always hold one element, such as a list of edited line ranges after a single edit. Writing them as `1,2` instead of `[[1,2]]` saves characters on the common case. But unwrapping every single-element array makes some values unreadable even with their type: in a `Vec<Vec<u32>>`, `[[1,2]]` and `[[1],[2]]` would both be written `1,2`.

## Decision

- An array with one element is written as that element, at any depth.
- The elements of an array with two or more elements are written in parentheses when they are arrays or objects, and are not unwrapped: `[[1],[2]]` is `(1),(2)`.
- An object that is a field's value is written in parentheses, which also covers a single-element array of objects after unwrapping.
- An empty array or object is nil, the absent value, and is written as nothing; so is an array whose only element is nil (see [absence.md](absence.md)).

## Consequences

- Gained: the common single-element case costs no brackets, and every value still reads back into its type. Empty collections cost nothing in the common case.
- Lost: the writer has to track whether a value is an element of a longer array or sits inside `Some`, so its output depends on context, not only on the value. A reader of the text has to know the type to count the nesting back.

## Alternatives not taken

- Never unwrapping keeps the writer context-free, but spends brackets on every single-element array, which is the case MISON is written for.
- Writing every empty array or object as `()` gives one rule instead of two, but spends two characters on a common case to settle a rare one.
