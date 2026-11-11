# スカラー型

## 評価規則

スカラー型の式は、次の規則で評価します。

\\[
\newcommand{\code}[1]{{\kern0.1em \style{background-color: rgb(128 128 128 / 0.1)}{\kern0.1em #1\mathstrut\kern0.1em}\kern0.1em}}
\dfrac{}{\rho \vdash \code{\texttt{true}} : \mathsf{bool} \Downarrow \mathrm{T}}\ \textsf{(E-True)}
\qquad
\dfrac{}{\rho \vdash \code{\texttt{false}} : \mathsf{bool} \Downarrow \mathrm{F}}\ \textsf{(E-False)}
\\]

\\[
\dfrac{d \in \mathit{Int}}{\rho \vdash d : \mathsf{int} \Downarrow \mathcal{N}(d)}\ \textsf{(E-Int)}
\qquad
\dfrac{f \in \mathit{Float}}{\rho \vdash f : \mathsf{float} \Downarrow \mathcal{D}(f)}\ \textsf{(E-Float)}
\\]

\\[
\dfrac{s \in \mathit{String}}{\rho \vdash s : \mathsf{string} \Downarrow \mathcal{S}(s)}\ \textsf{(E-Str)}
\qquad
\dfrac{a, b \in \mathit{RangeBound}}{\rho \vdash a\code{\texttt{-}}b : \mathsf{range} \Downarrow \lbrace k \in \mathbb{Z} \mid \mathcal{B}(a) \le k \le \mathcal{B}(b) \rbrace}\ \textsf{(E-Range)}
\\]

\\[
\begin{array}{rcll}
\text{where}\quad \mathcal{N} & : & \mathit{Int} \to \mathbb{Z} & d \mapsto \text{the integer that } d \text{ denotes} \cr
\mathcal{D} & : & \mathit{Float} \to [\negthinspace[\mathsf{float}]\negthinspace] & f \mapsto \text{the value that } f \text{ denotes} \cr
\mathcal{S} & : & \mathit{String} \to \text{Unicode strings} & s \mapsto \text{the string that } s \text{ denotes} \cr
\mathcal{B} & : & \mathit{RangeBound} \to \mathbb{Z} & d \mapsto \mathcal{N}(d),\ \code{\texttt{(}}d\code{\texttt{)}} \mapsto \mathcal{N}(d)
\end{array}
\\]

\\(\mathit{Int}\\)・\\(\mathit{RangeBound}\\)・\\(\mathit{Float}\\)・\\(\mathit{String}\\) は、[構文](syntax.md#リテラル)で定義したリテラルです。
