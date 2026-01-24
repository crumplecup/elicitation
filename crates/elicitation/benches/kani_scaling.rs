use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::process::Command;
use std::time::Duration;

/// Run a single Kani harness and return verification time
fn run_kani_harness(harness_name: &str) -> Duration {
    let start = std::time::Instant::now();

    let output = Command::new("cargo")
        .args(&[
            "kani",
            "--features",
            "verify-kani",
            "--harness",
            harness_name,
        ])
        .output()
        .expect("Failed to run Kani");

    let elapsed = start.elapsed();

    // Verify it succeeded
    if !output.status.success() {
        panic!("Kani verification failed for {}", harness_name);
    }

    elapsed
}

fn bench_kani_problem_spaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("kani_verification");

    // Reduce sample size since each run is expensive (seconds)
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(120));

    // Micro-benchmarks: problem_space_size -> harness_name
    let benchmarks = vec![
        (1, "bench_concrete_1_byte"),
        (4, "bench_2byte_2x2"),
        (6, "bench_2byte_2x3"),
        (9, "bench_2byte_3x3"),
        (16, "bench_2byte_4x4"),
        (16, "bench_4byte_2x2x2x2"),
    ];

    for (problem_space, harness_name) in benchmarks {
        group.bench_with_input(
            BenchmarkId::new("symbolic_utf8", problem_space),
            &harness_name,
            |b, &harness| {
                b.iter(|| {
                    let duration = run_kani_harness(black_box(harness));
                    duration
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_kani_problem_spaces);
criterion_main!(benches);
