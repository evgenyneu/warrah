use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use warrah::languages::language_maps::{
    get_comments_by_language, get_language_by_extension, get_language_by_filename,
};

fn generate_test_paths() -> Vec<PathBuf> {
    // Create a diverse set of test paths covering different languages
    vec![
        PathBuf::from("src/main.rs"),     // Rust
        PathBuf::from("src/main.py"),     // Python
        PathBuf::from("src/main.js"),     // JavaScript
        PathBuf::from("src/main.ts"),     // TypeScript
        PathBuf::from("src/main.java"),   // Java
        PathBuf::from("src/main.cpp"),    // C++
        PathBuf::from("src/main.c"),      // C
        PathBuf::from("src/main.go"),     // Go
        PathBuf::from("src/main.swift"),  // Swift
        PathBuf::from("src/main.rb"),     // Ruby
        PathBuf::from("src/main.php"),    // PHP
        PathBuf::from("src/main.cs"),     // C#
        PathBuf::from("src/main.dart"),   // Dart
        PathBuf::from("src/main.kt"),     // Kotlin
        PathBuf::from("src/main.scala"),  // Scala
        PathBuf::from("src/main.groovy"), // Groovy
        PathBuf::from("src/main.hs"),     // Haskell
        PathBuf::from("src/main.fs"),     // F#
        PathBuf::from("src/main.ex"),     // Elixir
        PathBuf::from("src/main.exs"),    // Elixir
        PathBuf::from("src/main.erl"),    // Erlang
        PathBuf::from("src/main.clj"),    // Clojure
        PathBuf::from("src/main.lisp"),   // Lisp
        PathBuf::from("src/main.scm"),    // Scheme
        PathBuf::from("src/main.zig"),    // Zig
        PathBuf::from("src/main.v"),      // Verilog
        PathBuf::from("src/main.sv"),     // SystemVerilog
        PathBuf::from("src/main.vb"),     // VB
        PathBuf::from("src/main.vbs"),    // VBScript
        PathBuf::from("src/main.zig"),    // Zig
    ]
}

fn detect_language(path: &PathBuf) -> Option<&'static str> {
    // First try to get language by filename
    if let Some(lang) = get_language_by_filename(path.file_name()?.to_str()?) {
        return Some(lang);
    }

    // Then try to get language by extension
    if let Some(ext) = path.extension() {
        if let Some(lang) = get_language_by_extension(&format!(".{}", ext.to_str()?)) {
            return Some(lang);
        }
    }

    None
}

fn benchmark_language_detection(c: &mut Criterion) {
    let paths = generate_test_paths();
    println!("== Testing with {} different language paths", paths.len());

    c.bench_function("detect_language", |b| {
        b.iter(|| {
            for path in &paths {
                if let Some(language) = detect_language(black_box(path)) {
                    let _comments = get_comments_by_language(black_box(language));
                }
            }
        })
    });
}

criterion_group!(benches, benchmark_language_detection);
criterion_main!(benches);
