# レコード

## 評価規則

レコード型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{\mathcal{S}(\ell_i) = l_i \quad e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n)}{\mathit{field}\_1\ \cdots\ \mathit{field}\_n : \mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace \Downarrow \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace}\ \textsf{(E-Rec)}
\\]

\\[
\begin{array}{rcl}
\text{where}\quad \mathit{field}\_i & = & \begin{cases} \ell_i\code{\texttt{=(}}e_i\code{\texttt{)}} & \text{if } \mathrm{composite\text{-}type}(\tau_i) \text{ and } e_i \neq \varepsilon \cr \ell_i\code{\texttt{=}}e_i & \text{otherwise} \end{cases} \cr
\mathrm{composite\text{-}type}(\sigma?) & \iff & \mathrm{composite\text{-}type}(\sigma) \cr
\mathrm{composite\text{-}type}(\sigma) & \iff & \sigma \text{ has the form } \mathsf{array}^{+}\ \tau,\ \tau_1 \ast \cdots \ast \tau_n \text{ or } \mathsf{record}\ \lbrace \dots \rbrace
\end{array}
\\]

\\(\ell_i\\) は式に書かれたキーで、\\(l_i = \mathcal{S}(\ell_i)\\) はそのキーが表すラベルです。そのため、`a=1` と `"a"=1` は同じラベルを持ちます。レコード型のフィールドの順序は問わないので、式のフィールドもどの順序で書いてもかまいません。ラベルの重複や過不足があるときは、導出なしです。
