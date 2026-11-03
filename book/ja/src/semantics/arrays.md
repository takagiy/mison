# 配列とタプル

## 評価規則

配列型とタプル型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{e_i : \tau_i \Downarrow v_i \quad \tau_i = \tau \quad (1 \le i \le n,\ n \ge 1)}{\mathit{element}\_1\code{\texttt{,}}\cdots\code{\texttt{,}}\mathit{element}\_n : \mathsf{array}^{+}\ \tau \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Arr)}
\\]

\\[
\dfrac{e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n,\ n \ge 2)}{\mathit{element}\_1\code{\texttt{,}}\cdots\code{\texttt{,}}\mathit{element}\_n : \tau_1 \ast \cdots \ast \tau_n \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Tup)}
\\]

\\[
\begin{array}{rcl}
\text{where}\quad \mathit{element}\_i & = & \begin{cases} \code{\texttt{(}}e_i\code{\texttt{)}} & \text{if } \mathrm{composite\text{-}type}(\tau_i) \text{ and } e_i \neq \varepsilon \cr e_i & \text{otherwise} \end{cases} \cr
\mathrm{composite\text{-}type}(\sigma?) & \iff & \mathrm{composite\text{-}type}(\sigma) \cr
\mathrm{composite\text{-}type}(\sigma) & \iff & \sigma \text{ has the form } \mathsf{array}^{+}\ \tau,\ \tau_1 \ast \cdots \ast \tau_n,\ \mathsf{record}\ \lbrace \dots \rbrace \text{ or } \mathsf{map}^{+}\ \tau
\end{array}
\\]

2 行以上のテキストは、次の規則で評価します。

\\[
\dfrac{e_i : \tau \Downarrow v_i \quad (1 \le i \le n,\ n \ge 2)}{e_1\ \mathit{Newline} \cdots \mathit{Newline}\ e_n : \mathsf{array}^{+}\ \tau \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Lines)}
\\]

\\[
\dfrac{e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n,\ n \ge 2)}{e_1\ \mathit{Newline} \cdots \mathit{Newline}\ e_n : \tau_1 \ast \cdots \ast \tau_n \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Lines-Tup)}
\\]
