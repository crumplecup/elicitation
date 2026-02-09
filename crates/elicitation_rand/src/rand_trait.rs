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
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
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
            },
        )
    }
}

// Chrono datetime types
#[cfg(feature = "chrono")]
mod chrono_impls {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};

    impl Rand for DateTime<Utc> {
        type Gen = crate::generators::MapGenerator<RandomGenerator<i64>, fn(i64) -> DateTime<Utc>>;

        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |timestamp: i64| {
                    DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
                },
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
                },
            )
        }
    }
}

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
                    Timestamp::from_second(seconds)
                        .unwrap_or_else(|_| Timestamp::from_second(0).unwrap())
                },
            )
        }
    }
}

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
                },
            )
        }
    }
}

// UUID types
#[cfg(feature = "uuid")]
mod uuid_impls {
    use super::*;
    use uuid::Uuid;

    impl Rand for Uuid {
        type Gen = crate::generators::MapGenerator<RandomGenerator<u128>, fn(u128) -> Uuid>;

        fn rand_generator(seed: u64) -> Self::Gen {
            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |random: u128| Uuid::from_u128(random),
            )
        }
    }
}

// URL types
#[cfg(feature = "url")]
mod url_impls {
    use super::*;
    use url::Url;

    impl Rand for Url {
        type Gen = crate::generators::MapGenerator<RandomGenerator<u64>, fn(u64) -> Url>;

        fn rand_generator(seed: u64) -> Self::Gen {
            use rand::Rng;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |url_seed: u64| {
                    let mut rng = ChaCha8Rng::seed_from_u64(url_seed);

                    // Generate random valid URL
                    let schemes = ["http", "https", "ftp"];
                    let scheme = schemes[rng.gen_range(0..schemes.len())];

                    let hosts = ["example.com", "test.org", "demo.net", "api.io"];
                    let host = hosts[rng.gen_range(0..hosts.len())];

                    let port = if rng.gen_bool(0.3) {
                        format!(":{}", rng.gen_range(8000..9000))
                    } else {
                        String::new()
                    };

                    let path_len = rng.gen_range(0..4);
                    let path = (0..path_len)
                        .map(|_| {
                            let segment_len = rng.gen_range(3..8);
                            (0..segment_len)
                                .map(|_| {
                                    let c = rng.gen_range(b'a'..=b'z');
                                    c as char
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<_>>()
                        .join("/");

                    let path_str = if path.is_empty() {
                        String::new()
                    } else {
                        format!("/{}", path)
                    };

                    let url_str = format!("{}://{}{}{}", scheme, host, port, path_str);

                    Url::parse(&url_str)
                        .unwrap_or_else(|_| Url::parse("http://example.com").unwrap())
                },
            )
        }
    }
}

// PathBuf (always available - stdlib)
mod pathbuf_impls {
    use super::*;
    use std::path::PathBuf;

    impl Rand for PathBuf {
        type Gen = crate::generators::MapGenerator<RandomGenerator<u64>, fn(u64) -> PathBuf>;

        fn rand_generator(seed: u64) -> Self::Gen {
            use rand::Rng;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            crate::generators::MapGenerator::new(
                RandomGenerator::with_seed(seed),
                |path_seed: u64| {
                    let mut rng = ChaCha8Rng::seed_from_u64(path_seed);

                    // Generate random path with 1-4 components
                    let depth = rng.gen_range(1..=4);
                    let components: Vec<String> = (0..depth)
                        .map(|_| {
                            let len = rng.gen_range(4..10);
                            (0..len)
                                .map(|_| {
                                    let c = rng.gen_range(b'a'..=b'z');
                                    c as char
                                })
                                .collect()
                        })
                        .collect();

                    // Unix-style path (works on all platforms)
                    let path_str = format!("/{}", components.join("/"));
                    PathBuf::from(path_str)
                },
            )
        }
    }
}
