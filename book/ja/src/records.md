# レコード

レコードは、名前の付いたフィールドの集まりです。フィールドはそれぞれキーと値を持ちます。フィールドは `key=value` と書き、フィールドどうしはスペースで区切ります。

```json
{"name":"bob","email":"bob@example.com"}
```

```mison
name=bob email=bob@example.com
```

キーは、[文字列](strings.md)と同じ規則で引用符で囲みます。

```json
{"first name":"Alice","last name":"Smith"}
```

```mison
"first name"=Alice "last name"=Smith
```

## ネストしたレコード

ネストしたレコードは、曖昧さを避けるため丸括弧で囲みます。

```json
{"from":{"city":"Tokyo","airport":"HND"},"to":{"city":"Paris","airport":"CDG"}}
```

```mison
from=(city=Tokyo airport=HND) to=(city=Paris airport=CDG)
```

[配列](arrays.md)の中のレコードも、丸括弧で囲みます。

```json
{"todos":[{"task":"laundry","done":true},{"task":"groceries","done":false}]}
```

```mison
todos=(task=laundry done=true),(task=groceries done=false)
```
