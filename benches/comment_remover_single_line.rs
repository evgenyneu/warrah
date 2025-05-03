use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::comment_remover::remove_all_comments::remove_all_comments;

fn generate_test_content() -> String {
    let mut content = String::with_capacity(1024 * 1024);
    for i in 0..699 {
        content.push_str(&format!("let x{} = {}; // comment {}\n", i, i, i));
    }
    content
}

fn benchmark_comment_removal_single_line(c: &mut Criterion) {
    let content = generate_test_content();
    let size_bytes = content.len();
    let size_kb = size_bytes as f64 / 1024.0;
    println!("== Test content size: {:.2} KB", size_kb);

    c.bench_function("remove_single_line_comments", |b| {
        b.iter(|| {
            remove_all_comments(
                black_box(&content),
                black_box(&["//", "<--"]),
                black_box(&[]),
            )
        })
    });
}

criterion_group!(benches, benchmark_comment_removal_single_line);
criterion_main!(benches);
