# Semantics

The following pages define evaluation as a big-step operational semantics.

## Judgment

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
e : \tau \Downarrow v
\\]

Here \\(e \in \mathit{Toplevel}\\) ([Syntax](semantics/syntax.md)), \\(\tau\\) is a type, and \\(v \in [\negthinspace[\tau]\negthinspace]\\) ([Types and values](semantics/types.md)).

## Evaluation relation

\\(\Downarrow\\) is defined inductively by the rules on the following pages. It is a partial relation with no error value, and the rules are deterministic.

## Notation

Typewriter type on a shaded background inside formulas (\\(\code{\texttt{true}}\\), \\(\code{\texttt{(}}\\) and so on) stands for the characters written in an expression.
