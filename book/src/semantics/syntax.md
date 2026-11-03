# Syntax

## Expressions

A MISON text is a \\(\mathit{Toplevel}\\), written by this grammar. A text is a sequence of expressions \\(\mathit{Expr}\\) separated by newlines.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\begin{array}{rcl}
\mathit{Toplevel} & ::= & \mathit{Expr}\ (\mathit{Newline}\ \mathit{Expr})^{\ast} \cr
\mathit{Newline} & ::= & \mathtt{LF} \mid \mathtt{CR}\ \mathtt{LF} \cr
\mathit{Expr} & ::= & \varepsilon \mid \mathit{Atom} \mid \mathit{List} \mid \mathit{Record} \cr
\mathit{List} & ::= & \mathit{Elem}\ (\code{\texttt{,}}\ \mathit{Elem})^{+} \cr
\mathit{Record} & ::= & \mathit{Field}\ (\mathit{Space}^{+}\ \mathit{Field})^{\ast} \cr
\mathit{Field} & ::= & \mathit{String}\ \code{\texttt{=}}\ \mathit{Elem} \cr
\mathit{Elem} & ::= & \varepsilon \mid \mathit{Atom} \mid \code{\texttt{(}}\ \mathit{Expr}\ \code{\texttt{)}} \cr
\text{where}\quad \mathit{Space} & = & \lbrace c \mid c \text{ has the Unicode White_Space property},\ c \neq \mathtt{LF} \rbrace
\end{array}
\\]

\\(\varepsilon\\) is the empty expression: nothing written at all.

## Literals

\\[
\begin{array}{rcl}
\mathit{Atom} & ::= & \mathit{Bool} \mid \mathit{Int} \mid \mathit{Float} \mid \mathit{Range} \mid \mathit{String} \cr
\mathit{Bool} & ::= & \code{\texttt{true}} \mid \code{\texttt{false}} \cr
\mathit{Int} & ::= & \code{\texttt{0}} \mid \code{\texttt{-}}^{?}\ [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{Float} & ::= & \code{\texttt{-}}^{?}\ (\code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast})\ (\code{\texttt{.}}\ [\code{\texttt{0-9}}]^{+})^{?}\ ([\code{\texttt{eE}}]\ [\code{\texttt{+-}}]^{?}\ [\code{\texttt{0-9}}]^{+})^{?} \cr
\mathit{Range} & ::= & \mathit{Int}\_{\geq 0}\ \code{\texttt{-}}\ \mathit{Int}\_{\geq 0} \cr
\mathit{Int}\_{\geq 0} & ::= & \code{\texttt{0}} \mid [\code{\texttt{1-9}}]\ [\code{\texttt{0-9}}]^{\ast} \cr
\mathit{String} & ::= & \code{\mathtt{\unicode{x22}}}\ \mathit{Char}\_{\mathrm{quoted}}^{\ast}\ \code{\mathtt{\unicode{x22}}} \mid \mathit{Char}\_{\mathrm{unquoted}}^{+} \cr
\mathit{Char}\_{\mathrm{unquoted}} & ::= & \text{any character} \setminus \mathit{Special} \cr
\mathit{Char}\_{\mathrm{quoted}} & ::= & (\text{any character} \setminus \lbrace \code{\mathtt{\unicode{x22}}},\ \code{\mathtt{\unicode{x5C}}} \rbrace) \mid \code{\mathtt{\unicode{x5C}\unicode{x22}}} \mid \code{\mathtt{\unicode{x5C}\unicode{x5C}}} \cr
\text{where}\quad \mathit{Special} & = & \mathit{Whitespace} \cup \lbrace \code{\texttt{,}},\ \code{\texttt{(}},\ \code{\texttt{)}},\ \code{\texttt{=}},\ \code{\mathtt{\unicode{x22}}} \rbrace \cr
\mathit{Whitespace} & = & \lbrace c \mid c \text{ has the Unicode White_Space property} \rbrace
\end{array}
\\]

A literal can match more than one alternative of \\(\mathit{Atom}\\). For example, `true` matches both \\(\mathit{Bool}\\) and \\(\mathit{String}\\). The type decides which one it is evaluated as.

## Valid texts

A text is valid when it evaluates to a result under some type.

\\[
\mathit{Valid} = \lbrace e \in \mathit{Toplevel} \mid \exists \tau.\ \exists v \in [\negthinspace[\tau]\negthinspace].\ e : \tau \Downarrow v \rbrace
\\]

\\(\tau\\) ranges over the [types](types.md), and \\(\Downarrow\\) is the evaluation relation defined on the following pages. For example, `a=()` and `a=1 a=2` are in \\(\mathit{Toplevel}\\), but they are not valid.
