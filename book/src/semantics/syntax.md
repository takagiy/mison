# Syntax

## Expressions

A MISON text is a \\(\mathit{Toplevel}\\), written by this grammar.

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
\mathit{BracketArrayItem} & ::= & \mathit{Atom}\_{\mathrm{brk}} \mid \code{\texttt{(}}\ \mathit{Record}\_{\mathrm{brk}}\ \code{\texttt{)}} \mid \mathit{BracketArray} \cr
\text{where}\quad \mathit{Space} & = & \lbrace c \mid c \text{ has the Unicode White_Space property},\ c \neq \mathtt{LF} \rbrace \cr
\mathit{Atom}\_{\mathrm{brk}} & = & \mathit{Atom} \text{ derived with } \mathit{String}\_{\mathrm{brk}} \text{ in place of } \mathit{String} \cr
\mathit{Record}\_{\mathrm{brk}} & = & \mathit{Record} \text{ derived with } \mathit{String}\_{\mathrm{brk}} \text{ in place of } \mathit{String}
\end{array}
\\]

\\(\varepsilon\\) stands for an expression with nothing written in it.

## Literals

\\[
\begin{array}{rcl}
\mathit{Literal} & ::= & \mathit{Bool} \mid \mathit{Int} \mid \mathit{Float} \mid \mathit{Range} \mid \mathit{String} \cr
\mathit{Bool} & ::= & \code{\texttt{true}} \mid \code{\texttt{false}} \cr
\mathit{Int} & ::= & \code{\texttt{0}} \mid \code{\texttt{-}}^{?}\ [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{Float} & ::= & \code{\texttt{-}}^{?}\ (\code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast})\ (\code{\texttt{.}}\ [\code{\texttt{0-9}}]^{+})^{?}\ ([\code{\texttt{eE}}]\ [\code{\texttt{+-}}]^{?}\ [\code{\texttt{0-9}}]^{+})^{?} \cr
& \mid & \code{\texttt{NaN}} \mid \code{\texttt{inf}} \mid \code{\texttt{-inf}} \cr
\mathit{Range} & ::= & \mathit{RangeBound}\ \code{\texttt{-}}\ \mathit{RangeBound} \cr
\mathit{RangeBound} & ::= & \mathit{Int}\_{\geq 0} \mid \code{\texttt{(}}\mathit{Int}\_{<0}\code{\texttt{)}} \cr
\mathit{Int}\_{\geq 0} & ::= & \code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{Int}\_{<0} & ::= & \code{\texttt{-}}\ [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{String} & ::= & \code{\mathtt{\unicode{x22}}}\ \mathit{Char}\_{\mathrm{quoted}}^{\ast}\ \code{\mathtt{\unicode{x22}}} \mid \mathit{Char}\_{\mathrm{unquoted}}^{+} \cr
\mathit{String}\_{\mathrm{brk}} & ::= & \code{\mathtt{\unicode{x22}}}\ \mathit{Char}\_{\mathrm{quoted}}^{\ast}\ \code{\mathtt{\unicode{x22}}} \mid \mathit{Char}\_{\mathrm{unquoted},\mathrm{brk}}^{+} \cr
\mathit{Char}\_{\mathrm{unquoted}} & ::= & \text{any character} \setminus \mathit{Special} \cr
\mathit{Char}\_{\mathrm{unquoted},\mathrm{brk}} & ::= & \text{any character} \setminus (\mathit{Special} \cup \lbrace \code{\mathtt{\unicode{x5B}}},\ \code{\mathtt{\unicode{x5D}}} \rbrace) \cr
\mathit{Char}\_{\mathrm{quoted}} & ::= & (\text{any character} \setminus \lbrace \code{\mathtt{\unicode{x22}}},\ \code{\mathtt{\unicode{x5C}}} \rbrace) \mid \code{\mathtt{\unicode{x5C}\unicode{x22}}} \mid \code{\mathtt{\unicode{x5C}\unicode{x5C}}} \cr
\text{where}\quad \mathit{Special} & = & \mathit{Whitespace} \cup \lbrace \code{\texttt{,}},\ \code{\texttt{;}},\ \code{\texttt{(}},\ \code{\texttt{)}},\ \code{\texttt{=}},\ \code{\mathtt{\unicode{x22}}} \rbrace \cr
\mathit{Whitespace} & = & \lbrace c \mid c \text{ has the Unicode White_Space property} \rbrace
\end{array}
\\]

A literal can match more than one alternative of \\(\mathit{Literal}\\). For example, `true` matches both \\(\mathit{Bool}\\) and \\(\mathit{String}\\). The type decides which one it is evaluated as.

## Valid texts

A text is valid when it evaluates to a result under some type.

\\[
\mathit{Valid} = \lbrace t \in \mathit{Toplevel} \mid \exists \tau.\ \exists v \in [\negthinspace[\tau]\negthinspace].\ \mathrm{top} \vdash t : \tau \Downarrow v \rbrace
\\]

\\(\tau\\) ranges over the [types](types.md), and \\(\Downarrow\\) is the evaluation relation defined on the following pages. For example, `a=1 a=2` is in \\(\mathit{Toplevel}\\), but it is not valid.
