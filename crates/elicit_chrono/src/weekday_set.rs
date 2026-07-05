//! `WeekdaySet` — shadow for `chrono::WeekdaySet`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(elicitation::WeekdaySetWrap, as WeekdaySet, serde);
elicitation::elicit_newtype_traits!(WeekdaySet, elicitation::WeekdaySetWrap, [cmp]);

impl From<chrono::WeekdaySet> for WeekdaySet {
    fn from(s: chrono::WeekdaySet) -> Self {
        elicitation::WeekdaySetWrap::from(s).into()
    }
}

impl From<WeekdaySet> for chrono::WeekdaySet {
    fn from(s: WeekdaySet) -> Self {
        (*s.0).into_inner()
    }
}

impl Default for WeekdaySet {
    fn default() -> Self {
        Self::from(chrono::WeekdaySet::EMPTY)
    }
}

impl std::iter::FromIterator<crate::Weekday> for WeekdaySet {
    fn from_iter<I: IntoIterator<Item = crate::Weekday>>(iter: I) -> Self {
        let mut set = chrono::WeekdaySet::EMPTY;
        for w in iter {
            set.insert(*w.0);
        }
        Self::from(set)
    }
}

fn inner(s: &WeekdaySet) -> chrono::WeekdaySet {
    (*s.0).into_inner()
}

fn wrap(set: chrono::WeekdaySet) -> WeekdaySet {
    WeekdaySet::from(set)
}

impl WeekdaySet {
    /// Creates a `WeekdaySet` from an array of weekday names (e.g. `["Mon", "Fri"]`).
    #[instrument]
    pub fn from_array(days: Vec<crate::Weekday>) -> WeekdaySet {
        days.into_iter().collect()
    }

    /// Creates a `WeekdaySet` containing only the given weekday.
    #[instrument]
    pub fn single(day: crate::Weekday) -> WeekdaySet {
        let mut set = chrono::WeekdaySet::EMPTY;
        set.insert(*day.0);
        wrap(set)
    }
}

