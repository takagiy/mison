# Semantics

The following pages define evaluation as a big-step operational semantics.

## Environment

The environment \\(\rho\\) of an evaluation says where the expression is: \\(\mathrm{top}\\) for the whole text, \\(\mathrm{inner}\\) for inside a text, and \\(\mathrm{brk}\\) for an element of a \\(\mathit{BracketArray}\\).

\\[
\rho \in \mathit{Env} = \lbrace \mathrm{top}, \mathrm{inner}, \mathrm{brk} \rbrace
\\]

## Judgment

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\rho \vdash e : \tau \Downarrow v
\\]

It says that evaluating the expression \\(e\\) ([Syntax](semantics/syntax.md)) as the type \\(\tau\\) in the environment \\(\rho\\) gives the value \\(v \in [\negthinspace[\tau]\negthinspace]\\) ([Types and values](semantics/types.md)). A text \\(t \in \mathit{Toplevel}\\) is evaluated in \\(\mathrm{top}\\).

## Evaluation relation

\\(\Downarrow\\) is defined inductively by the rules on the following pages. It is a partial relation with no error value, and the rules are deterministic.

## Notation

Typewriter type on a shaded background inside formulas (\\(\code{\texttt{true}}\\), \\(\code{\texttt{(}}\\) and so on) stands for the characters written in an expression.
