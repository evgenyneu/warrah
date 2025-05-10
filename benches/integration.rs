use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;
use warrah::test_utils::fixture::{assert_eq_fixture, fixture_path};

fn bench_main_success_run(c: &mut Criterion) {
    let input_path = fixture_path("javascript/integration_bench.js");
    let path_str = input_path.to_str().unwrap();

    c.bench_function("main_success_run", |b| {
        b.iter(|| {
            let output = Command::new("target/release/warrah")
                .arg(path_str)
                .output()
                .expect("Failed to run warrah");

            assert!(
                output.status.success(),
                "Command failed: {:?}",
                output.status
            );

            let output_str =
                String::from_utf8(output.stdout).expect("Failed to convert output to string");

            assert_eq_fixture(&output_str, "javascript/integration_bench.expected.js");
        })
    });
}

criterion_group!(benches, bench_main_success_run);
criterion_main!(benches);