mod emit_impls {
    use super::WeekdaySet;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for WeekdaySet {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).into_inner();
            use chrono::Weekday::*;
            let variants: Vec<proc_macro2::Ident> = [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
                .iter()
                .filter(|&&d| inner.contains(d))
                .map(|d| {
                    let name = match d {
                        Mon => "Mon",
                        Tue => "Tue",
                        Wed => "Wed",
                        Thu => "Thu",
                        Fri => "Fri",
                        Sat => "Sat",
                        Sun => "Sun",
                    };
                    proc_macro2::Ident::new(name, proc_macro2::Span::call_site())
                })
                .collect();
            if variants.is_empty() {
                quote::quote! {
                    ::elicit_chrono::WeekdaySet::from(::chrono::WeekdaySet::EMPTY)
                }
            } else {
                quote::quote! {
                    ::elicit_chrono::WeekdaySet::from(
                        ::chrono::WeekdaySet::from_array([#(::chrono::Weekday::#variants),*])
                    )
                }
            }
        }

        fn type_tokens() -> TokenStream {
            quote::quote! { ::elicit_chrono::WeekdaySet }
        }
    }
}

impl elicitation::ElicitComplete for WeekdaySet {}

#[reflect_methods]
impl WeekdaySet {
    /// Returns the single day if this set has exactly one member.
    #[instrument(skip(self))]
    pub fn single_day(&self) -> Option<crate::Weekday> {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let s = inner(self);
        let found: Vec<chrono::Weekday> = [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
            .iter()
            .copied()
            .filter(|&d| s.contains(d))
            .collect();
        if found.len() == 1 {
            Some(found[0].into())
        } else {
            None
        }
    }

    /// Returns the first (lowest-indexed) weekday in the set, or `None` if empty.
    #[instrument(skip(self))]
    pub fn first(&self) -> Option<crate::Weekday> {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let s = inner(self);
        [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
            .iter()
            .copied()
            .find(|&d| s.contains(d))
            .map(Into::into)
    }

    /// Returns the last (highest-indexed) weekday in the set, or `None` if empty.
    #[instrument(skip(self))]
    pub fn last(&self) -> Option<crate::Weekday> {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let s = inner(self);
        [Sun, Sat, Fri, Thu, Wed, Tue, Mon]
            .iter()
            .copied()
            .find(|&d| s.contains(d))
            .map(Into::into)
    }

    /// Returns `true` if the set contains the given weekday.
    #[instrument(skip(self))]
    pub fn contains(&self, day: crate::Weekday) -> bool {
        inner(self).contains(*day.0)
    }

    /// Returns `true` if the set is empty.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        inner(self) == chrono::WeekdaySet::EMPTY
    }

    /// Returns the number of weekdays in the set (0–7).
    #[instrument(skip(self))]
    pub fn len(&self) -> u32 {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let s = inner(self);
        [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
            .iter()
            .filter(|&&d| s.contains(d))
            .count() as u32
    }

    /// Returns `true` if every day in `self` is also in `other`.
    #[instrument(skip(self))]
    pub fn is_subset(&self, other: WeekdaySet) -> bool {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let a = inner(self);
        let b = inner(&other);
        ![Mon, Tue, Wed, Thu, Fri, Sat, Sun]
            .iter()
            .any(|&d| a.contains(d) && !b.contains(d))
    }

    /// Returns a new set with the given weekday added.
    #[instrument(skip(self))]
    pub fn insert(&self, day: crate::Weekday) -> WeekdaySet {
        let mut s = inner(self);
        s.insert(*day.0);
        wrap(s)
    }

    /// Returns a new set with the given weekday removed.
    #[instrument(skip(self))]
    pub fn remove(&self, day: crate::Weekday) -> WeekdaySet {
        let mut s = inner(self);
        s.remove(*day.0);
        wrap(s)
    }

    /// Returns all days in this set as a list.
    #[instrument(skip(self))]
    pub fn iter(&self) -> Vec<crate::Weekday> {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let s = inner(self);
        [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
            .iter()
            .copied()
            .filter(|&d| s.contains(d))
            .map(Into::into)
            .collect()
    }

    /// Returns the union of two sets.
    #[instrument(skip(self))]
    pub fn union(&self, other: WeekdaySet) -> WeekdaySet {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let a = inner(self);
        let b = inner(&other);
        let mut result = chrono::WeekdaySet::EMPTY;
        for d in [Mon, Tue, Wed, Thu, Fri, Sat, Sun] {
            if a.contains(d) || b.contains(d) {
                result.insert(d);
            }
        }
        wrap(result)
    }

    /// Returns the intersection of two sets.
    #[instrument(skip(self))]
    pub fn intersection(&self, other: WeekdaySet) -> WeekdaySet {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let a = inner(self);
        let b = inner(&other);
        let mut result = chrono::WeekdaySet::EMPTY;
        for d in [Mon, Tue, Wed, Thu, Fri, Sat, Sun] {
            if a.contains(d) && b.contains(d) {
                result.insert(d);
            }
        }
        wrap(result)
    }

    /// Returns days in `self` that are not in `other`.
    #[instrument(skip(self))]
    pub fn difference(&self, other: WeekdaySet) -> WeekdaySet {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let a = inner(self);
        let b = inner(&other);
        let mut result = chrono::WeekdaySet::EMPTY;
        for d in [Mon, Tue, Wed, Thu, Fri, Sat, Sun] {
            if a.contains(d) && !b.contains(d) {
                result.insert(d);
            }
        }
        wrap(result)
    }

    /// Returns days present in exactly one of the two sets.
    #[instrument(skip(self))]
    pub fn symmetric_difference(&self, other: WeekdaySet) -> WeekdaySet {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let a = inner(self);
        let b = inner(&other);
        let mut result = chrono::WeekdaySet::EMPTY;
        for d in [Mon, Tue, Wed, Thu, Fri, Sat, Sun] {
            if a.contains(d) != b.contains(d) {
                result.insert(d);
            }
        }
        wrap(result)
    }
}
