# Variants

## Evaluation rules

An expression of a variant type is evaluated by the following rules.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{\ell \in \mathit{String} \quad \mathcal{S}(\ell) = l \quad l \in C}{\rho \vdash \ell : \mathsf{variant}\ C \Downarrow l}\ \textsf{(E-Label)}
\\]

\\[
\dfrac{\mathcal{S}(\ell) = l \quad (l : \tau) \in C \quad \mathrm{inner} \vdash e : \tau \Downarrow v}{\rho \vdash \ell\code{\texttt{(}}\mathit{payload}\code{\texttt{)}} : \mathsf{variant}\ C \Downarrow (l, v)}\ \textsf{(E-Variant)}
\\]

\\[
\text{where}\quad C = \lbrace c_1, \dots, c_n \rbrace, \quad \mathit{payload} = \begin{cases} \mathit{Newline}\ e\ \mathit{Newline} & \text{if } e \in \mathit{Matrix} \cr e & \text{otherwise} \end{cases}
\\]

\\(\ell\\) is the label as written in the expression, and \\(l = \mathcal{S}(\ell)\\) is the label it stands for.
