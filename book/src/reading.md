# Reading MISON

A MISON text does not say what type each value has. The program reading it brings the shape of the data it expects, a schema, and reads each value by that shape. These texts mean different data depending on what the reader expects:

| Text | Read as |
|---|---|
| `age=20` | the number 20, or the string `"20"` |
| `span=1-10` | a range from 1 to 10, or the string `"1-10"` |
| `a=1,2` | the array `[1,2]`, or the array `[[1,2]]` with one element |
| `name=` | nil: an absent value, an empty array or an empty record, whichever the reader expects |

The shape is what lets MISON leave these things out. In turn, MISON cannot be read without one: reading a text into a value whose shape is decided by the text itself, such as a generic JSON value, is not possible. Writing any data works.

Reading also gives back data, not the exact value that was written. Values that are the same data, such as an empty array and nil ([Nil](nil.md)), read back as one of them.
