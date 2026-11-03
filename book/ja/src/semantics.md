# 意味論

以降のページでは、MISON の意味を big-step 操作的意味論で定めます。このページでは、その判断と規則の形を説明します。

## 判断

評価は、次の判断で表します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
e : \tau \Downarrow v
\\]

式 \\(e \in \mathit{Toplevel}\\)（[構文](semantics/syntax.md)）を型 \\(\tau\\)（[型と値](semantics/types.md)）のもとで評価すると、値 \\(v \in [\negthinspace[\tau]\negthinspace]\\) になる、という判断です。

## 評価関係

評価関係 \\(\Downarrow\\) は、以降のページの推論規則で帰納的に定義される最小の関係です。判断 \\(e : \tau \Downarrow v\\) は、規則による導出があるときに成り立ちます。

\\(\Downarrow\\) は部分関係で、エラー値を持ちません。どの規則でも導出できない組 \\((e, \tau)\\) には、評価結果がありません。また、規則は決定的で、どの \\((e, \tau)\\) についても \\(v\\) は高々 1 つです。

## 推論規則

規則は、次の形で書きます。

\\[
\dfrac{P_1 \quad \cdots \quad P_n}{C}\ \textsf{(Name)}
\\]

前提 \\(P_1, \dots, P_n\\) がすべて成り立つとき、結論 \\(C\\) が成り立ちます。前提のない規則は公理です。規則の後に where が続くときは、規則で使う補助関数や述語をそこで定義します。

## 記法

数式の中のタイプライター体（\\(\code{\texttt{true}}\\)、\\(\code{\texttt{(}}\\) など）は、式に書かれる文字そのものです。
