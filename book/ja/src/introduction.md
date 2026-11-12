# はじめに

## 概要

MISON（minimal and simple object notation）は、JSON に似た、token-efficient で human-friendly な構造化データの記法です。初めて見る人も読みやすく、そのため言語モデルにとっても解釈しやすくなっています。

```json
[{"ok":true,"file":"a.rs","ranges":[{"start":10,"end":15},{"start":20,"end":25}]}]
```

```mison
ok=true file=a.rs ranges=10-15,20-25
```

## 用途

主な用途は、構造化データを言語モデルに渡すことです。

RPC や API のリクエストとレスポンスのように、言語モデルを介さないプログラム同士のやりとりは想定していません。

## 型とパース

MISON をパースするプログラムは、期待するデータの型をスキーマとして使います。そこで MISON は、型から補える引用符や括弧を書かず、同じデータを JSON より短く書きます。たとえば `age=20` は、型が数値なら 20、文字列なら `"20"` と読みます。
