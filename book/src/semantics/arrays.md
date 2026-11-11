# Arrays and tuples

## Evaluation rules

An expression of an array or tuple type is evaluated by the following rules.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{\mathrm{inner} \vdash e_i : \tau \Downarrow v_i \quad (1 \le i \le n,\ n \ge 1,\ n = 1 \Rightarrow v_1 \neq \mathsf{nil}) \quad \rho \neq \mathrm{brk} \quad \mathrm{height}(\mathsf{array}^{+}\ \tau) \le 4}{\rho \vdash \mathit{element}\_1\ s\ \cdots\ s\ \mathit{element}\_n : \mathsf{array}^{+}\ \tau \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Arr)}
\\]

\\[
\dfrac{\mathrm{inner} \vdash e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n,\ n \ge 2) \quad \rho \neq \mathrm{brk} \quad \mathrm{height}(\tau_1 \ast \cdots \ast \tau_n) \le 4}{\rho \vdash \mathit{element}\_1\ s\ \cdots\ s\ \mathit{element}\_n : \tau_1 \ast \cdots \ast \tau_n \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-Tup)}
\\]

\\[
\begin{array}{rcl}
\text{where}\quad \mathit{element}\_i & = & \begin{cases} \code{\texttt{(}}e_i\code{\texttt{)}} & \text{if } e_i \in \mathit{Record} ,\ s \in \mathit{ArrayDelimiter} ,\ n \ge 2 \cr e_i & \text{otherwise} \end{cases} \cr
s & = & \mathrm{sep}\_{\rho}(\mathrm{height}(\sigma)) \quad \text{for the type } \sigma \text{ in the conclusion} \cr
\mathrm{sep}\_{\mathrm{top}}(h) & = & \begin{cases} \mathit{Newline} & \text{if } h \le 2 \cr \mathrm{sep}\_{\mathrm{inner}}(h) & \text{otherwise} \end{cases} \cr
\mathrm{sep}\_{\mathrm{inner}}(h) & = & \begin{cases} \code{\texttt{,}} & \text{if } h = 1 \cr \code{\texttt{;}} & \text{if } h = 2 \cr \mathit{Newline} & \text{if } h = 3 \cr \mathit{Newline}\ \code{\mathtt{\unicode{x2D}\unicode{x2D}\unicode{x2D}\unicode{x2D}}}\ \mathit{Newline} & \text{if } h = 4 \end{cases}
\end{array}
\\]

A literal of an array of five or more dimensions is evaluated by the following rules.

\\[
\dfrac{\mathrm{brk} \vdash e_i : \tau \Downarrow v_i \quad (1 \le i \le n,\ n \ge 1,\ n = 1 \Rightarrow v_1 \neq \mathsf{nil}) \quad \rho = \mathrm{brk} \lor \mathrm{height}(\mathsf{array}^{+}\ \tau) \ge 5}{\rho \vdash \code{\mathtt{\unicode{x5B}}}\mathit{item}\_1\code{\texttt{,}}\cdots\code{\texttt{,}}\mathit{item}\_n\code{\mathtt{\unicode{x5D}}} : \mathsf{array}^{+}\ \tau \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-BArr)}
\\]

\\[
\dfrac{\mathrm{brk} \vdash e_i : \tau_i \Downarrow v_i \quad (1 \le i \le n,\ n \ge 2) \quad \rho = \mathrm{brk} \lor \mathrm{height}(\tau_1 \ast \cdots \ast \tau_n) \ge 5}{\rho \vdash \code{\mathtt{\unicode{x5B}}}\mathit{item}\_1\code{\texttt{,}}\cdots\code{\texttt{,}}\mathit{item}\_n\code{\mathtt{\unicode{x5D}}} : \tau_1 \ast \cdots \ast \tau_n \Downarrow \langle v_1, \dots, v_n \rangle}\ \textsf{(E-BTup)}
\\]

\\[
\text{where}\quad \mathit{item}\_i = \begin{cases} \code{\texttt{(}}e_i\code{\texttt{)}} & \text{if } e_i \in \mathit{Record} \cr e_i & \text{otherwise} \end{cases}
\\]

\\(\mathrm{height}\\) is defined in [Types and values](types.md#height).
