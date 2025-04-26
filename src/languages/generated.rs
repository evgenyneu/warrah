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
        single_line: vec![
            "--".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("assembly", CommentConfig {
        single_line: vec![
            ";".to_string(),
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("c#", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("c / c++", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("cobol", CommentConfig {
        single_line: vec![
            "*>".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("cmake", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("coffeescript", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
            ("###".to_string(), "###".to_string()),
        ],
    }),
    ("css", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("config files / scripting (misc)", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("d", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("dart", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("dockerfile", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("ejs (embedded javascript)", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("<%--".to_string(), "--%>".to_string()),
        ],
    }),
    ("elixir / erlang", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("elm", CommentConfig {
        single_line: vec![
            "--".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("f#", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("fortran", CommentConfig {
        single_line: vec![
            "!".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("gherkin (cucumber bdd)", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("go", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("gradle", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("groovy", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("handlebars / mustache", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("{{!".to_string(), "}}".to_string()),
        ],
    }),
    ("haskell", CommentConfig {
        single_line: vec![
            "--".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("haxe", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("hcl (hashicorp configuration language)", CommentConfig {
        single_line: vec![
            "#".to_string(),
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("html", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("<!--".to_string(), "-->".to_string()),
        ],
    }),
    ("java", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("javascript / typescript", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("jinja2", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("{#".to_string(), "#}".to_string()),
        ],
    }),
    ("jsonc (json with comments)", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("jsx / tsx (react)", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("julia", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("kotlin", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("latex", CommentConfig {
        single_line: vec![
            "%".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("less", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("lisp / clojure / scheme", CommentConfig {
        single_line: vec![
            ";".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("liquid", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("{% comment %}".to_string(), "{% endcomment %}".to_string()),
        ],
    }),
    ("makefile", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("markdown", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("<!--".to_string(), "-->".to_string()),
        ],
    }),
    ("objective-c, matlab", CommentConfig {
        single_line: vec![
            "//".to_string(),
            "%".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("org-mode", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("perl", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("php", CommentConfig {
        single_line: vec![
            "//".to_string(),
            "#".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("powershell", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
            ("<#".to_string(), "#>".to_string()),
        ],
    }),
    ("python", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
            ("\"\"\"".to_string(), "\"\"\"".to_string()),
            ("'''".to_string(), "'''".to_string()),
        ],
    }),
    ("qml (qt modeling language)", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("r", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("rego (open policy agent)", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("ruby", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("rust", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("sass / scss", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("scala", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("shell / bash", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("starlark (bazel build language)", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("swift", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("sql", CommentConfig {
        single_line: vec![
            "--".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("terraform (hcl)", CommentConfig {
        single_line: vec![
            "#".to_string(),
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("twig", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("{#".to_string(), "#}".to_string()),
        ],
    }),
    ("verilog / systemverilog", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("vb / vbscript", CommentConfig {
        single_line: vec![
            "'".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("vue", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("<!--".to_string(), "-->".to_string()),
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
    ("xml", CommentConfig {
        single_line: vec![
        ],
        multi_line: vec![
            ("<!--".to_string(), "-->".to_string()),
        ],
    }),
    ("yaml / toml / ini", CommentConfig {
        single_line: vec![
            "#".to_string(),
        ],
        multi_line: vec![
        ],
    }),
    ("zig", CommentConfig {
        single_line: vec![
            "//".to_string(),
        ],
        multi_line: vec![
            ("/*".to_string(), "*/".to_string()),
        ],
    }),
];
