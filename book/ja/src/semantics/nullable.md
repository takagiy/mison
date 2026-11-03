# nullable 型

## 評価規則

nullable 型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{}{\varepsilon : \sigma? \Downarrow \mathsf{nil}}\ \textsf{(E-Nil)}
\qquad
\dfrac{e : \sigma \Downarrow v \quad e \neq \varepsilon}{e : \sigma? \Downarrow v}\ \textsf{(E-Lift)}
\\]

- E-Nil は \\(\varepsilon\\) にだけ、E-Lift は \\(\varepsilon\\) 以外にだけ当てはまるので、2 つの規則は重なりません。
- \\(\varepsilon : \mathsf{string}? \Downarrow \mathsf{nil}\\) と \\(\code{\mathtt{\unicode{x22}\unicode{x22}}} : \mathsf{string}? \Downarrow \mathtt{\unicode{x22}\unicode{x22}}\\) は区別されます。
