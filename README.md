[![Crates.io](https://img.shields.io/crates/v/warrah.svg)](https://crates.io/crates/warrah)
[![CI](https://github.com/evgenyneu/warrah/actions/workflows/release.yml/badge.svg)](https://github.com/evgenyneu/warrah/actions/workflows/release.yml)
[![Tests](https://github.com/evgenyneu/warrah/actions/workflows/tests.yml/badge.svg)](https://github.com/evgenyneu/warrah/actions/workflows/tests.yml)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](UNLICENSE)

# Warrah

`warrah` is a command-line utility and a Rust library that **naively** removes code comments from a text file and outputs the result to stdout:

```sh
> warrah code.py
```

Please note comments inside strings will also be removed and may result in invalid code. The main use of this code is to strip comments from code in order to feed it to an LLM, where occassionally incorrect code is fine. Thus the main focus here is on speed and simplicity, not correctness.

The program supports single and multi-line comments in the following languages:

- Ada
- Assembly
- Bash
- C
- C#
- C++
- COBOL
- CMake
- CoffeeScript
- CSS
- D
- Dart
- Dockerfile
- EJS
- Elixir
- Elm
- Erlang
- F#
- Fortran
- Gherkin (Cucumber BDD)
- Go
- Gradle
- Groovy
- Handlebars / Mustache
- Haskell
- Haxe
- HCL (HashiCorp Configuration Language)
- HTML
- INI
- Java
- JavaScript
- Jinja2
- JSONC (JSON with Comments)
- JSX / TSX (React)
- Julia
- Kotlin
- LaTeX
- Less
- Liquid
- Lisp / Clojure / Scheme
- Makefile
- Markdown
- MATLAB / Octave
- Objective-C
- Org-mode
- Perl
- PHP
- PowerShell
- Prolog
- Python
- QML (Qt Modeling Language)
- R
- Rego (Open Policy Agent)
- Ruby
- Rust
- SASS / SCSS
- Scala
- Shell
- SQL
- Starlark (Bazel build language)
- Swift
- Terraform
- TOML
- Twig
- TypeScript
- VB / VBScript
- Verilog / SystemVerilog
- Vue
- XML
- YAML
- Zig

See [docs/languages.md](docs/languages.md) for more details about the types of comments supported in each language.

## Installation

### Install using Cargo

First [install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
cargo install warrah
```

### Homebrew

Install with [Homebrew](https://brew.sh/):

```bash
brew tap evgenyneu/warrah
brew install warrah
```

### Pre-built binaries

Download pre-built binaries from the [GitHub Releases page](https://github.com/evgenyneu/warrah/releases).

1. Download the appropriate version for your platform.
2. Move the binary to a location in your PATH.


## Usage

```bash
warrah [PATH]
```

*PATH*: The path to the code file to strip comments from. The program will automatically detect the language based on the file extension or file name.

The output is sent to stdout. If you want to save the output to a file, use the `>` operator on Unix-like systems:

```bash
warrah code.py > code_no_comments.py
```

## Development

See [docs/development.md](docs/development.md) for instructions on how to set up the development environment.


## Contributing

See contributing guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).

## What's Warrah?

The warrah, or Falkland Islands wolf (Dusicyon australis), also known as the "Antarctic wolf," was the only native land mammal of the Falkland Islands. It was hunted by settlers for its fur and perceived threat to livestock, leading to its extinction in 1876.

<img src='./images/FalklandIslandFox2.jpg' alt='Picture of Falkland Island Fox'>

*Falkland Island fox or "Antarctic Wolf" by John Gerrard Keulemans, from St. George Mivart's Dogs, Jackals, Wolves, and Foxes: A Monograph of the Canidae, published by R. H. Porter, London, 1890. Lithography by Mintern Brothers. Source: [Wikimedia Commons](https://commons.wikimedia.org/wiki/File:FalklandIslandFox2.jpg).*


## Feedback is welcome

If you need help or notice a bug, feel free to create an issue ticket. We will be happy to help. :D


## The unlicense

This work is in [public domain](UNLICENSE).
