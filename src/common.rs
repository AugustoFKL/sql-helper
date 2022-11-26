use std::fmt;

use nom::character::is_alphanumeric;

pub mod ast;
pub mod parsers;
pub mod tokens;

/// SQL identifiers [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#identifier
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Ident {
    /// Identifier internal unquoted value.
    value: String,
    /// Identifier quote style.
    quote_style: QuoteStyle,
}

/// Possible quote styles for identifiers for all dialects.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum QuoteStyle {
    /// Nonexistent quote style.
    None,
    /// Double quote style (").
    DoubleQuote,
}

impl Ident {
    #[must_use]
    pub fn new(value: &[u8]) -> Self {
        Self::new_quoted(value, QuoteStyle::None)
    }

    #[must_use]
    pub fn new_quoted(value: &[u8], quote_style: QuoteStyle) -> Self {
        Self {
            value: String::from_utf8_lossy(value).to_string(),
            quote_style,
        }
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub const fn quote_style(&self) -> &QuoteStyle {
        &self.quote_style
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quote_style() {
            QuoteStyle::None => {
                write!(f, "{}", self.value)
            }
            QuoteStyle::DoubleQuote => {
                write!(f, "\"{}\"", self.value)
            }
        }
    }
}

#[must_use]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_'
}

#[must_use]
pub fn display_comma_separated(list: &[impl ToString]) -> String {
    list.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
pub fn if_some_string_preceded_by(opt_item: Option<impl ToString>, preceded_by: &str) -> String {
    opt_item.map_or_else(String::default, |item| {
        format!("{preceded_by}{}", item.to_string())
    })
}

#[cfg(test)]
mod tests {
    use crate::common::parsers::ident;

    use super::*;

    #[test]
    fn test_parse_ident() {
        macro_rules! validate {
            ($input:expr, $expected:expr) => {
                let (_, parsed) = ident($input).unwrap();
                assert_eq!(parsed, $expected);
            };
        }

        validate!(b"name_1", Ident::new(b"name_1"));
        validate!(b"name1", Ident::new(b"name1"));
        validate!(b"spaced name", Ident::new(b"spaced"));
        validate!(
            b"\"name_1\"",
            Ident::new_quoted(b"name_1", QuoteStyle::DoubleQuote)
        );
        validate!(b"\"1\"", Ident::new_quoted(b"1", QuoteStyle::DoubleQuote));
    }

    #[test]
    fn test_parse_invalid_ident() {
        let result = ident(b"1");
        assert!(result.is_err());
    }
}
