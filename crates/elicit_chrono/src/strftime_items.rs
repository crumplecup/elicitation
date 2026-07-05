//! `Item` and `StrftimeItems` — shadows for `chrono::format::Item` and
//! `chrono::format::StrftimeItems`.

use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Item ──────────────────────────────────────────────────────────────────────

/// Shadow for [`chrono::format::Item`].
///
/// Stores the item as an owned [`elicitation::OwnedItem`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Item(pub elicitation::OwnedItem);

impl From<elicitation::OwnedItem> for Item {
    fn from(o: elicitation::OwnedItem) -> Self {
        Self(o)
    }
}

impl<'a> From<chrono::format::Item<'a>> for Item {
    fn from(item: chrono::format::Item<'a>) -> Self {
        Self(elicitation::OwnedItem::from(item))
    }
}

#[reflect_methods]
impl Item {
    /// Returns an owned copy of this item (no-op since `Item` is already owned).
    #[instrument(skip(self))]
    pub fn to_owned(&self) -> Item {
        self.clone()
    }
}

// ── StrftimeItems ─────────────────────────────────────────────────────────────

/// Shadow for [`chrono::format::StrftimeItems`].
///
/// Parses and stores a strftime format string as a sequence of owned [`Item`]s.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StrftimeItems {
    /// The parsed format items.
    pub items: Vec<Item>,
    /// Current cursor position for `next()`.
    #[serde(skip)]
    cursor: usize,
}

impl StrftimeItems {
    fn from_iter_internal(iter: chrono::format::StrftimeItems<'_>) -> Self {
        let items = iter.map(Item::from).collect();
        Self { items, cursor: 0 }
    }
}

impl StrftimeItems {
    /// Parses a strftime-format string into a sequence of items (strict: errors on unknown specs).
    #[instrument]
    pub fn new(fmt: String) -> StrftimeItems {
        Self::from_iter_internal(chrono::format::StrftimeItems::new(&fmt))
    }

    /// Parses a strftime-format string leniently (unknown specs become literals).
    #[instrument]
    pub fn new_lenient(fmt: String) -> StrftimeItems {
        Self::from_iter_internal(chrono::format::StrftimeItems::new_lenient(&fmt))
    }

    /// Parses a strftime-format string strictly, returning an error if any item is invalid.
    #[instrument]
    pub fn parse(fmt: String) -> Result<StrftimeItems, String> {
        let items = chrono::format::StrftimeItems::new(&fmt)
            .parse()
            .map_err(|e: chrono::ParseError| e.to_string())?;
        let owned: Vec<Item> = items.into_iter().map(Item::from).collect();
        Ok(Self {
            items: owned,
            cursor: 0,
        })
    }

    /// Like `parse`, but explicitly returns only `'static` items with no borrowed slices.
    #[instrument]
    pub fn parse_to_owned(fmt: String) -> Result<StrftimeItems, String> {
        let items = chrono::format::StrftimeItems::new(&fmt)
            .parse_to_owned()
            .map_err(|e: chrono::ParseError| e.to_string())?;
        let owned: Vec<Item> = items.into_iter().map(Item::from).collect();
        Ok(Self {
            items: owned,
            cursor: 0,
        })
    }

    /// Returns the next item from the format sequence, advancing the internal cursor.
    ///
    /// Returns `None` when all items have been consumed.
    #[instrument(skip(self))]
    pub fn next_item(&mut self) -> Option<Item> {
        if self.cursor < self.items.len() {
            let item = self.items[self.cursor].clone();
            self.cursor += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl Iterator for StrftimeItems {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        self.next_item()
    }
}
