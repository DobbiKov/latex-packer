# Latex packer
This CLI reads a provided main `tex` file and if there are inputet or included
files using `\input` or `\inlcude` the CLI copies their content and pastes to
the output files. In the end you obtain a single `tex` file containing all the
contents without nested files.

## Usage by example:
```main.tex
\documentclass{article}
\begin{document}
\input{abstract.tex}
Hello, world!
\input{first.tex}
\end{document}
```

```
\abstract{This is a simple abstract.}
```

```first.tex
\textbf{First nested files}

But there's another one:
\input{second.tex}
```

```second.tex
Second \textit{nested file}
```

Then if you call:
```shell
latex-packer main.tex output.tex
```

You obtain the resulted file:
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
