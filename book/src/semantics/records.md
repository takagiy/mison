# Records and maps

## Evaluation rules

An expression of a record or map type is evaluated by the following rules.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{\mathcal{S}(\ell_i) = l_i \quad e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n)}{\mathit{field}\_1\ \cdots\ \mathit{field}\_n : \mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace \Downarrow \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace}\ \textsf{(E-Rec)}
\\]

\\[
\dfrac{\mathcal{S}(\ell_i) = l_i \quad e_i : \tau_i \Downarrow v_i \quad \tau_i = \tau \quad (1 \le i \le n,\ n \ge 1,\ i \neq j \Rightarrow l_i \neq l_j)}{\mathit{field}\_1\ \cdots\ \mathit{field}\_n : \mathsf{map}^{+}\ \tau \Downarrow \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace}\ \textsf{(E-Map)}
\\]

\\[
\begin{array}{rcl}
\text{where}\quad \mathit{field}\_i & = & \begin{cases} \ell_i\code{\texttt{=(}}e_i\code{\texttt{)}} & \text{if } \mathrm{composite\text{-}type}(\tau_i) \text{ and } e_i \neq \varepsilon \cr \ell_i\code{\texttt{=}}e_i & \text{otherwise} \end{cases} \cr
\mathrm{composite\text{-}type}(\sigma?) & \iff & \mathrm{composite\text{-}type}(\sigma) \cr
\mathrm{composite\text{-}type}(\sigma) & \iff & \sigma \text{ has the form } \mathsf{array}^{+}\ \tau,\ \tau_1 \ast \cdots \ast \tau_n,\ \mathsf{record}\ \lbrace \dots \rbrace \text{ or } \mathsf{map}^{+}\ \tau
\end{array}
\\]

\\(\ell_i\\) is the key as written in the expression, and \\(l_i = \mathcal{S}(\ell_i)\\) is the label it stands for. So `a=1` and `"a"=1` have the same label. Since the order of the fields in a record type does not matter, the fields of an expression may be written in any order. A duplicate, missing or extra label has no derivation.
