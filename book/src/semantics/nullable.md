# Nullable types

## Evaluation rules

An expression of a nullable type is evaluated by the following rules.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{}{\varepsilon : \sigma? \Downarrow \mathsf{nil}}\ \textsf{(E-Nil)}
\qquad
\dfrac{e : \sigma \Downarrow v \quad e \neq \varepsilon}{e : \sigma? \Downarrow v}\ \textsf{(E-Lift)}
\\]

- E-Nil applies only to \\(\varepsilon\\), and E-Lift only to everything else, so the two rules never overlap.
- \\(\varepsilon : \mathsf{string}? \Downarrow \mathsf{nil}\\) and \\(\code{\mathtt{\unicode{x22}\unicode{x22}}} : \mathsf{string}? \Downarrow \mathtt{\unicode{x22}\unicode{x22}}\\) are different.
