# Arrays, matrices and tuples

Arrays and tuples are sequences of values.

## Arrays

A one-dimensional array separates its elements with `,`:

```json
{"ingredients":["spam","ham","eggs"]}
```

```mison
ingredients=spam,ham,eggs
```

When a one- or two-dimensional array holds records, they are written in parentheses:

```json
{"todos":[{"task":"laundry","done":true},{"task":"groceries","done":false}]}
```

```mison
todos=(task=laundry done=true),(task=groceries done=false)
```

## Matrices

A two-dimensional array separates one-dimensional arrays with `;`:

```json
{"matrix":[[1,2],[3,4]]}
```

```mison
matrix=1,2;3,4
```

A three-dimensional array separates two-dimensional arrays with `\n`:

```json
{"matrix":[[[1,2],[3,4]],[[5,6],[7,8]]]}
```

```mison
matrix=(
1,2;3,4
5,6;7,8
)
```

A four-dimensional array separates three-dimensional arrays with `\n----\n`:

```json
{"matrix":[[[[1,2],[3,4]],[[5,6],[7,8]]],[[[9,10],[11,12]],[[13,14],[15,16]]]]}
```

```mison
matrix=(
1,2;3,4
5,6;7,8
----
9,10;11,12
13,14;15,16
)
```

## Arrays of five or more dimensions

An array of five or more dimensions is written like a JSON array: every level is enclosed in `[ ]`, and elements are separated by `,`.

```json
{"matrix":[[[[[1,2]]]],[[[[3,4]]]]]}
```

```mison
matrix=[[[[[1,2]]]],[[[[3,4]]]]]
```

## Arrays at the top level

At the top level, one- and two-dimensional arrays separate their elements with newlines instead of `,` and `;`:

```json
[{"name":"Ash","buddy":"Pikachu"},{"name":"Dawn","buddy":"Piplup"},{"name":"Brock","buddy":"Croagunk"}]
```

```mison
name=Ash buddy=Pikachu
name=Dawn buddy=Piplup
name=Brock buddy=Croagunk
```

```json
[[1,2],[3,4]]
```

```mison
1,2
3,4
```

## Tuples

A tuple is written like an array. It is treated as having as many dimensions as an array of its element with the most dimensions. The `pair` below is a tuple of a one-dimensional array and a number, so it is separated by `;` like a two-dimensional array:

```json
{"pair":[["a","b"],3]}
```

```mison
pair=a,b;3
```
