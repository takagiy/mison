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

## マップ

レコードが持つキーは、型で決まっています。最初の例なら `name` と `email` です。マップは、文字列のキーをいくつでも持てます。キーが前もってわからないデータは、マップで表します。環境変数の一覧がその例です。書き方は、レコードと同じです。
