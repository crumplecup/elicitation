//! SQL syntax tokenizer and Catppuccin Mocha colour constants.
//!
//! Provides a single canonical [`sql_tokens`] function consumed by all
//! renderers (egui, ratatui, leptos) so that the keyword set and palette
//! never drift apart.

/// Catppuccin Mocha RGB palette — raw `(r, g, b)` tuples so every renderer
/// can convert to its own colour type without coupling to any GUI crate.
pub mod mocha {
    /// Keyword colour (Mauve).
    pub const KW: (u8, u8, u8) = (0xcb, 0xa6, 0xf7);
    /// String literal / quoted identifier (Green).
    pub const STR: (u8, u8, u8) = (0xa6, 0xe3, 0xa1);
    /// Comment (Overlay1).
    pub const COMMENT: (u8, u8, u8) = (0x6c, 0x70, 0x86);
    /// Numeric literal (Peach).
    pub const NUM: (u8, u8, u8) = (0xfa, 0xb3, 0x87);
    /// Plain identifier / operator text (Text).
    pub const TEXT: (u8, u8, u8) = (0xcd, 0xd6, 0xf4);
    /// Base background.
    pub const BASE: (u8, u8, u8) = (0x1e, 0x1e, 0x2e);
    /// Surface0 — panels, cards.
    pub const SURFACE0: (u8, u8, u8) = (0x31, 0x32, 0x44);
    /// Surface1 — raised elements.
    pub const SURFACE1: (u8, u8, u8) = (0x45, 0x47, 0x5a);
    /// Subtext0 — muted / secondary text.
    pub const SUBTEXT0: (u8, u8, u8) = (0xa6, 0xad, 0xc8);
    /// Blue accent (Sapphire).
    pub const BLUE: (u8, u8, u8) = (0x89, 0xb4, 0xfa);
}

/// Classification of a SQL token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlTokenKind {
    /// SQL keyword (`SELECT`, `FROM`, `WHERE`, …).
    Keyword,
    /// Single-quoted string or double-quoted identifier.
    StringLiteral,
    /// `--` line comment or `/* */` block comment.
    Comment,
    /// Numeric literal.
    Number,
    /// Identifiers, operators, punctuation.
    Plain,
}

/// A slice of SQL source text with its [`SqlTokenKind`].
///
/// The `text` field borrows directly from the input — no allocation.
#[derive(Debug, Clone, Copy)]
pub struct SqlToken<'a> {
    /// Raw source slice.  Never empty.
    pub text: &'a str,
    /// Token classification.
    pub kind: SqlTokenKind,
}

fn is_sql_keyword(word: &str) -> bool {
    matches!(
        word,
        "SELECT"
            | "FROM"
            | "WHERE"
            | "JOIN"
            | "LEFT"
            | "RIGHT"
            | "INNER"
            | "OUTER"
            | "FULL"
            | "CROSS"
            | "ON"
            | "GROUP"
            | "BY"
            | "ORDER"
            | "HAVING"
            | "LIMIT"
            | "OFFSET"
            | "INSERT"
            | "INTO"
            | "VALUES"
            | "UPDATE"
            | "SET"
            | "DELETE"
            | "CREATE"
            | "TABLE"
            | "VIEW"
            | "INDEX"
            | "DROP"
            | "ALTER"
            | "ADD"
            | "COLUMN"
            | "CONSTRAINT"
            | "PRIMARY"
            | "KEY"
            | "FOREIGN"
            | "REFERENCES"
            | "UNIQUE"
            | "NOT"
            | "NULL"
            | "DEFAULT"
            | "AND"
            | "OR"
            | "IN"
            | "IS"
            | "LIKE"
            | "ILIKE"
            | "BETWEEN"
            | "EXISTS"
            | "CASE"
            | "WHEN"
            | "THEN"
            | "ELSE"
            | "END"
            | "AS"
            | "DISTINCT"
            | "ALL"
            | "UNION"
            | "INTERSECT"
            | "EXCEPT"
            | "WITH"
            | "RETURNING"
            | "BEGIN"
            | "COMMIT"
            | "ROLLBACK"
            | "TRANSACTION"
            | "EXPLAIN"
            | "ANALYZE"
            | "TRUNCATE"
            | "GRANT"
            | "REVOKE"
            | "SCHEMA"
            | "DATABASE"
            | "SEQUENCE"
            | "FUNCTION"
            | "PROCEDURE"
            | "TRIGGER"
            | "EXTENSION"
    )
}

/// Tokenise `input` into [`SqlToken`]s, classifying each slice.
///
/// Handles (in priority order):
/// * `/* … */` block comments (may span lines)
/// * `-- …` line comments (terminated by `\n`)
/// * `'…'` single-quoted strings (backslash escape)
/// * `"…"` double-quoted identifiers (backslash escape)
/// * numeric literals (`[0-9][0-9.]*`)
/// * identifiers / SQL keywords
/// * everything else as [`SqlTokenKind::Plain`]
pub fn sql_tokens(input: &str) -> Vec<SqlToken<'_>> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut seg_start = 0;

    macro_rules! flush {
        ($end:expr) => {
            if seg_start < $end {
                tokens.push(SqlToken {
                    text: &input[seg_start..$end],
                    kind: SqlTokenKind::Plain,
                });
            }
        };
    }

    while i < len {
        // Block comment  /* … */
        if i + 1 < len && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            flush!(i);
            let start = i;
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(len);
            tokens.push(SqlToken {
                text: &input[start..i],
                kind: SqlTokenKind::Comment,
            });
            seg_start = i;
            continue;
        }
        // Line comment  -- …
        if i + 1 < len && bytes[i] == b'-' && bytes[i + 1] == b'-' {
            flush!(i);
            let start = i;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            tokens.push(SqlToken {
                text: &input[start..i],
                kind: SqlTokenKind::Comment,
            });
            seg_start = i;
            continue;
        }
        // Single-quoted string  '…'
        if bytes[i] == b'\'' {
            flush!(i);
            let start = i;
            i += 1;
            while i < len {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            tokens.push(SqlToken {
                text: &input[start..i],
                kind: SqlTokenKind::StringLiteral,
            });
            seg_start = i;
            continue;
        }
        // Double-quoted identifier  "…"
        if bytes[i] == b'"' {
            flush!(i);
            let start = i;
            i += 1;
            while i < len {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            tokens.push(SqlToken {
                text: &input[start..i],
                kind: SqlTokenKind::StringLiteral,
            });
            seg_start = i;
            continue;
        }
        // Numeric literal
        if bytes[i].is_ascii_digit() {
            flush!(i);
            let start = i;
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            tokens.push(SqlToken {
                text: &input[start..i],
                kind: SqlTokenKind::Number,
            });
            seg_start = i;
            continue;
        }
        // Identifier / keyword
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' {
            flush!(i);
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &input[start..i];
            let kind = if is_sql_keyword(&word.to_ascii_uppercase()) {
                SqlTokenKind::Keyword
            } else {
                SqlTokenKind::Plain
            };
            tokens.push(SqlToken { text: word, kind });
            seg_start = i;
            continue;
        }
        i += 1;
    }
    flush!(len);
    tokens
}
