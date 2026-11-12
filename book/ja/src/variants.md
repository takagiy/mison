# バリアント

バリアントは、何種類かの形をとりうる値です。バリアントは、種類を表すタグと、中身のデータの組み合わせとして書きます。

```json
{"type":"rect","width":1,"height":2}
```

```mison
rect(width=1 height=2)
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

中身のない種類は、タグだけを書きます。

```json
{"result":{"type":"ok"}}
```

```mison
result=ok
```
