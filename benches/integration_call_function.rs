use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warrah::process::file_path::remove_comments;
use warrah::test_utils::fixture::fixture_path;

fn bench_integration_call_function(c: &mut Criterion) {
    let input_path = fixture_path("javascript/benches/comment_remover.js");

    c.bench_function("integration_call_function", |b| {
        b.iter(|| {
            let output = remove_comments(input_path.clone(), 10 * 1024).unwrap();
            black_box(output)
        })
    });
}

criterion_group!(benches, bench_integration_call_function);
criterion_main!(benches);
