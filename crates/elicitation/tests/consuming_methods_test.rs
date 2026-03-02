//! Tests for consuming methods in elicit_newtype_methods! macro.

use elicitation::elicit_newtype_methods;

// Test with a simple builder pattern
#[derive(Debug, Clone, PartialEq)]
pub struct BuilderInner {
    value: i32,
}

impl BuilderInner {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn add(self, n: i32) -> Self {
        Self {
            value: self.value + n,
        }
    }

    fn multiply(self, n: i32) -> Self {
        Self {
            value: self.value * n,
        }
    }

    fn build(self) -> i32 {
        self.value
    }
}

elicit_newtype_methods! {
    Builder => BuilderInner,
    consuming fn add(n: i32) -> Self;
    consuming fn multiply(n: i32) -> Self;
    consuming fn build() -> i32;
}

impl Builder {
    fn new() -> Self {
        BuilderInner::new().into()
    }
}

#[test]
fn test_consuming_method_chain() {
    let result = Builder::new()
        .add(5) // Consuming: unwraps Arc
        .multiply(3) // Consuming: unwraps Arc
        .add(2) // Consuming: unwraps Arc
        .build(); // Consuming: unwraps Arc

    assert_eq!(result, 17); // (0 + 5) * 3 + 2 = 17
}

#[test]
fn test_consuming_method_single_step() {
    let builder = Builder::new();
    let builder = builder.add(10);
    let result = builder.build();

    assert_eq!(result, 10);
}

#[test]
fn test_consuming_method_with_clone() {
    // Test the clone path: clone wrapper, then consume
    let builder = Builder::new().add(5);
    let builder2 = builder.clone(); // Arc refcount becomes 2

    // Both should still work (inner BuilderInner is Clone)
    let result1 = builder.multiply(2).build(); // Clones inner, then builds
    let result2 = builder2.multiply(3).build(); // Clones inner, then builds

    assert_eq!(result1, 10); // 5 * 2 = 10
    assert_eq!(result2, 15); // 5 * 3 = 15
}
