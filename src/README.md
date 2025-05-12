# Warrah Library

A Rust library that sloppily removes code comments from a text file. Supports [60+ programming languages](../docs/languages.md).

It's a sloppy program because it doesn't fully parse the code. It uses simple and fast logic to detect single- and multi-line comments. Consequently, it will remove comments inside strings and may result in invalid code.

The main use of this utility is to strip comments from code in order to feed it to an LLM, where occasionally incorrect code is fine. Thus the main focus here is on speed, low resource usage, language coverage and simplicity, not correctness.

## Main Functions

### Remove comments from a file

```rust
use warrah::process::file_path::remove_comments;
use std::path::PathBuf;

let path = PathBuf::from("/dir/example.rs");

// Remove comments from a file given a path (case insensitive)
let result = remove_comments(path, 1024 * 1024, true)?;
```

### Remove comments from a string

```rust
use warrah::comment_remover::remove_all_comments::remove_all_comments;

let content = "let x = 1; // comment\nlet y = 2;";

// Remove comments from a string given comment markers
let result = remove_all_comments(
    content,
    &[("//", None), ("/*", Some("*/"))],
    true
);
```

Note: use the functions below to automatically detect comment markers.

### Detect language and return comment markers

From file path:

```rust
use std::path::PathBuf;
use warrah::process::file_path::get_marker_by_file_path;

let path = PathBuf::from("/dir/example.rs");
let markers = get_marker_by_file_path(&path).unwrap();
```

From file extension or filename:

```rust
use warrah::languages::language_maps::{get_markers_by_extension, get_markers_by_filename};

// Get comment markers by file extension (must be lowercase)
let markers = get_markers_by_extension("rs")?;

// Get comment markers by filename (must be lowercase)
let markers = get_markers_by_filename("dockerfile")?;
```

## The unlicense

This work is in public domain.
