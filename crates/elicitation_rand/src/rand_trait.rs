//! The Rand trait for random generation with associated Generator types.

use crate::Generator;

/// Trait for types that can be randomly generated.
///
/// Each implementation specifies its concrete generator type via the
/// associated `Gen` type. This allows different types to use different
/// generation strategies while maintaining type safety.
///
/// # Examples
///
/// Basic types use RandomGenerator:
/// ```rust,ignore
/// impl Rand for u32 {
///     type Gen = RandomGenerator<u32>;
///     fn rand_generator(seed: u64) -> Self::Gen {
///         RandomGenerator::with_seed(seed)
///     }
/// }
/// ```
///
/// Unit structs use ConstantGenerator:
/// ```rust,ignore
/// impl Rand for Marker {
///     type Gen = ConstantGenerator<Marker>;
///     fn rand_generator(_seed: u64) -> Self::Gen {
///         ConstantGenerator::new(Marker)
///     }
/// }
/// ```
pub trait Rand: Sized {
    /// The generator type for this type.
    type Gen: Generator<Target = Self>;

    /// Create a random generator for this type.
    fn rand_generator(seed: u64) -> Self::Gen;
}

// Implementations for primitive types
use crate::generators::RandomGenerator;
use rand::distributions::Standard;

/// Macro to implement Rand for types with Standard distribution.
macro_rules! impl_rand_standard {
    ($($t:ty),*) => {
        $(
            impl Rand for $t {
                type Gen = RandomGenerator<$t>;
                
                fn rand_generator(seed: u64) -> Self::Gen {
                    RandomGenerator::with_seed(seed)
                }
            }
        )*
    };
}

// Implement for all standard primitive types
impl_rand_standard!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool, char
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Generator;

    #[test]
    fn test_rand_u32() {
        let gen = u32::rand_generator(42);
        let _value = gen.generate();
        // Just verify it compiles and generates
    }

    #[test]
    fn test_rand_bool() {
        let gen = bool::rand_generator(123);
        let _value = gen.generate();
    }
}
