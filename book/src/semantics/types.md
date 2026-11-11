# Types and values

## Types

Types are defined inductively as follows. They do not appear in a MISON text; they are paired with expressions for evaluation.

\\[
\begin{array}{rcll}
\tau & ::= & \sigma \mid \sigma? & \cr
\sigma & ::= & \mathsf{bool} \mid \mathsf{int} \mid \mathsf{float} \mid \mathsf{string} \mid \mathsf{range} & \cr
& \mid & \mathsf{array}^{+}\ \tau & \text{non-empty array} \cr
& \mid & \tau_1 \ast \cdots \ast \tau_n \quad (n \ge 2) & \text{tuple} \cr
& \mid & \mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace \quad (n \ge 1,\ i \neq j \Rightarrow l_i \neq l_j) & \text{record} \cr
& \mid & \mathsf{map}^{+}\ \tau & \text{non-empty map} \cr
\mathsf{array}\ \tau & := & (\mathsf{array}^{+}\ \tau)? & \text{array} \cr
\mathsf{map}\ \tau & := & (\mathsf{map}^{+}\ \tau)? & \text{map}
\end{array}
\\]

Record types that differ only in the order of their fields are the same type.

\\[
\mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace = \mathsf{record}\ \lbrace l_{\pi(1)} : \tau_{\pi(1)}, \dots, l_{\pi(n)} : \tau_{\pi(n)} \rbrace \quad \text{for any permutation } \pi
\\]

Operators bind from strongest to weakest: \\(?\\), \\(\mathsf{array}^{+}\\) and \\(\mathsf{map}^{+}\\), \\(\ast\\).

## Height

The height of a type \\(\tau\\), \\(\mathrm{height}(\tau)\\), is defined as follows.

\\[
\begin{array}{rcl}
\mathrm{height}(\sigma?) & = & \mathrm{height}(\sigma) \cr
\mathrm{height}(\mathsf{array}^{+}\ \tau) & = & \mathrm{height}(\tau) + 1 \cr
\mathrm{height}(\tau_1 \ast \cdots \ast \tau_n) & = & \max_i \mathrm{height}(\tau_i) + 1 \cr
\mathrm{height}(\sigma) & = & 0 \quad \text{otherwise}
\end{array}
\\]

## Values

The values \\([\negthinspace[\tau]\negthinspace]\\) of a type \\(\tau\\) are defined as follows.

\\[
\begin{array}{rcl}
[\negthinspace[\mathsf{bool}]\negthinspace] & = & \lbrace \mathrm{T}, \mathrm{F} \rbrace \cr
[\negthinspace[\mathsf{int}]\negthinspace] & = & \mathbb{Z} \cr
[\negthinspace[\mathsf{float}]\negthinspace] & = & \mathbb{Q} \cup \lbrace +\infty, -\infty, \mathrm{NaN} \rbrace \cr
[\negthinspace[\mathsf{string}]\negthinspace] & = & \text{Unicode strings} \cr
[\negthinspace[\mathsf{range}]\negthinspace] & = & \lbrace \lbrace k \in \mathbb{Z} \mid m \le k \le n \rbrace \mid m, n \in \mathbb{Z} \rbrace \cr
[\negthinspace[\mathsf{array}^{+}\ \tau]\negthinspace] & = & \lbrace \langle v_1, \dots, v_n \rangle \mid n \ge 1,\ v_i \in [\negthinspace[\tau]\negthinspace] \rbrace \setminus \lbrace \langle \mathsf{nil} \rangle \rbrace \cr
[\negthinspace[\tau_1 \ast \cdots \ast \tau_n]\negthinspace] & = & \lbrace \langle v_1, \dots, v_n \rangle \mid v_i \in [\negthinspace[\tau_i]\negthinspace] \rbrace \cr
[\negthinspace[\mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace]\negthinspace] & = & \lbrace \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace \mid v_i \in [\negthinspace[\tau_i]\negthinspace] \rbrace \cr
[\negthinspace[\mathsf{map}^{+}\ \tau]\negthinspace] & = & \lbrace f : [\negthinspace[\mathsf{string}]\negthinspace] \rightharpoonup_{\mathrm{fin}} [\negthinspace[\tau]\negthinspace] \mid \mathrm{dom}(f) \neq \emptyset \rbrace \cr
[\negthinspace[\sigma?]\negthinspace] & = & [\negthinspace[\sigma]\negthinspace] \cup \lbrace \mathsf{nil} \rbrace \cr
\text{where}\quad \langle v_1, \dots, v_n \rangle & = & \lbrace 1 \mapsto v_1, \dots, n \mapsto v_n \rbrace
\end{array}
\\]

