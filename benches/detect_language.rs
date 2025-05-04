use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::languages::language_maps::get_markers_by_extension;

fn generate_test_paths() -> Vec<String> {
    vec![
        ".rs".to_string(),     // Rust
        ".py".to_string(),     // Python
        ".js".to_string(),     // JavaScript
        ".ts".to_string(),     // TypeScript
        ".java".to_string(),   // Java
        ".cpp".to_string(),    // C++
        ".c".to_string(),      // C
        ".go".to_string(),     // Go
        ".swift".to_string(),  // Swift
        ".rb".to_string(),     // Ruby
        ".php".to_string(),    // PHP
        ".cs".to_string(),     // C#
        ".dart".to_string(),   // Dart
        ".kt".to_string(),     // Kotlin
        ".scala".to_string(),  // Scala
        ".groovy".to_string(), // Groovy
        ".hs".to_string(),     // Haskell
        ".fs".to_string(),     // F#
        ".ex".to_string(),     // Elixir
        ".exs".to_string(),    // Elixir
        ".erl".to_string(),    // Erlang
        ".clj".to_string(),    // Clojure
        ".lisp".to_string(),   // Lisp
        ".scm".to_string(),    // Scheme
        ".zig".to_string(),    // Zig
        ".v".to_string(),      // Verilog
        ".sv".to_string(),     // SystemVerilog
        ".vb".to_string(),     // VB
        ".vbs".to_string(),    // VBScript
        ".zig".to_string(),    // Zig
    ]
}

fn benchmark_language_detection(c: &mut Criterion) {
    let extensions = generate_test_paths();

    println!(
        "== Testing with {} different language extensions",
        extensions.len()
    );

    c.bench_function("detect_language_by_extension", |b| {
        b.iter(|| {
            for ext in &extensions {
                get_markers_by_extension(black_box(ext));
            }
        })
    });
}

criterion_group!(benches, benchmark_language_detection);
criterion_main!(benches);
