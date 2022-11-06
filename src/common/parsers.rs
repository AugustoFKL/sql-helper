use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, line_ending, multispace0};
use nom::combinator::{eof, map, peek};
use nom::sequence::delimited;
use nom::IResult;

use crate::common::{is_sql_identifier, Ident, QuoteStyle};

/// Parse a terminator that ends a SQL statement, returning the remaining
/// string.
///
/// Since this function parses the end of a SQL statement, it is safe to assume
/// that if the result is not empty, it should be an additional SQL statement.
///
/// # Errors
/// If the input string does not contain an `'`, an line ending (\n, \r, \r\n),
/// or an EOF, this function returns an error.
pub fn parse_statement_terminator(i: &[u8]) -> IResult<&[u8], ()> {
    let (remaining_input, _) =
        delimited(multispace0, alt((tag(";"), line_ending, eof)), multispace0)(i)?;

    Ok((remaining_input, ()))
}

/// Parses a sql identifier.
///
/// Since this is a common structure, the resultant identifier is not
/// necessarily based on any dialects and, therefore, may not actually be valid
/// for a given database. The caller must validate it accordingly with its own
/// valid syntax.
///
/// OBS: ignores spaces before the identifier.
///
/// # Errors
/// If no possible identifier is found, or the identifier has not a valid quote
/// style, this method will return an error.
pub fn ident(i: &[u8]) -> IResult<&[u8], Ident> {
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
