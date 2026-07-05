//! `Weekday` — shadow for `chrono::Weekday`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::Weekday, as Weekday, serde);
elicitation::elicit_newtype_traits!(Weekday, chrono::Weekday, [eq_hash]);

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Weekday {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chrono::Weekday as W;
        let w = match s.to_ascii_lowercase().as_str() {
            "mon" | "monday" => W::Mon,
            "tue" | "tuesday" => W::Tue,
            "wed" | "wednesday" => W::Wed,
            "thu" | "thursday" => W::Thu,
            "fri" | "friday" => W::Fri,
            "sat" | "saturday" => W::Sat,
            "sun" | "sunday" => W::Sun,
            _ => return Err(format!("unknown weekday: {s}")),
        };
        Ok(w.into())
    }
}

impl TryFrom<u8> for Weekday {
    type Error = String;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Weekday::from_u64(n as u64).ok_or_else(|| format!("weekday index out of range: {n}"))
    }
}

#[reflect_methods]
impl Weekday {
    /// Returns the number of days from Monday (0 = Monday, 6 = Sunday).
    #[instrument(skip(self))]
    pub fn num_days_from_monday(&self) -> u32 {
        self.0.num_days_from_monday()
    }

    /// Returns the number of days from Sunday (0 = Sunday, 6 = Saturday).
    #[instrument(skip(self))]
    pub fn num_days_from_sunday(&self) -> u32 {
        self.0.num_days_from_sunday()
    }

    /// Returns the 1-indexed number from Monday (1 = Monday, 7 = Sunday).
    #[instrument(skip(self))]
    pub fn number_from_monday(&self) -> u32 {
        self.0.num_days_from_monday() + 1
    }

    /// Returns the 1-indexed number from Sunday (1 = Sunday, 7 = Saturday).
    #[instrument(skip(self))]
    pub fn number_from_sunday(&self) -> u32 {
        self.0.num_days_from_sunday() + 1
    }

    /// Returns the next weekday (Sunday wraps to Monday).
    #[instrument(skip(self))]
    pub fn succ(&self) -> Weekday {
        self.0.succ().into()
    }

    /// Returns the previous weekday (Monday wraps to Sunday).
    #[instrument(skip(self))]
    pub fn pred(&self) -> Weekday {
        self.0.pred().into()
    }

    /// Returns the number of days since the given weekday (0–6).
    #[instrument(skip(self))]
    pub fn days_since(&self, other: Weekday) -> u32 {
        self.0.days_since(*other.0)
    }
}

impl Weekday {
    /// Converts a Monday-indexed integer (0–6) to a `Weekday`, or `None` if out of range.
    #[instrument]
    pub fn from_i64(n: i64) -> Option<Weekday> {
        if !(0..=6).contains(&n) {
            return None;
        }
        Weekday::from_u64(n as u64)
    }

    /// Converts a Monday-indexed integer (0–6) to a `Weekday`, or `None` if out of range.
    #[instrument]
    pub fn from_u64(n: u64) -> Option<Weekday> {
        use chrono::Weekday as W;
        let w = match n {
            0 => W::Mon,
            1 => W::Tue,
            2 => W::Wed,
            3 => W::Thu,
            4 => W::Fri,
            5 => W::Sat,
            6 => W::Sun,
            _ => return None,
        };
        Some(w.into())
    }
}

mod emit_impls {
    use super::Weekday;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Weekday {
        fn to_code_literal(&self) -> TokenStream {
            let n = self.0.num_days_from_monday();
            quote::quote! {
                ::elicit_chrono::Weekday::from_u64(#n as u64)
                    .expect("valid weekday index")
            }
        }

        fn type_tokens() -> TokenStream {
            quote::quote! { ::elicit_chrono::Weekday }
        }
    }
}

impl elicitation::ElicitComplete for Weekday {}
