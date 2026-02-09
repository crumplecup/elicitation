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
    
    #[test]
    fn test_rand_string() {
        let gen = String::rand_generator(42);
        let s = gen.generate();
        
        // Should generate lowercase letters
        assert!(s.len() < 32);
        for c in s.chars() {
            assert!(c.is_ascii_lowercase());
        }
    }
    
    #[test]
    fn test_rand_string_deterministic() {
        let gen1 = String::rand_generator(42);
        let gen2 = String::rand_generator(42);
        
        assert_eq!(gen1.generate(), gen2.generate());
    }
}

// String implementation - generates random alphanumeric strings
impl Rand for String {
    type Gen = crate::generators::MapGenerator<RandomGenerator<usize>, fn(usize) -> String>;
    
    fn rand_generator(seed: u64) -> Self::Gen {
        use rand::Rng;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        
        crate::generators::MapGenerator::new(
            RandomGenerator::with_seed(seed),
            move |length_seed: usize| {
                let mut rng = ChaCha8Rng::seed_from_u64(length_seed as u64);
                let length = rng.gen_range(0..32); // Default: 0-32 chars
                
                (0..length)
                    .map(|_| {
                        let c = rng.gen_range(b'a'..=b'z');
                        c as char
                    })
                    .collect()
            }
        )
    }
}

// Chrono datetime types
#[cfg(feature = "chrono")]
mod chrono_impls {
    use super::*;
    use chrono::{DateTime, Utc, NaiveDateTime};
    
    impl Rand for DateTime<Utc> {
        type Gen = crate::generators::MapGenerator<RandomGenerator<i64>, fn(i64) -> DateTime<Utc>>;
        
        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |timestamp: i64| {
                    DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
                }
            )
        }
    }
    
    impl Rand for NaiveDateTime {
        type Gen = crate::generators::MapGenerator<RandomGenerator<i64>, fn(i64) -> NaiveDateTime>;
        
        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |timestamp: i64| {
                    DateTime::<Utc>::from_timestamp(timestamp, 0)
                        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
                        .naive_utc()
                }
            )
        }
    }
}

#[cfg(feature = "chrono")]
pub use chrono_impls::*;

// Jiff datetime types
#[cfg(feature = "jiff")]
mod jiff_impls {
    use super::*;
    use jiff::Timestamp;
    
    impl Rand for Timestamp {
        type Gen = crate::generators::MapGenerator<RandomGenerator<i64>, fn(i64) -> Timestamp>;
        
        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |seconds: i64| {
                    Timestamp::from_second(seconds).unwrap_or_else(|_| Timestamp::from_second(0).unwrap())
                }
            )
        }
    }
}

#[cfg(feature = "jiff")]
pub use jiff_impls::*;

// Time datetime types
#[cfg(feature = "time")]
mod time_impls {
    use super::*;
    use time::OffsetDateTime;
    
    impl Rand for OffsetDateTime {
        type Gen = crate::generators::MapGenerator<RandomGenerator<i64>, fn(i64) -> OffsetDateTime>;
        
        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |timestamp: i64| {
                    OffsetDateTime::from_unix_timestamp(timestamp)
                        .unwrap_or_else(|_| OffsetDateTime::from_unix_timestamp(0).unwrap())
                }
            )
        }
    }
}

#[cfg(feature = "time")]
pub use time_impls::*;
