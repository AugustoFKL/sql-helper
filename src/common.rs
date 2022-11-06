use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::alpha1;
use nom::character::is_alphanumeric;
use nom::combinator::{map, peek};
use nom::sequence::delimited;
use nom::IResult;

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

/// Parses a sql identifier.
///
/// Since this is a common structure, the resultant identifier is not
/// necessarily based on any dialects and, therefore, may not actually be valid
/// for a given database. The caller must validate it accordingly with its own
/// valid syntax.
///
/// # Errors
/// If no possible identifier is found, or the identifier has not a valid quote
/// style, this method will return an error.
pub fn parse_ident(i: &[u8]) -> IResult<&[u8], Ident> {
    let double_quoted_parse = map(
        delimited(tag("\""), take_while1(is_sql_identifier), tag("\"")),
        |bytes| Ident::new_quoted(bytes, QuoteStyle::DoubleQuote),
    );

    // Here I guarantee that non-quoted identifiers must start with characters

    let unquoted = map(
        permutation((peek(alpha1), take_while1(is_sql_identifier))),
        |(_, bytes)| Ident::new(bytes),
    );

    alt((double_quoted_parse, unquoted))(i)
}

/// Returns true if the given character is a valid identifier character.
#[inline]
#[must_use]
pub fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8 || chr == '@' as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ident() {
        macro_rules! validate {
            ($input:expr, $expected:expr) => {
                let (_, parsed) = parse_ident($input).unwrap();
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
    #[should_panic]
    fn test_parse_invalid_ident() {
        parse_ident(b"1").unwrap();
    }
}
