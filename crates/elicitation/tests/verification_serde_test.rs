//! Serde bridge tests for constrained verification types.
//!
//! Verifies that:
//! - Valid values deserialize successfully
//! - Invalid values are rejected at the deserialization boundary
//! - Round-trip: serialize then deserialize preserves value
//! - JsonSchema contains the correct constraint fields

#![cfg(feature = "verification")]

use elicitation::verification::types::{
    F32Finite, F32NonNegative, F32Positive, F64Finite, F64NonNegative, F64Positive, I8NonNegative,
    I8NonZero, I8Positive, I16NonNegative, I16NonZero, I16Positive, StringNonEmpty, U8NonZero,
    U8Positive, U16NonZero, U16Positive,
};
use serde_json::json;

// ---------------------------------------------------------------------------
// Float tests
// ---------------------------------------------------------------------------

mod float_serde {
    use super::*;

    #[test]
    fn f64_positive_accepts_positive() {
        let v: F64Positive = serde_json::from_str("1.5").unwrap();
        assert_eq!(v.get(), 1.5_f64);
    }

    #[test]
    fn f64_positive_rejects_zero() {
        assert!(serde_json::from_str::<F64Positive>("0.0").is_err());
    }

    #[test]
    fn f64_positive_rejects_negative() {
        assert!(serde_json::from_str::<F64Positive>("-0.5").is_err());
    }

    #[test]
    fn f64_positive_round_trip() {
        let v = F64Positive::new(std::f64::consts::PI).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: F64Positive = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get(), v2.get());
    }

    #[test]
    fn f64_non_negative_accepts_zero() {
        let v: F64NonNegative = serde_json::from_str("0.0").unwrap();
        assert_eq!(v.get(), 0.0_f64);
    }

    #[test]
    fn f64_non_negative_rejects_negative() {
        assert!(serde_json::from_str::<F64NonNegative>("-0.001").is_err());
    }

    #[test]
    fn f64_finite_accepts_finite() {
        let _: F64Finite = serde_json::from_str("42.0").unwrap();
    }

    #[test]
    fn f64_finite_rejects_infinity() {
        // JSON doesn't have infinity literals, but we can test via API
        let inf = f64::INFINITY;
        assert!(F64Finite::new(inf).is_err());
    }

    #[test]
    fn f64_finite_rejects_nan() {
        assert!(F64Finite::new(f64::NAN).is_err());
    }

    #[test]
    fn f32_positive_accepts_positive() {
        let v: F32Positive = serde_json::from_str("2.5").unwrap();
        assert!(v.get() > 0.0_f32);
    }

    #[test]
    fn f32_positive_rejects_zero() {
        assert!(serde_json::from_str::<F32Positive>("0.0").is_err());
    }

    #[test]
    fn f32_non_negative_accepts_zero() {
        let v: F32NonNegative = serde_json::from_str("0.0").unwrap();
        assert_eq!(v.get(), 0.0_f32);
    }

    #[test]
    fn f32_finite_accepts_finite() {
        let _: F32Finite = serde_json::from_str("1.0").unwrap();
    }

    #[test]
    fn f64_positive_schema_has_exclusive_minimum() {
        let schema = schemars::schema_for!(F64Positive);
        let value = serde_json::to_value(&schema).unwrap();
        // schemars wraps schema in {"$schema":..., "$defs":..., ...}
        // the inner schema lives at root or under definitions
        let schema_str = value.to_string();
        assert!(
            schema_str.contains("exclusiveMinimum"),
            "schema missing exclusiveMinimum: {schema_str}"
        );
    }

    #[test]
    fn f64_non_negative_schema_has_minimum() {
        let schema = schemars::schema_for!(F64NonNegative);
        let value = serde_json::to_value(&schema).unwrap();
        let schema_str = value.to_string();
        assert!(
            schema_str.contains("\"minimum\""),
            "schema missing minimum: {schema_str}"
        );
    }
}

// ---------------------------------------------------------------------------
// Integer tests
// ---------------------------------------------------------------------------

mod integer_serde {
    use super::*;

    #[test]
    fn i8_positive_accepts_positive() {
        let v: I8Positive = serde_json::from_str("5").unwrap();
        assert_eq!(v.get(), 5_i8);
    }

    #[test]
    fn i8_positive_rejects_zero() {
        assert!(serde_json::from_str::<I8Positive>("0").is_err());
    }

    #[test]
    fn i8_positive_rejects_negative() {
        assert!(serde_json::from_str::<I8Positive>("-1").is_err());
    }

