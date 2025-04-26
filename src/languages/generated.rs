// The code was automatically generated from docs/languages.md

use super::comment_config::CommentConfig;

pub static EXTENSION_TO_LANGUAGE: &[(&str, &str)] = &[
    (".adb", "ada"),
    (".ads", "ada"),
    (".asm", "assembly"),
    (".bash", "shell / bash"),
    (".bzl", "starlark (bazel build language)"),
    (".c", "c / c++"),
    (".cbl", "cobol"),
    (".cfg", "config files / scripting (misc)"),
    (".clj", "lisp / clojure / scheme"),
    (".cmake", "cmake"),
    (".cob", "cobol"),
    (".coffee", "coffeescript"),
    (".conf", "config files / scripting (misc)"),
    (".cpp", "c / c++"),
    (".cs", "c#"),
    (".css", "css"),
    (".d", "d"),
    (".dart", "dart"),
    (".ejs", "ejs (embedded javascript)"),
    (".elm", "elm"),
    (".env", "config files / scripting (misc)"),
    (".erl", "elixir / erlang"),
    (".ex", "elixir / erlang"),
    (".exs", "elixir / erlang"),
    (".f", "fortran"),
    (".f03", "fortran"),
    (".f08", "fortran"),
    (".f90", "fortran"),
    (".f95", "fortran"),
    (".feature", "gherkin (cucumber bdd)"),
    (".for", "fortran"),
    (".fs", "f#"),
    (".fsx", "f#"),
    (".gitignore", "config files / scripting (misc)"),
    (".go", "go"),
    (".gradle", "gradle"),
    (".groovy", "groovy"),
    (".h", "c / c++"),
    (".handlebars", "handlebars / mustache"),
    (".hbs", "handlebars / mustache"),
    (".hcl", "hcl (hashicorp configuration language)"),
    (".hpp", "c / c++"),
    (".hs", "haskell"),
    (".htm", "html"),
    (".html", "html"),
    (".hx", "haxe"),
    (".ini", "yaml / toml / ini"),
    (".j2", "jinja2"),
    (".java", "java"),
    (".jinja", "jinja2"),
    (".jl", "julia"),
    (".js", "javascript / typescript"),
    (".jsonc", "jsonc (json with comments)"),
    (".jsx", "jsx / tsx (react)"),
    (".kt", "kotlin"),
    (".kts", "gradle"),
    (".less", "less"),
    (".liquid", "liquid"),
    (".lisp", "lisp / clojure / scheme"),
    (".m", "objective-c, matlab"),
    (".md", "markdown"),
    (".mk", "makefile"),
    (".mm", "objective-c, matlab"),
    (".mustache", "handlebars / mustache"),
    (".org", "org-mode"),
    (".php", "php"),
    (".pl", "perl"),
    (".pm", "perl"),
    (".pom", "xml"),
    (".ps1", "powershell"),
    (".py", "python"),
    (".qml", "qml (qt modeling language)"),
    (".r", "r"),
    (".rb", "ruby"),
    (".rego", "rego (open policy agent)"),
    (".rs", "rust"),
    (".s", "assembly"),
    (".sass", "sass / scss"),
    (".scala", "scala"),
    (".scm", "lisp / clojure / scheme"),
    (".scss", "sass / scss"),
    (".sh", "shell / bash"),
    (".sql", "sql"),
    (".sv", "verilog / systemverilog"),
    (".svh", "verilog / systemverilog"),
    (".swift", "swift"),
    (".tex", "latex"),
    (".tf", "terraform (hcl)"),
    (".toml", "yaml / toml / ini"),
    (".ts", "javascript / typescript"),
    (".tsx", "jsx / tsx (react)"),
    (".twig", "twig"),
    (".v", "verilog / systemverilog"),
    (".vb", "vb / vbscript"),
    (".vbs", "vb / vbscript"),
    (".vue", "vue"),
    (".xml", "xml"),
    (".xsd", "xml"),
    (".yaml", "yaml / toml / ini"),
    (".yml", "yaml / toml / ini"),
    (".zig", "zig"),
];

pub static FILENAME_TO_LANGUAGE: &[(&str, &str)] = &[
    ("cmakelists.txt", "cmake"),
    ("dockerfile", "dockerfile"),
    ("makefile", "makefile"),
    ("tsconfig.json", "jsonc (json with comments)"),
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
