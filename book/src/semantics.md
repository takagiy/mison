# Semantics

The following pages define the meaning of MISON as a big-step operational semantics. This page describes its judgment and the form of its rules.

## Judgment

Evaluation is stated by the judgment

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
e : \tau \Downarrow v
\\]

which says that evaluating the expression \\(e \in \mathit{Toplevel}\\) ([Syntax](semantics/syntax.md)) under the type \\(\tau\\) ([Types and values](semantics/types.md)) gives the value \\(v \in [\negthinspace[\tau]\negthinspace]\\).

## Evaluation relation

The evaluation relation \\(\Downarrow\\) is the least relation closed under the inference rules on the following pages. The judgment \\(e : \tau \Downarrow v\\) holds when the rules derive it.

\\(\Downarrow\\) is a partial relation with no error value: a pair \\((e, \tau)\\) that no rule derives has no result. The rules are deterministic: for any \\((e, \tau)\\) there is at most one \\(v\\).

## Inference rules

A rule is written in this form:

\\[
\dfrac{P_1 \quad \cdots \quad P_n}{C}\ \textsf{(Name)}
\\]

When all the premises \\(P_1, \dots, P_n\\) hold, the conclusion \\(C\\) holds. A rule with no premises is an axiom. When a rule is followed by `where`, the auxiliary functions and predicates it uses are defined there.

## Notation

Typewriter type inside formulas (\\(\code{\texttt{true}}\\), \\(\code{\texttt{(}}\\) and so on) stands for the characters written in an expression.
