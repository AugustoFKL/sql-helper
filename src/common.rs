use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, char, space0};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::combinator::{cond, map, peek};
use nom::sequence::{delimited, preceded};
use nom::{IResult, InputTake};

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
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            quote_style: QuoteStyle::None,
        }
    }

    /// Create a new identifier with the given value and quote style.
    #[must_use]
    pub fn new_quoted(value: &str, quote_style: QuoteStyle) -> Self {
        Self {
            value: value.to_owned(),
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
pub fn parse_ident(i: &str) -> IResult<&str, Ident> {
    let double_quoted_parse = map(
        delimited(tag("\""), take_while1(is_sql_identifier), tag("\"")),
        |str| Ident::new_quoted(str, QuoteStyle::DoubleQuote),
    );

    // Here I guarantee that non-quoted identifiers must start with characters

    let unquoted = map(
        permutation((peek(alpha1), take_while1(is_sql_identifier))),
        |(_, str)| Ident::new(str),
    );

    alt((double_quoted_parse, unquoted))(i)
}

#[inline]
pub fn is_sql_identifier(chr: char) -> bool {
    is_alphanumeric(chr as u8) || chr == '_' || chr == '@'
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("name_1", Ident::new("name_1"))]
    #[test_case("name1", Ident::new("name1"))]
    #[test_case("giant name", Ident::new("giant"))]
    #[test_case(r#""name_1""#, Ident::new_quoted("name_1", QuoteStyle::DoubleQuote))]
    #[test_case(r#""1""#, Ident::new_quoted("1", QuoteStyle::DoubleQuote))]
    fn test_parse_ident(input: &str, expected: Ident) {
        let (_, parsed) = parse_ident(input).unwrap();
        assert_eq!(parsed, expected)
    }
}
