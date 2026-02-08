//! Integration test runner for rand Kani proofs.
//!
//! Run with: cargo test --test rand_verification_runner -- --ignored

#[cfg(kani)]
use elicitation_rand::verification::runner;

#[test]
#[ignore] // Only run explicitly to avoid CI overhead
#[cfg(kani)]
fn run_all_rand_kani_proofs() {
    let output_path = std::path::Path::new("rand_kani_verification_results.csv");
    
    let results = runner::run_all_proofs(output_path)
        .expect("Failed to run Kani proofs");
    
    // Assert all proofs passed
    let failed = results.iter().filter(|r| r.status == "FAIL").count();
    assert_eq!(
        failed, 0,
        "{} proofs failed - see output above",
        failed
    );
}
