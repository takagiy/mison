# 意味論

以降のページでは、評価を big-step 操作的意味論として定めます。

## 環境

評価の環境 \\(\rho\\) は、式がテキスト全体（\\(\mathrm{top}\\)）か、テキストの中の式（\\(\mathrm{inner}\\)）か、\\(\mathit{BracketArray}\\) の中の式（\\(\mathrm{brk}\\)）かを表します。

\\[
\rho \in \mathit{Env} = \lbrace \mathrm{top}, \mathrm{inner}, \mathrm{brk} \rbrace
\\]

## 判断

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\rho \vdash e : \tau \Downarrow v
\\]

環境 \\(\rho\\) で、式 \\(e\\)（[構文](semantics/syntax.md)）を型 \\(\tau\\) として評価すると、値 \\(v \in [\negthinspace[\tau]\negthinspace]\\)（[型と値](semantics/types.md)）になる、という判断です。テキスト \\(t \in \mathit{Toplevel}\\) は \\(\mathrm{top}\\) で評価します。

## 評価関係

\\(\Downarrow\\) は、以降のページの規則で帰納的に定義します。\\(\Downarrow\\) は部分関係で、エラー値を持ちません。規則は決定的です。

## 記法

数式の中の背景付きのタイプライター体（\\(\code{\texttt{true}}\\)、\\(\code{\texttt{(}}\\) など）は、式に書かれる文字そのものです。