    #[test]
    fn i8_positive_round_trip() {
        let v = I8Positive::new(10).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: I8Positive = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get(), v2.get());
    }

    #[test]
    fn i8_non_negative_accepts_zero() {
        let v: I8NonNegative = serde_json::from_str("0").unwrap();
        assert_eq!(v.get(), 0_i8);
    }

    #[test]
    fn i8_non_negative_rejects_negative() {
        assert!(serde_json::from_str::<I8NonNegative>("-1").is_err());
    }

    #[test]
    fn i8_non_zero_accepts_negative() {
        let v: I8NonZero = serde_json::from_str("-3").unwrap();
        assert_eq!(v.get(), -3_i8);
    }

    #[test]
    fn i8_non_zero_rejects_zero() {
        assert!(serde_json::from_str::<I8NonZero>("0").is_err());
    }

    #[test]
    fn i16_positive_accepts_positive() {
        let v: I16Positive = serde_json::from_str("100").unwrap();
        assert_eq!(v.get(), 100_i16);
    }

    #[test]
    fn i16_positive_rejects_zero() {
        assert!(serde_json::from_str::<I16Positive>("0").is_err());
    }

    #[test]
    fn i16_non_negative_accepts_zero() {
        let _: I16NonNegative = serde_json::from_str("0").unwrap();
    }

    #[test]
    fn i16_non_zero_rejects_zero() {
        assert!(serde_json::from_str::<I16NonZero>("0").is_err());
    }

    #[test]
    fn u8_positive_accepts_positive() {
        let _: U8Positive = serde_json::from_str("1").unwrap();
    }

    #[test]
    fn u8_positive_rejects_zero() {
        assert!(serde_json::from_str::<U8Positive>("0").is_err());
    }

    #[test]
    fn u8_non_zero_rejects_zero() {
        assert!(serde_json::from_str::<U8NonZero>("0").is_err());
    }

    #[test]
    fn u16_positive_accepts_positive() {
        let _: U16Positive = serde_json::from_str("500").unwrap();
    }

    #[test]
    fn u16_non_zero_rejects_zero() {
        assert!(serde_json::from_str::<U16NonZero>("0").is_err());
    }

    #[test]
    fn i8_positive_schema_has_description() {
        let schema = schemars::schema_for!(I8Positive);
        let schema_str = serde_json::to_value(&schema).unwrap().to_string();
        assert!(
            schema_str.contains("Positive"),
            "schema missing description: {schema_str}"
        );
    }

    #[test]
    fn i16_positive_schema_has_minimum() {
        let schema = schemars::schema_for!(I16Positive);
        let schema_str = serde_json::to_value(&schema).unwrap().to_string();
        // i16 positive: minimum: 1 (using >= 1 rather than exclusiveMinimum: 0)
        assert!(
            schema_str.contains("\"minimum\":1"),
            "schema missing minimum constraint: {schema_str}"
        );
    }
}

// ---------------------------------------------------------------------------
// Integer range tests
// ---------------------------------------------------------------------------

#[cfg(feature = "verification")]
mod integer_range_serde {
    use elicitation::verification::types::{I8Range, I16Range, U8Range, U16Range};

    #[test]
    fn i8_range_accepts_in_bounds() {
        let v: I8Range<0, 100> = serde_json::from_str("50").unwrap();
        assert_eq!(v.get(), 50_i8);
    }

    #[test]
    fn i8_range_rejects_below_min() {
        assert!(serde_json::from_str::<I8Range<0, 100>>("-1").is_err());
    }

    #[test]
    fn i8_range_rejects_above_max() {
        assert!(serde_json::from_str::<I8Range<0, 100>>("101").is_err());
    }

    #[test]
    fn i8_range_accepts_boundary_values() {
        let _: I8Range<0, 100> = serde_json::from_str("0").unwrap();
        let _: I8Range<0, 100> = serde_json::from_str("100").unwrap();
    }

    #[test]
    fn i8_range_round_trip() {
        let v = I8Range::<0, 100>::new(42).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: I8Range<0, 100> = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get(), v2.get());
    }
    #[test]
    fn u8_range_rejects_above_max() {
        assert!(serde_json::from_str::<U8Range<1, 10>>("11").is_err());
    }

    #[test]
    fn i16_range_accepts_in_bounds() {
        let v: I16Range<-100, 100> = serde_json::from_str("-50").unwrap();
        assert_eq!(v.get(), -50_i16);
    }

    #[test]
    fn u16_range_rejects_below_min() {
        assert!(serde_json::from_str::<U16Range<5, 50>>("3").is_err());
    }

    #[test]
    fn i8_range_schema_has_min_max() {
        use elicitation::verification::types::I8Range;

        let schema = schemars::schema_for!(I8Range<0, 100>);
        let schema_str = serde_json::to_value(&schema).unwrap().to_string();
        assert!(
            schema_str.contains("minimum"),
            "schema missing minimum: {schema_str}"
        );
        assert!(
            schema_str.contains("maximum"),
            "schema missing maximum: {schema_str}"
        );
    }
}

// ---------------------------------------------------------------------------
// String tests
// ---------------------------------------------------------------------------

