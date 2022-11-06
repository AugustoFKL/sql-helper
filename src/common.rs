use nom::character::is_alphanumeric;

/// Parsers functions for generic structures as `Ident`, and generic concepts,
/// as the end of a statement.
pub mod parsers;

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
    /// Create a new identifier without a quote style.
    #[must_use]
    pub fn new(value: &[u8]) -> Self {
        Self::new_quoted(value, QuoteStyle::None)
    }

    /// Create a new identifier with the given value and quote style.
    #[must_use]
    pub fn new_quoted(value: &[u8], quote_style: QuoteStyle) -> Self {
        Self {
            value: String::from_utf8_lossy(value).to_string(),
            quote_style,
        }
    }

    /// Returns the string stored inside the identifier.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the quote style value.
    #[must_use]
    pub fn quote_style(&self) -> &QuoteStyle {
        &self.quote_style
    }
}

/// Returns true if the given character is a valid identifier character.
#[inline]
#[must_use]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'@'
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
        validate!(b"   Potato", Ident::new(b"Potato"));
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
