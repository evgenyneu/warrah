use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::comment_remover::remove_all_comments::remove_all_comments;
use warrah::test_utils::fixture::fixture_text;

fn generate_test_content() -> String {
    let content = fixture_text("javascript/benches/comment_remover.js");
    content.repeat(10000)
}

fn benchmark_comment_removal_large_file(c: &mut Criterion) {
    let content = generate_test_content();
    let size_bytes = content.len();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    println!("== Test content size: {:.2} MB", size_mb);

    c.bench_function("remove_comments_large_file", |b| {
        b.iter(|| {
            remove_all_comments(
                black_box(&content),
                black_box(&[("//", None), ("some_marker", None), ("/*", Some("*/"))]),
                true,
            )
        })
    });
}

criterion_group!(benches, benchmark_comment_removal_large_file);
criterion_main!(benches);
