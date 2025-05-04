// The code was automatically generated from docs/languages.md

pub static EXTENSION_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[
    ("adb", &[
        ("--", None),
    ]),
    ("ads", &[
        ("--", None),
    ]),
    ("asm", &[
        (";", None),
        ("#", None),
    ]),
    ("bash", &[
        ("#", None),
    ]),
    ("bzl", &[
        ("#", None),
    ]),
    ("c", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("cbl", &[
        ("*>", None),
    ]),
    ("cfg", &[
        ("#", None),
    ]),
    ("clj", &[
        (";", None),
    ]),
    ("cmake", &[
        ("#", None),
    ]),
    ("cob", &[
        ("*>", None),
    ]),
    ("coffee", &[
        ("#", None),
        ("###", Some("###")),
    ]),
    ("conf", &[
        ("#", None),
    ]),
    ("cpp", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("cs", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("css", &[
        ("/*", Some("*/")),
    ]),
    ("d", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("dart", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("ejs", &[
        ("<%--", Some("--%>")),
    ]),
    ("elm", &[
        ("--", None),
    ]),
    ("env", &[
        ("#", None),
    ]),
    ("erl", &[
        ("#", None),
    ]),
    ("ex", &[
        ("#", None),
    ]),
    ("exs", &[
        ("#", None),
    ]),
    ("f", &[
        ("!", None),
    ]),
    ("f03", &[
        ("!", None),
    ]),
    ("f08", &[
        ("!", None),
    ]),
    ("f90", &[
        ("!", None),
    ]),
    ("f95", &[
        ("!", None),
    ]),
    ("feature", &[
        ("#", None),
    ]),
    ("for", &[
        ("!", None),
    ]),
    ("fs", &[
        ("//", None),
    ]),
    ("fsx", &[
        ("//", None),
    ]),
    ("gitignore", &[
        ("#", None),
    ]),
    ("go", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("gradle", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("groovy", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("h", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("handlebars", &[
        ("{{!", Some("}}")),
    ]),
    ("hbs", &[
        ("{{!", Some("}}")),
    ]),
    ("hcl", &[
        ("#", None),
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("hpp", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("hs", &[
        ("--", None),
    ]),
    ("htm", &[
        ("<!--", Some("-->")),
    ]),
    ("html", &[
        ("<!--", Some("-->")),
    ]),
    ("hx", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("ini", &[
        ("#", None),
    ]),
    ("j2", &[
        ("{#", Some("#}")),
    ]),
    ("java", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("jinja", &[
        ("{#", Some("#}")),
    ]),
    ("jl", &[
        ("#", None),
    ]),
    ("js", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("jsonc", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("jsx", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("kt", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("kts", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("less", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("liquid", &[
        ("{% comment %}", Some("{% endcomment %}")),
    ]),
    ("lisp", &[
        (";", None),
    ]),
    ("m", &[
        ("//", None),
        ("%", None),
        ("/*", Some("*/")),
    ]),
    ("md", &[
        ("<!--", Some("-->")),
    ]),
    ("mk", &[
        ("#", None),
    ]),
    ("mm", &[
        ("//", None),
        ("%", None),
        ("/*", Some("*/")),
    ]),
    ("mustache", &[
        ("{{!", Some("}}")),
    ]),
    ("org", &[
        ("#", None),
    ]),
    ("php", &[
        ("//", None),
        ("#", None),
        ("/*", Some("*/")),
    ]),
    ("pl", &[
        ("#", None),
    ]),
    ("pm", &[
        ("#", None),
    ]),
    ("pom", &[
        ("<!--", Some("-->")),
    ]),
    ("ps1", &[
        ("#", None),
        ("<#", Some("#>")),
    ]),
    ("py", &[
        ("#", None),
        ("\"\"\"", Some("\"\"\"")),
        ("'''", Some("'''")),
    ]),
    ("qml", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("r", &[
        ("#", None),
    ]),
    ("rb", &[
        ("#", None),
    ]),
    ("rego", &[
        ("#", None),
    ]),
    ("rs", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("s", &[
        (";", None),
        ("#", None),
    ]),
    ("sass", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("scala", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("scm", &[
        (";", None),
    ]),
    ("scss", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("sh", &[
        ("#", None),
    ]),
    ("sql", &[
        ("--", None),
        ("/*", Some("*/")),
    ]),
    ("sv", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("svh", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("swift", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("tex", &[
        ("%", None),
    ]),
    ("tf", &[
        ("#", None),
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("toml", &[
        ("#", None),
    ]),
    ("ts", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("tsx", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("twig", &[
        ("{#", Some("#}")),
    ]),
    ("v", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
    ("vb", &[
        ("'", None),
    ]),
    ("vbs", &[
        ("'", None),
    ]),
    ("vue", &[
        ("//", None),
        ("<!--", Some("-->")),
        ("/*", Some("*/")),
    ]),
    ("xml", &[
        ("<!--", Some("-->")),
    ]),
    ("xsd", &[
        ("<!--", Some("-->")),
    ]),
    ("yaml", &[
        ("#", None),
    ]),
    ("yml", &[
        ("#", None),
    ]),
    ("zig", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
];

pub static FILENAME_TO_MARKERS: &[(&str, &[(&str, Option<&str>)])] = &[
    ("cmakelists.txt", &[
        ("#", None),
    ]),
    ("dockerfile", &[
        ("#", None),
    ]),
    ("makefile", &[
        ("#", None),
    ]),
    ("tsconfig.json", &[
        ("//", None),
        ("/*", Some("*/")),
    ]),
];
