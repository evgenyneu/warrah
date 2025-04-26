# Supported languages

Here is the list of supported languages in which Warrah will remove comments. Languages are detected based on file extensions or file names.

- **Ada**
  - Extensions: `.adb`, `.ads`
  - Comments: `--`

- **Assembly**
  - Extensions: `.asm`, `.s`
  - Comments: `;`, `#`

- **C#**
  - Extension: `.cs`
  - Comments: `//`, `/* ... */`

- **C / C++**
  - Extensions: `.c`, `.cpp`, `.h`, `.hpp`
  - Comments: `//`, `/* ... */`

- **COBOL**
  - Extensions: `.cob`, `.cbl`
  - Comments: `*>`
  - ⚠️ Only modern comment style supported; fixed-format (column-based) is not supported

- **CMake**
  - Extension: `.cmake`
  - File: `CMakeLists.txt`
  - Comments: `#`

- **CoffeeScript**
  - Extension: `.coffee`
  - Comments: `#`, `### ... ###`

- **CSS**
  - Extensions: `.css`
  - Comments: `/* ... */`

- **Config files / scripting (misc)**
  - Extensions: `.env`, `.conf`, `.gitignore`, `.cfg`
  - Comments: `#`

- **D**
  - Extension: `.d`
  - Comments: `//`, `/* ... */`

- **Dart**
  - Extension: `.dart`
  - Comments: `//`, `/* ... */`

- **Dockerfile**
  - File: `Dockerfile`
  - Comments: `#`

- **EJS (Embedded JavaScript)**
  - Extension: `.ejs`
  - Comments: `<%-- ... --%>`

- **Elixir / Erlang**
  - Extensions: `.ex`, `.exs`, `.erl`
  - Comments: `#`

- **Elm**
  - Extension: `.elm`
  - Comments: `--`

- **F#**
  - Extension: `.fs`, `.fsx`
  - Comments: `//`
  - ⚠️ Multiline comments `(* ... *)` are not supported

- **Fortran**
  - Extensions: `.f90`, `.f95`, `.f03`, `.f08`, `.f`, `.for`
  - Comments: `!`

- **Gherkin (Cucumber BDD)**
  - Extension: `.feature`
  - Comments: `#`

- **Go**
  - Extension: `.go`
  - Comments: `//`, `/* ... */`

- **Gradle**
  - Extensions: `.gradle`, `.kts`
  - Comments: `//`, `/* ... */`

- **Groovy**
  - Extension: `.groovy`
  - Comments: `//`, `/* ... */`

- **Handlebars / Mustache**
  - Extensions: `.hbs`, `.handlebars`, `.mustache`
  - Comments: `{{! ... }}`

- **Haskell**
  - Extension: `.hs`
  - Comments: `--`
  - ⚠️ Multiline comments `{- -}` are not supported

- **Haxe**
  - Extension: `.hx`
  - Comments: `//`, `/* ... */`

- **HCL (HashiCorp Configuration Language)**
  - Extension: `.hcl`
  - Comments: `#`, `//`, `/* ... */`

- **HTML**
  - Extensions: `.html`, `.htm`
  - Comments: `<!-- ... -->`

- **Java**
  - Extension: `.java`
  - Comments: `//`, `/* ... */`

- **JavaScript / TypeScript**
  - Extensions: `.js`, `.ts`
  - Comments: `//`, `/* ... */`

- **Jinja2**
  - Extensions: `.jinja`, `.j2`
  - Comments: `{# ... #}`

- **JSONC (JSON with Comments)**
  - Extension: `.jsonc`
  - File: `tsconfig.json`
  - Comments: `//`, `/* ... */`

- **JSX / TSX (React)**
  - Extensions: `.jsx`, `.tsx`
  - Comments: `//`, `/* ... */`

- **Julia**
  - Extension: `.jl`
  - Comments: `#`
  - ⚠️ Multiline comments `#= =#` are not supported

- **Kotlin**
  - Extension: `.kt`
  - Comments: `//`, `/* ... */`

- **LaTeX**
  - Extension: `.tex`
  - Comments: `%`

- **Less**
  - Extension: `.less`
  - Comments: `//`, `/* ... */`

- **Lisp / Clojure / Scheme**
  - Extensions: `.lisp`, `.clj`, `.scm`
  - Comments: `;`
  - ⚠️ Multiline comments `#| ... |#` are not supported

- **Liquid**
  - Extension: `.liquid`
  - Comments: `{% comment %} ... {% endcomment %}`

- **Makefile**
  - Extension: `.mk`
  - File: `Makefile`
  - Comments: `#`

- **Markdown**
  - Extension: `.md`
  - Comments: `<!-- ... -->`

- **Objective-C, MATLAB**
  - Extensions: `.m`, `.mm`
  - Comments: `//`, `%`, `/* ... */`

- **Org-mode**
  - Extension: `.org`
  - Comments: `#`

- **Perl**
  - Extension: `.pl`, `.pm`
  - Comments: `#`

- **PHP**
  - Extension: `.php`
  - Comments: `//`, `#`, `/* ... */`

- **PowerShell**
  - Extension: `.ps1`
  - Comments: `#`, `<# ... #>`

- **Python**
  - Extension: `.py`
  - Comments: `#`, `""" ... """`, `''' ... '''`
  - ⚠️ Triple-quoted strings are treated as docstrings and will be removed, even if used as actual strings.

- **QML (Qt Modeling Language)**
  - Extension: `.qml`
  - Comments: `//`, `/* ... */`

- **R**
  - Extensions: `.r`
  - Comments: `#`

- **Rego (Open Policy Agent)**
  - Extension: `.rego`
  - Comments: `#`

- **Ruby**
  - Extension: `.rb`
  - Comments: `#`

- **Rust**
  - Extension: `.rs`
  - Comments: `//`, `/* ... */`

- **SASS / SCSS**
  - Extensions: `.sass`, `.scss`
  - Comments: `//`, `/* ... */`


- **Scala**
  - Extension: `.scala`
  - Comments: `//`, `/* ... */`

- **Shell / Bash**
  - Extensions: `.sh`, `.bash`
  - Comments: `#`

- **Starlark (Bazel build language)**
  - Extension: `.bzl`
  - Comments: `#`

- **Swift**
  - Extension: `.swift`
  - Comments: `//`, `/* ... */`

- **SQL**
  - Extensions: `.sql`
  - Comments: `--`, `/* ... */`

- **Terraform (HCL)**
  - Extension: `.tf`
  - Comments: `#`, `//`, `/* ... */`

- **Twig**
  - Extension: `.twig`
  - Comments: `{# ... #}`

- **Verilog / SystemVerilog**
  - Extensions: `.v`, `.sv`, `.svh`
  - Comments: `//`, `/* ... */`

- **VB / VBScript**
  - Extensions: `.vb`, `.vbs`
  - Comments: `'`

- **Vue**
  - Extension: `.vue`
  - Comments: `//`, `<!-- ... -->`,  `/* ... */`

- **XML**
  - Extensions: `.xml`, `.xsd`, `.pom`
  - Comments: `<!-- ... -->`

- **YAML / TOML / INI**
  - Extensions: `.yaml`, `.yml`, `.toml`, `.ini`
  - Comments: `#`

- **Zig**
  - Extension: `.zig`
  - Comments: `//`, `/* ... */`
