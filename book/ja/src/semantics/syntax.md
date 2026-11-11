# 構文

## 式

MISON のテキストは、次の文法で書く \\(\mathit{Toplevel}\\) です。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\begin{array}{rcl}
\mathit{Toplevel} & ::= & \mathit{Row} \mid \mathit{Matrix} \mid \mathit{BracketArray} \cr
\mathit{Row} & ::= & \mathit{Array} \mid \mathit{Record} \mid \mathit{Atom} \cr
\mathit{Array} & ::= & \mathit{ArrayItem}\ (\mathit{ArrayDelimiter}\ \mathit{ArrayItem})^{+} \cr
\mathit{ArrayItem} & ::= & \mathit{Atom} \mid \code{\texttt{(}}\ \mathit{Record}\ \code{\texttt{)}} \cr
\mathit{ArrayDelimiter} & ::= & \code{\texttt{,}} \mid \code{\texttt{;}} \cr
\mathit{Record} & ::= & \mathit{String}\ \code{\texttt{=}}\ \mathit{RecordItem}\ (\mathit{Space}^{+}\ \mathit{String}\ \code{\texttt{=}}\ \mathit{RecordItem})^{\ast} \cr
\mathit{RecordItem} & ::= & \mathit{Atom} \mid \mathit{Array} \mid \code{\texttt{(}}\ \mathit{Record}\ \code{\texttt{)}} \mid \code{\texttt{(}}\ \mathit{Newline}\ \mathit{Matrix}\ \mathit{Newline}\ \code{\texttt{)}} \cr
& \mid & \mathit{BracketArray} \cr
\mathit{Atom} & ::= & \varepsilon \mid \mathit{Literal} \cr
\mathit{Matrix} & ::= & \mathit{Row}\ (\mathit{MatrixDelimiter}\ \mathit{Row})^{+} \cr
\mathit{MatrixDelimiter} & ::= & \mathit{Newline} \mid \mathit{Newline}\ \code{\mathtt{\unicode{x2D}\unicode{x2D}\unicode{x2D}\unicode{x2D}}}\ \mathit{Newline} \cr
\mathit{Newline} & ::= & \mathtt{LF} \mid \mathtt{CR}\ \mathtt{LF} \cr
\mathit{BracketArray} & ::= & \code{\mathtt{\unicode{x5B}}}\ \mathit{BracketArrayItem}\ (\code{\texttt{,}}\ \mathit{BracketArrayItem})^{\ast}\ \code{\mathtt{\unicode{x5D}}} \cr
\mathit{BracketArrayItem} & ::= & \mathit{Atom} \mid \code{\texttt{(}}\ \mathit{Record}\ \code{\texttt{)}} \mid \mathit{BracketArray} \cr
\text{where}\quad \mathit{Space} & = & \lbrace c \mid c \text{ has the Unicode White_Space property},\ c \neq \mathtt{LF} \rbrace
\end{array}
\\]

\\(\varepsilon\\) は、何も書かれていない式を表します。

## リテラル

\\[
\begin{array}{rcl}
\mathit{Literal} & ::= & \mathit{Bool} \mid \mathit{Int} \mid \mathit{Float} \mid \mathit{Range} \mid \mathit{String} \cr
\mathit{Bool} & ::= & \code{\texttt{true}} \mid \code{\texttt{false}} \cr
\mathit{Int} & ::= & \code{\texttt{0}} \mid \code{\texttt{-}}^{?}\ [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{Float} & ::= & \code{\texttt{-}}^{?}\ (\code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast})\ (\code{\texttt{.}}\ [\code{\texttt{0-9}}]^{+})^{?}\ ([\code{\texttt{eE}}]\ [\code{\texttt{+-}}]^{?}\ [\code{\texttt{0-9}}]^{+})^{?} \cr
\mathit{Range} & ::= & \mathit{Int}\_{\geq 0}\ \code{\texttt{-}}\ \mathit{Int}\_{\geq 0} \cr
\mathit{Int}\_{\geq 0} & ::= & \code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{String} & ::= & \code{\mathtt{\unicode{x22}}}\ \mathit{Char}\_{\mathrm{quoted}}^{\ast}\ \code{\mathtt{\unicode{x22}}} \mid \mathit{Char}\_{\mathrm{unquoted}}^{+} \cr
\mathit{Char}\_{\mathrm{unquoted}} & ::= & \text{any character} \setminus \mathit{Special} \cr
\mathit{Char}\_{\mathrm{quoted}} & ::= & (\text{any character} \setminus \lbrace \code{\mathtt{\unicode{x22}}},\ \code{\mathtt{\unicode{x5C}}} \rbrace) \mid \code{\mathtt{\unicode{x5C}\unicode{x22}}} \mid \code{\mathtt{\unicode{x5C}\unicode{x5C}}} \cr
\text{where}\quad \mathit{Special} & = & \mathit{Whitespace} \cup \lbrace \code{\texttt{,}},\ \code{\texttt{;}},\ \code{\texttt{(}},\ \code{\texttt{)}},\ \code{\texttt{=}},\ \code{\mathtt{\unicode{x22}}} \rbrace \cup \mathit{Brackets} \cr
\mathit{Whitespace} & = & \lbrace c \mid c \text{ has the Unicode White_Space property} \rbrace \cr
\mathit{Brackets} & = & \begin{cases} \lbrace \code{\mathtt{\unicode{x5B}}},\ \code{\mathtt{\unicode{x5D}}} \rbrace & \text{within a } \mathit{BracketArray} \cr \emptyset & \text{otherwise} \end{cases}
\end{array}
\\]

1 つのリテラルが、\\(\mathit{Literal}\\) の複数の候補に当てはまることがあります。たとえば `true` は \\(\mathit{Bool}\\) にも \\(\mathit{String}\\) にも当てはまります。どれとして評価するかは、型が決めます。

## 有効なテキスト

ある型と組み合わせて評価結果が定まるテキストを、有効なテキストとします。

\\[
\mathit{Valid} = \lbrace t \in \mathit{Toplevel} \mid \exists \tau.\ \exists v \in [\negthinspace[\tau]\negthinspace].\ \mathrm{top} \vdash t : \tau \Downarrow v \rbrace
\\]

\\(\tau\\) は[型](types.md)、\\(\Downarrow\\) は以降のページで定める評価関係です。たとえば `a=1 a=2` は \\(\mathit{Toplevel}\\) ですが、有効なテキストではありません。
