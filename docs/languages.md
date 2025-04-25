# Supported languages

Here is the list of supported languages, in which Warrah will remove comments. The languages is detected based on the specifies file extensions or file names.  Please note comments inside strings will also be removed and result in invalid code.

- **Assembly**
  - Extensions: `.asm`, `.s`
  - Comments: `;`, `#`

- **C / C++**
  - Extensions: `.c`, `.cpp`, `.h`, `.hpp`
  - Comments: `//`, `/* ... */`

- **C#**
  - Extension: `.cs`
  - Comments: `//`, `/* ... */`

- **CMake**
  - File: `CMakeLists.txt`, `.cmake`
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
  - Extension: `Dockerfile`
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
  - Extensions: `.gradle`, `.gradle.kts`
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
  - Extensions: `.jinja`, `.j2`, `.html`
  - Comments: `{# ... #}`

- **JSONC (JSON with Comments)**
  - Extensions: `.jsonc`, `tsconfig.json`
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
  - Extension: `Makefile`, `.mk`
  - Comments: `#`

- **Markdown**
  - Extension: `.md`
  - Comments: `<!-- ... -->`

- **MATLAB / Octave**
  - Extension: `.m`
  - Comments: `%`

- **Objective-C**
  - Extensions: `.m`, `.mm`, `.h`
  - Comments: `//`, `/* ... */`

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

- **Prolog**
  - Extension: `.pl`
  - Comments: `%`, `/* ... */`

- **Python** (partial)
  - Extension: `.py`
  - Comments: `#`
  - ⚠️ Doesn’t handle `""" docstrings """`

- **QML (Qt Modeling Language)**
  - Extension: `.qml`
  - Comments: `//`, `/* ... */`

- **R**
  - Extension: `.r`, `.R`
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
  - Comments:
    - HTML: `<!-- ... -->`
    - JS: `//`, `/* ... */`
    - CSS: `/* ... */`

- **XML**
  - Extensions: `.xml`, `.xsd`, `.pom`, etc.
  - Comments: `<!-- ... -->`

- **YAML / TOML / INI**
  - Extensions: `.yaml`, `.yml`, `.toml`, `.ini`
  - Comments: `#`

- **Zig**
  - Extension: `.zig`
  - Comments: `//`, `/* ... */`


Of course! Here you go, ready to paste in:

---
