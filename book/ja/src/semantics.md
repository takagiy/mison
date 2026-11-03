# 意味論

以降のページでは、評価を big-step 操作的意味論として定めます。

## 判断

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
e : \tau \Downarrow v
\\]

\\(e \in \mathit{Toplevel}\\)（[構文](semantics/syntax.md)）、\\(\tau\\) は型、\\(v \in [\negthinspace[\tau]\negthinspace]\\)（[型と値](semantics/types.md)）です。

## 評価関係

\\(\Downarrow\\) は、以降のページの規則で帰納的に定義します。\\(\Downarrow\\) は部分関係で、エラー値を持ちません。規則は決定的です。

## 記法

数式の中の背景付きのタイプライター体（\\(\code{\texttt{true}}\\)、\\(\code{\texttt{(}}\\) など）は、式に書かれる文字そのものです。
