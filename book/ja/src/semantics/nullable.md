# nullable 型

## 評価規則

nullable 型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{}{\rho \vdash \varepsilon : \sigma? \Downarrow \mathsf{nil}}\ \textsf{(E-Nil)}
\qquad
\dfrac{\rho \vdash e : \sigma \Downarrow v \quad e \neq \varepsilon}{\rho \vdash e : \sigma? \Downarrow v}\ \textsf{(E-Lift)}
\\]
