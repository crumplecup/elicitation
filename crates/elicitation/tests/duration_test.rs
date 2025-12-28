//! Tests for Duration implementation.

use elicitation::{Elicitation, Prompt};
use std::time::Duration;

#[test]
fn test_duration_has_prompt() {
    assert!(Duration::prompt().is_some());
}

#[test]
fn test_duration_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Duration>();
}

#[test]
fn test_duration_from_secs_f64() {
    // Test integer seconds
    let duration = Duration::from_secs_f64(5.0);
    assert_eq!(duration.as_secs(), 5);

    // Test decimal seconds
    let duration = Duration::from_secs_f64(1.5);
    assert_eq!(duration.as_millis(), 1500);

    // Test zero
    let duration = Duration::from_secs_f64(0.0);
    assert_eq!(duration.as_secs(), 0);
}

#[test]
fn test_duration_operations() {
    let d1 = Duration::from_secs(10);
    let d2 = Duration::from_secs(5);

    // Test addition
    let sum = d1 + d2;
    assert_eq!(sum.as_secs(), 15);

    // Test subtraction
    let diff = d1 - d2;
    assert_eq!(diff.as_secs(), 5);
}
