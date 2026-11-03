# nil

nil は、値がないことを表します。何も書かないことで表します。

| JSON | MISON |
|---|---|
| `null` | （空のテキスト） |
| `{"name":null}` | `name=` |
| `{"answers":[0,null,42]}` | `answers=0,,42` |

## 何も持たない値は nil

MISON は「何もない」をひとつの書き方で表します。次の値はすべて nil です。

- 空の配列
- 空のレコード
- ただ 1 つの要素が nil である配列（ネストの深さは問わない）

```json
{"empty":[],"none":{},"void":[[[[[[[[[[]]]]]]]]]]}
```

```mison
empty= none= void=
```
