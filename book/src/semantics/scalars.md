# Scalar types

## Evaluation rules

An expression of a scalar type is evaluated by the following rules.

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{}{\code{\texttt{true}} : \mathsf{bool} \Downarrow \mathrm{T}}\ \textsf{(E-True)}
\qquad
\dfrac{}{\code{\texttt{false}} : \mathsf{bool} \Downarrow \mathrm{F}}\ \textsf{(E-False)}
\\]

\\[
\dfrac{d \in \mathit{Int}}{d : \mathsf{int} \Downarrow \mathcal{N}(d)}\ \textsf{(E-Int)}
\qquad
\dfrac{f \in \mathit{Float}}{f : \mathsf{float} \Downarrow \mathcal{D}(f)}\ \textsf{(E-Float)}
\\]

\\[
\dfrac{s \in \mathit{String}}{s : \mathsf{string} \Downarrow \mathcal{S}(s)}\ \textsf{(E-Str)}
\qquad
\dfrac{a, b \in \mathit{Int}\_{\geq 0}}{a\code{\texttt{-}}b : \mathsf{range} \Downarrow \lbrace k \in \mathbb{N} \mid \mathcal{N}(a) \le k \le \mathcal{N}(b) \rbrace}\ \textsf{(E-Range)}
\\]

\\[
\begin{array}{rcll}
\text{where}\quad \mathcal{N} & : & \mathit{Int} \to \mathbb{Z} & d \mapsto \text{the integer that } d \text{ denotes} \cr
\mathcal{D} & : & \mathit{Float} \to \mathbb{Q} & f \mapsto \text{the rational number that } f \text{ denotes} \cr
\mathcal{S} & : & \mathit{String} \to \text{Unicode strings} & s \mapsto \text{the string that } s \text{ denotes}
\end{array}
\\]

\\(\mathit{Int}\\), \\(\mathit{Int}\_{\geq 0}\\), \\(\mathit{Float}\\) and \\(\mathit{String}\\) are the literals defined in [Syntax](syntax.md#literals).