mod string_serde {
    use super::*;

    #[test]
    fn string_non_empty_accepts_non_empty() {
        let v: StringNonEmpty<4096> = serde_json::from_str("\"hello\"").unwrap();
        assert_eq!(v.get(), "hello");
    }

    #[test]
    fn string_non_empty_rejects_empty() {
        assert!(serde_json::from_str::<StringNonEmpty<4096>>("\"\"").is_err());
    }

    #[test]
    fn string_non_empty_round_trip() {
        let v = StringNonEmpty::<4096>::new("test".to_string()).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: StringNonEmpty<4096> = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get(), v2.get());
    }

    #[test]
    fn string_non_empty_schema_has_min_length() {
        let schema = schemars::schema_for!(StringNonEmpty<4096>);
        let schema_str = serde_json::to_value(&schema).unwrap().to_string();
        assert!(
            schema_str.contains("minLength"),
            "schema missing minLength: {schema_str}"
        );
    }

    #[test]
    fn string_serializes_as_plain_string_not_object() {
        let v = StringNonEmpty::<4096>::new("world".to_string()).unwrap();
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"world\"");
    }
}

// ---------------------------------------------------------------------------
// URL tests (feature-gated)
// ---------------------------------------------------------------------------

#[cfg(feature = "url")]
mod url_serde {
    use elicitation::verification::types::{UrlHttp, UrlHttps, UrlValid, UrlWithHost};

    #[test]
    fn url_https_accepts_https() {
        let v: UrlHttps = serde_json::from_str("\"https://example.com\"").unwrap();
        assert_eq!(v.get().as_str(), "https://example.com/");
    }

    #[test]
    fn url_https_rejects_http() {
        assert!(serde_json::from_str::<UrlHttps>("\"http://example.com\"").is_err());
    }

    #[test]
    fn url_https_rejects_invalid() {
        assert!(serde_json::from_str::<UrlHttps>("\"not a url\"").is_err());
    }

    #[test]
    fn url_https_round_trip() {
        let v = UrlHttps::new("https://example.com/path").unwrap();
        let s = serde_json::to_string(&v).unwrap();
        let v2: UrlHttps = serde_json::from_str(&s).unwrap();
        assert_eq!(v.get().as_str(), v2.get().as_str());
    }

    #[test]
    fn url_http_accepts_http() {
        let _: UrlHttp = serde_json::from_str("\"http://example.com\"").unwrap();
    }

    #[test]
    fn url_http_rejects_https() {
        assert!(serde_json::from_str::<UrlHttp>("\"https://example.com\"").is_err());
    }

    #[test]
    fn url_valid_accepts_any_valid_url() {
        let _: UrlValid = serde_json::from_str("\"ftp://files.example.com\"").unwrap();
    }

    #[test]
    fn url_valid_rejects_invalid() {
        assert!(serde_json::from_str::<UrlValid>("\"not-a-url\"").is_err());
    }

    #[test]
    fn url_with_host_accepts_url_with_host() {
        let _: UrlWithHost = serde_json::from_str("\"https://example.com\"").unwrap();
    }

    #[test]
    fn url_serializes_as_plain_string() {
        let v = UrlHttps::new("https://example.com/").unwrap();
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "\"https://example.com/\"");
    }

    #[test]
    fn url_https_schema_has_format_uri() {
        let schema = schemars::schema_for!(UrlHttps);
        let schema_str = serde_json::to_value(&schema).unwrap().to_string();
        assert!(
            schema_str.contains("uri"),
            "schema missing uri format: {schema_str}"
        );
    }
}

// ---------------------------------------------------------------------------
// Cross-type: JSON object deserialization (params struct simulation)
// ---------------------------------------------------------------------------

mod params_struct {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TimeoutParams {
        timeout_secs: F64Positive,
        max_retries: I8NonNegative,
        label: StringNonEmpty<4096>,
    }

    #[test]
    fn valid_params_deserialize() {
        let json = json!({
            "timeout_secs": 30.0,
            "max_retries": 3,
            "label": "test-run"
        });
        let p: TimeoutParams = serde_json::from_value(json).unwrap();
        assert_eq!(p.timeout_secs.get(), 30.0_f64);
        assert_eq!(p.max_retries.get(), 3_i8);
        assert_eq!(p.label.get(), "test-run");
    }

    #[test]
    fn invalid_timeout_rejected_in_params() {
        let json = json!({
            "timeout_secs": -1.0,
            "max_retries": 0,
            "label": "test"
        });
        assert!(serde_json::from_value::<TimeoutParams>(json).is_err());
    }

    #[test]
    fn empty_label_rejected_in_params() {
        let json = json!({
            "timeout_secs": 5.0,
            "max_retries": 0,
            "label": ""
        });
        assert!(serde_json::from_value::<TimeoutParams>(json).is_err());
    }
}
