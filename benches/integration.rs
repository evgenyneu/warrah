use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;
use warrah::test_utils::fixture::fixture_path;

fn bench_main_success_run(c: &mut Criterion) {
    let input_path = fixture_path("javascript/benches/integration_bench.js");
    let path_str = input_path.to_str().unwrap();

    c.bench_function("main_success_run", |b| {
        b.iter(|| {
            let output = Command::new("target/release/warrah")
                .arg(path_str)
                .output()
                .expect("Failed to run warrah");

            assert!(output.status.success());
        })
    });
}

criterion_group!(benches, bench_main_success_run);
criterion_main!(benches);
