use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::comment_remover::multi_line::remove_all_comments;

fn generate_test_content() -> String {
    let mut content = String::with_capacity(1024 * 1024);
    for i in 0..100000 {
        content.push_str(&format!("let x{} = {}; // comment {}\n", i, i, i));
    }
    content
}

fn benchmark_multi_line_comment_removal(c: &mut Criterion) {
    let content = generate_test_content();

    c.bench_function("remove_all_comments", |b| {
        b.iter(|| {
            remove_all_comments(
                black_box(&content),
                black_box(&["//", "<--"]),
                black_box(&[]),
            )
        })
    });
}

criterion_group!(benches, benchmark_multi_line_comment_removal);
criterion_main!(benches);
