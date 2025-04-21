# Latex packer
This CLI reads a provided main `tex` file and if there are inputet or included
files using `\input` or `\inlcude` the CLI copies their content and pastes to
the output files. In the end you obtain a single `tex` file containing all the
contents without nested files.

## Installation
### Install with Cargo
Install this CLI using `Cargo`:
```
cargo install latex-packer
```

### Build it yourself
1. `git clone https://github.com/DobbiKov/latex-packer`
2. `cd latex-packer`
3. `cargo build -r`
4. Use the built exectuable that is placed in `target/release/`


## Usage by example:
Suppose you have the next latex project structure: \
`main.tex`
```main.tex
\documentclass{article}
\begin{document}
\input{abstract.tex}
Hello, world!
\input{first.tex}
\end{document}
```

`abstract.tex`
```
\abstract{This is a simple abstract.}
```

`first.tex`
```first.tex
\textbf{First nested files}

But there's another one:
\input{second.tex}
```

`second.tex`
```second.tex
Second \textit{nested file}
```

Then if you call:
```shell
latex-packer main.tex output.tex
```

You obtain the resulted file: \
`output.tex`
```output.tex

\documentclass{article}
\begin{document}
\abstract{This is a simple abstract.}
Hello, world!
\textbf{First nested files}

But there's another one:
Second \textit{nested file}
\end{document}
```
