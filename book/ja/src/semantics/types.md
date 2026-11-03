# 型と値

## 型

型を、次のように帰納的に定義します。型は MISON のテキストには現れず、評価のときに式と組にして使います（[MISON を読む](../reading.md)）。

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

フィールドの順序だけが違うレコード型は、同じ型とみなします。

\\[
\mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace = \mathsf{record}\ \lbrace l_{\pi(1)} : \tau_{\pi(1)}, \dots, l_{\pi(n)} : \tau_{\pi(n)} \rbrace \quad \text{for any permutation } \pi
\\]

結合の強さは、強い順に \\(?\\)、\\(\mathsf{array}^{+}\\)・\\(\mathsf{map}^{+}\\)、\\(\ast\\) です。

## 値域

型 \\(\tau\\) の値域 \\([\negthinspace[\tau]\negthinspace]\\) を、次のように定義します。

\\[
\begin{array}{rcl}
[\negthinspace[\mathsf{bool}]\negthinspace] & = & \lbrace \mathrm{T}, \mathrm{F} \rbrace \cr
[\negthinspace[\mathsf{int}]\negthinspace] & = & \mathbb{Z} \cr
[\negthinspace[\mathsf{float}]\negthinspace] & = & \mathbb{Q} \cr
[\negthinspace[\mathsf{string}]\negthinspace] & = & \text{Unicode strings} \cr
[\negthinspace[\mathsf{range}]\negthinspace] & = & \lbrace \lbrace k \in \mathbb{N} \mid m \le k \le n \rbrace \mid m, n \in \mathbb{N} \rbrace \cr
[\negthinspace[\mathsf{array}^{+}\ \tau]\negthinspace] & = & \bigcup_{n \ge 1} [\negthinspace[\tau]\negthinspace]^{n} \cr
[\negthinspace[\tau_1 \ast \cdots \ast \tau_n]\negthinspace] & = & [\negthinspace[\tau_1]\negthinspace] \times \cdots \times [\negthinspace[\tau_n]\negthinspace] \cr
[\negthinspace[\mathsf{record}\ \lbrace l_1 : \tau_1, \dots, l_n : \tau_n \rbrace]\negthinspace] & = & \lbrace \lbrace l_1 \mapsto v_1, \dots, l_n \mapsto v_n \rbrace \mid v_i \in [\negthinspace[\tau_i]\negthinspace] \rbrace \cr
[\negthinspace[\mathsf{map}^{+}\ \tau]\negthinspace] & = & \lbrace f : [\negthinspace[\mathsf{string}]\negthinspace] \rightharpoonup_{\mathrm{fin}} [\negthinspace[\tau]\negthinspace] \mid \mathrm{dom}(f) \neq \emptyset \rbrace \cr
[\negthinspace[\sigma?]\negthinspace] & = & [\negthinspace[\sigma]\negthinspace] \cup \lbrace \mathsf{nil} \rbrace
\end{array}
\\]

