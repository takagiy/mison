# バリアント

バリアントは、いくつかの選択肢から 1 つを名前で選んだ値です。中身のある選択肢は、名前の後ろの丸括弧に中身を書きます。

```json
{"type":"rect","w":1,"h":2}
```

```mison
rect(w=1 h=2)
```

```json
{"type":"circle","radius":1}
```

```mison
circle(radius=1)
```

中身は、レコードに限らずどんな値でもかまいません。

```json
{"result":{"type":"err","value":"not found"}}
```

```mison
result=err("not found")
```

中身のない選択肢は、名前だけを書きます。

```json
{"result":{"type":"ok"}}
```

```mison
result=ok
```
