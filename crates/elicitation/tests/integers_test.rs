//! Tests for integer type elicitation.

use elicitation::{Elicitation, Prompt};

macro_rules! test_integer_type {
    ($t:ty, $name_prompt:ident, $name_bounds:ident) => {
        #[test]
        fn $name_prompt() {
            let prompt = <$t>::prompt();
            assert!(prompt.is_some());
            let text = prompt.unwrap();
            assert!(text.contains(stringify!($t)));
        }

        #[test]
        fn $name_bounds() {
            fn assert_send<T: Send>() {}
            fn assert_sync<T: Sync>() {}
            fn assert_elicitation<T: Elicitation>() {}

            assert_send::<$t>();
            assert_sync::<$t>();
            assert_elicitation::<$t>();
        }
    };
}

// Signed integers
test_integer_type!(i8, test_i8_has_prompt, test_i8_trait_bounds);
test_integer_type!(i16, test_i16_has_prompt, test_i16_trait_bounds);
test_integer_type!(i32, test_i32_has_prompt, test_i32_trait_bounds);
test_integer_type!(i64, test_i64_has_prompt, test_i64_trait_bounds);
test_integer_type!(i128, test_i128_has_prompt, test_i128_trait_bounds);
test_integer_type!(isize, test_isize_has_prompt, test_isize_trait_bounds);

// Unsigned integers
test_integer_type!(u8, test_u8_has_prompt, test_u8_trait_bounds);
test_integer_type!(u16, test_u16_has_prompt, test_u16_trait_bounds);
test_integer_type!(u32, test_u32_has_prompt, test_u32_trait_bounds);
test_integer_type!(u64, test_u64_has_prompt, test_u64_trait_bounds);
test_integer_type!(u128, test_u128_has_prompt, test_u128_trait_bounds);
test_integer_type!(usize, test_usize_has_prompt, test_usize_trait_bounds);
