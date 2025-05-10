use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;
use warrah::test_utils::fixture::fixture_path;

fn bench_main_success_run(c: &mut Criterion) {
    let input_path = fixture_path("javascript/integration_bench.js");

    c.bench_function("main_success_run", |b| {
        b.iter(|| {
            let output = Command::new("cargo")
                .args(["run", "--bin", "warrah", "--", input_path.to_str().unwrap()])
                .output()
                .expect("Failed to run warrah");

            assert!(
                output.status.success(),
                "Command failed: {:?}",
                output.status
            );
        })
    });
}

criterion_group!(benches, bench_main_success_run);
criterion_main!(benches);
