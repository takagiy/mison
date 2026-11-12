# レコードとマップ

## 評価規則

レコード型とマップ型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{\mathcal{S}(\ell_i) = l_i \quad \mathrm{inner} \vdash e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n)}{\rho \vdash \mathit{field}\_1\ \cdots\ \mathit{field}\_n : \mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace \Downarrow \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace}\ \textsf{(E-Rec)}
\\]

\\[
\dfrac{\mathcal{S}(\ell_i) = l_i \quad \mathrm{inner} \vdash e_i : \tau \Downarrow v_i \quad (1 \le i \le n,\ n \ge 1,\ i \neq j \Rightarrow l_i \neq l_j)}{\rho \vdash \mathit{field}\_1\ \cdots\ \mathit{field}\_n : \mathsf{map}^{+}\ \tau \Downarrow \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace}\ \textsf{(E-Map)}
\\]

\\[
\text{where}\quad \mathit{field}\_i = \begin{cases} \ell_i\code{\texttt{=}}e_i & \text{if } e_i \in \mathit{Atom} \cup \mathit{Array} \cup \mathit{Variant} \cup \mathit{BracketArray} \cr \ell_i\code{\texttt{=(}}e_i\code{\texttt{)}} & \text{if } e_i \in \mathit{Record} \cr \ell_i\code{\texttt{=(}}\ \mathit{Newline}\ e_i\ \mathit{Newline}\ \code{\texttt{)}} & \text{if } e_i \in \mathit{Matrix} \end{cases}
\\]

\\(\ell_i\\) は式に書かれたキーで、\\(l_i = \mathcal{S}(\ell_i)\\) はそのキーが表すラベルです。つまり、`a=1` と `"a"=1` は同じラベルを持ちます。レコード型のフィールドの順序は問わないので、式のフィールドもどの順序で書いてもかまいません。ラベルの重複や過不足があるときは、導出なしです。
