use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::comment_remover::remove_all_comments::remove_all_comments;
use warrah::test_utils::fixture::fixture_text;

fn benchmark_comment_removal(c: &mut Criterion) {
    let content = fixture_text("javascript/benches/comment_remover.js");
    let size_bytes = content.len();
    let size_kb = size_bytes as f64 / 1024.0;
    println!("== Test content size: {:.2} KB", size_kb);

    c.bench_function("remove_comments", |b| {
        b.iter(|| {
            remove_all_comments(
                black_box(&content),
                black_box(&[("//", None), ("some_marker", None), ("/*", Some("*/"))]),
                true,
            )
        })
    });
}

criterion_group!(benches, benchmark_comment_removal);
criterion_main!(benches);
