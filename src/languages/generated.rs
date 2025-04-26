// The code was automatically generated from docs/languages.md

use super::comment_config::CommentConfig;

pub static EXTENSION_TO_LANGUAGE: &[(&str, &str)] = &[
    (".adb", "ada"),
    (".ads", "ada"),
    (".asm", "assembly"),
    (".s", "assembly"),
    (".cs", "c#"),
    (".c", "c / c++"),
    (".cpp", "c / c++"),
    (".h", "c / c++"),
    (".hpp", "c / c++"),
    (".cob", "cobol"),
    (".cbl", "cobol"),
    (".cmake", "cmake"),
    (".coffee", "coffeescript"),
    (".css", "css"),
    (".env", "config files / scripting (misc)"),
    (".conf", "config files / scripting (misc)"),
    (".gitignore", "config files / scripting (misc)"),
    (".cfg", "config files / scripting (misc)"),
    (".d", "d"),
    (".dart", "dart"),
    (".ejs", "ejs (embedded javascript)"),
    (".ex", "elixir / erlang"),
    (".exs", "elixir / erlang"),
    (".erl", "elixir / erlang"),
    (".elm", "elm"),
    (".fs", "f#"),
    (".fsx", "f#"),
    (".f90", "fortran"),
    (".f95", "fortran"),
    (".f03", "fortran"),
    (".f08", "fortran"),
    (".f", "fortran"),
    (".for", "fortran"),
    (".feature", "gherkin (cucumber bdd)"),
    (".go", "go"),
    (".gradle", "gradle"),
    (".kts", "gradle"),
    (".groovy", "groovy"),
    (".hbs", "handlebars / mustache"),
    (".handlebars", "handlebars / mustache"),
    (".mustache", "handlebars / mustache"),
    (".hs", "haskell"),
    (".hx", "haxe"),
    (".hcl", "hcl (hashicorp configuration language)"),
    (".html", "html"),
    (".htm", "html"),
    (".java", "java"),
    (".js", "javascript / typescript"),
    (".ts", "javascript / typescript"),
    (".jinja", "jinja2"),
    (".j2", "jinja2"),
    (".jsonc", "jsonc (json with comments)"),
    (".jsx", "jsx / tsx (react)"),
    (".tsx", "jsx / tsx (react)"),
    (".jl", "julia"),
    (".kt", "kotlin"),
    (".tex", "latex"),
    (".less", "less"),
    (".lisp", "lisp / clojure / scheme"),
    (".clj", "lisp / clojure / scheme"),
    (".scm", "lisp / clojure / scheme"),
    (".liquid", "liquid"),
    (".mk", "makefile"),
    (".md", "markdown"),
    (".m", "objective-c, matlab"),
    (".mm", "objective-c, matlab"),
    (".org", "org-mode"),
    (".pl", "perl"),
    (".pm", "perl"),
    (".php", "php"),
    (".ps1", "powershell"),
    (".py", "python"),
    (".qml", "qml (qt modeling language)"),
    (".r", "r"),
    (".rego", "rego (open policy agent)"),
    (".rb", "ruby"),
    (".rs", "rust"),
    (".sass", "sass / scss"),
    (".scss", "sass / scss"),
    (".scala", "scala"),
    (".sh", "shell / bash"),
    (".bash", "shell / bash"),
    (".bzl", "starlark (bazel build language)"),
    (".swift", "swift"),
    (".sql", "sql"),
    (".tf", "terraform (hcl)"),
    (".twig", "twig"),
    (".v", "verilog / systemverilog"),
    (".sv", "verilog / systemverilog"),
    (".svh", "verilog / systemverilog"),
    (".vb", "vb / vbscript"),
    (".vbs", "vb / vbscript"),
    (".vue", "vue"),
    (".xml", "xml"),
    (".xsd", "xml"),
    (".pom", "xml"),
    (".yaml", "yaml / toml / ini"),
    (".yml", "yaml / toml / ini"),
    (".toml", "yaml / toml / ini"),
    (".ini", "yaml / toml / ini"),
    (".zig", "zig"),
];

pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[
    ("cmakelists.txt", "cmake"),
    ("dockerfile", "dockerfile"),
    ("tsconfig.json", "jsonc (json with comments)"),
    ("makefile", "makefile"),
];

pub static LANGUAGE_TO_COMMENTS: &[(&str, CommentConfig)] = &[
    ("ada", CommentConfig {
        single_line: &[
            "--",
        ],
        multi_line: &[
        ],
    }),
    ("assembly", CommentConfig {
        single_line: &[
            ";",
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("c#", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("c / c++", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("cobol", CommentConfig {
        single_line: &[
            "*>",
        ],
        multi_line: &[
        ],
    }),
    ("cmake", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("coffeescript", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
            ("###", "###"),
        ],
    }),
    ("css", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("config files / scripting (misc)", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("d", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("dart", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("dockerfile", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("ejs (embedded javascript)", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("<%--", "--%>"),
        ],
    }),
    ("elixir / erlang", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("elm", CommentConfig {
        single_line: &[
            "--",
        ],
        multi_line: &[
        ],
    }),
    ("f#", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
        ],
    }),
    ("fortran", CommentConfig {
        single_line: &[
            "!",
        ],
        multi_line: &[
        ],
    }),
    ("gherkin (cucumber bdd)", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("go", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("gradle", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("groovy", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("handlebars / mustache", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("{{!", "}}"),
        ],
    }),
    ("haskell", CommentConfig {
        single_line: &[
            "--",
        ],
        multi_line: &[
        ],
    }),
    ("haxe", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("hcl (hashicorp configuration language)", CommentConfig {
        single_line: &[
            "#",
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("html", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("<!--", "-->"),
        ],
    }),
    ("java", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("javascript / typescript", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("jinja2", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("{#", "#}"),
        ],
    }),
    ("jsonc (json with comments)", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("jsx / tsx (react)", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("julia", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("kotlin", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("latex", CommentConfig {
        single_line: &[
            "%",
        ],
        multi_line: &[
        ],
    }),
    ("less", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("lisp / clojure / scheme", CommentConfig {
        single_line: &[
            ";",
        ],
        multi_line: &[
        ],
    }),
    ("liquid", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("{% comment %}", "{% endcomment %}"),
        ],
    }),
    ("makefile", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("markdown", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("<!--", "-->"),
        ],
    }),
    ("objective-c, matlab", CommentConfig {
        single_line: &[
            "//",
            "%",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("org-mode", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("perl", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("php", CommentConfig {
        single_line: &[
            "//",
            "#",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("powershell", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
            ("<#", "#>"),
        ],
    }),
    ("python", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
            ("\"\"\"", "\"\"\""),
            ("'''", "'''"),
        ],
    }),
    ("qml (qt modeling language)", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("r", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("rego (open policy agent)", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("ruby", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("rust", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("sass / scss", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("scala", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("shell / bash", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("starlark (bazel build language)", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("swift", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("sql", CommentConfig {
        single_line: &[
            "--",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("terraform (hcl)", CommentConfig {
        single_line: &[
            "#",
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("twig", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("{#", "#}"),
        ],
    }),
    ("verilog / systemverilog", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
    ("vb / vbscript", CommentConfig {
        single_line: &[
            "'",
        ],
        multi_line: &[
        ],
    }),
    ("vue", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("<!--", "-->"),
            ("/*", "*/"),
        ],
    }),
    ("xml", CommentConfig {
        single_line: &[
        ],
        multi_line: &[
            ("<!--", "-->"),
        ],
    }),
    ("yaml / toml / ini", CommentConfig {
        single_line: &[
            "#",
        ],
        multi_line: &[
        ],
    }),
    ("zig", CommentConfig {
        single_line: &[
            "//",
        ],
        multi_line: &[
            ("/*", "*/"),
        ],
    }),
];
