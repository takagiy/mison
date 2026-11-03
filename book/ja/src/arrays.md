# 配列

配列（またはタプル）は、値の並びです。要素は `,` で区切ります。

```json
{"ingredients":["spam","ham","eggs"]}
```

```mison
ingredients=spam,ham,eggs
```

## トップレベルの配列

トップレベルの配列は、要素を `,` ではなく改行で区切ります。

```json
[{"name":"Ash","buddy":"Pikachu"},{"name":"Dawn","buddy":"Piplup"},{"name":"Brock","buddy":"Croagunk"}]
```

```mison
name=Ash buddy=Pikachu
name=Dawn buddy=Piplup
name=Brock buddy=Croagunk
```

## ネストした配列

ネストした配列は、丸括弧で囲みます。

```json
{"matrix":[[1,2,3],[4,5,6]]}
```

```mison
matrix=(1,2,3),(4,5,6)
```

## 要素が 1 つの配列

要素が 1 つだけの配列は、どれだけ深くネストしていても、その要素として書きます。値を配列でいくつ包み直すかは、読み手が知っています。

| JSON | MISON |
|---|---|
| `{"teams":[["Ann","Ben"]]}` | `teams=Ann,Ben` |
| `{"teams":[[["Ann"]]]}` | `teams=Ann` |
| `{"teams":[["Ann"],["Ben"]]}` | `teams=(Ann),(Ben)` |

最後の行は、要素が 2 つ以上の配列で要素の丸括弧を残す理由を示しています。丸括弧がなければ、`[["Ann"],["Ben"]]` と `[["Ann","Ben"]]` はどちらも `Ann,Ben` と書かれ、読み手の期待する形でも区別できなくなります。
