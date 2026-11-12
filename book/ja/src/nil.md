# nil

nil は、値がないことを表します。何も書かないことで表します。

```json
null
```

```mison

```

```json
{"name":null}
```

```mison
name=
```

```json
{"answers":[0,null,42]}
```

```mison
answers=0,,42
```

## 何も持たない値は nil

MISON は「何もない」をひとつの書き方で表します。次の値はすべて nil です。

- 空の配列
- 空のマップ
- ただ 1 つの要素が nil である配列

```json
{"empty":[],"none":{},"void":[[[[[[[[[[]]]]]]]]]]}
```

```mison
empty= none= void=
```
