use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::{eof, map, peek};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::delimited;
use nom::{AsChar, Compare, IResult, InputTake, InputTakeAtPosition, Parser};

use crate::common::ast::SqlSpecialCharacter;
use crate::common::tokens::{
    ampersand, asterisk, circumflex, colon, comma, dollar_sign, double_quote, equals_operator,
    greater_than_operator, is_whitespace, left_brace, left_bracket, left_paren, less_than_operator,
    minus_sign, percent, period, plus_sign, question_mark, quote, right_brace, right_bracket,
    right_paren, semicolon, solidus, space, underscore, vertical_bar,
};
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
pub fn statement_terminator(i: &[u8]) -> IResult<&[u8], ()> {
    let (remaining_input, _) =
        delimited(whitespace0, alt((tag(";"), line_ending, eof)), whitespace0)(i)?;

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

/// Parses zero or more whitespace characters.
///
/// # Errors
/// This function should not fail, but as the parser can fail, this function let
/// the upstream decide what to do with this possible failure.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::whitespace0;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     whitespace0(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     21c"), Ok(("21c", " \t\n\r     ")));
/// assert_eq!(parser("Z21c"), Ok(("Z21c", "")));
/// assert_eq!(parser(""), Ok(("", "")));
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn whitespace0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !is_whitespace(c)
    })
}

/// Parses one or more whitespace characters.
///
/// # Errors
/// This function will fail if there's no whitespace characters identified.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::whitespace1;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     whitespace1(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     21c"), Ok(("21c", " \t\n\r     ")));
/// assert_eq!(parser(" Z21c"), Ok(("Z21c", " ")));
/// assert_eq!(
///     parser("Z21c"),
///     Err(Err::Error(Error::new("Z21c", ErrorKind::MultiSpace)))
/// );
/// assert_eq!(
///     parser(""),
///     Err(Err::Error(Error::new("", ErrorKind::MultiSpace)))
/// );
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn whitespace1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !is_whitespace(c)
        },
        ErrorKind::MultiSpace,
    )
}

/// A combinator that takes zero or more leading and trailing whitespaces,
/// returning the result of the received parser.
///
/// # Errors
/// If the received parser fails, this function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::delimited_ws0;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     delimited_ws0(tag_no_case("Potato"))(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     Potato    "), Ok(("", "Potato")));
/// assert_eq!(parser(" Potato"), Ok(("", "Potato")));
/// assert_eq!(parser("Potato "), Ok(("", "Potato")));
/// assert_eq!(parser("Potato"), Ok(("", "Potato")));
/// assert_eq!(
///     parser("(  Potato"),
///     Err(Err::Error(Error::new("(  Potato", ErrorKind::Tag)))
/// );
/// ```
pub fn delimited_ws0<T, O1, E: ParseError<T>, F>(mut first: F) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, _) = whitespace0(i)?;
        let (i, o1) = first.parse(i)?;
        let (i, _) = whitespace0(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes zero or more leading and whitespaces, returning the
/// result of the received parser.
///
/// # Errors
/// If the received parser fails, this function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::preceded_ws0;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     preceded_ws0(tag_no_case("Potato"))(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     Potato    "), Ok(("    ", "Potato")));
/// assert_eq!(parser(" Potato"), Ok(("", "Potato")));
/// assert_eq!(parser("Potato "), Ok((" ", "Potato")));
/// assert_eq!(parser("Potato"), Ok(("", "Potato")));
/// assert_eq!(
///     parser("(  Potato"),
///     Err(Err::Error(Error::new("(  Potato", ErrorKind::Tag)))
/// );
/// ```
pub fn preceded_ws0<T, O1, E: ParseError<T>, F>(mut parser: F) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, _) = whitespace0(i)?;
        let (i, o1) = parser.parse(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes zero or more leading and whitespaces, returning the
/// result of the received parser.
///
/// # Errors
/// If the received parser fails, this function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::terminated_ws0;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     terminated_ws0(tag_no_case("Potato"))(s)
/// }
///
/// assert_eq!(parser("Potato    "), Ok(("", "Potato")));
/// assert_eq!(parser("Potato  \n\r\t"), Ok(("", "Potato")));
/// assert_eq!(parser("Potato"), Ok(("", "Potato")));
/// assert_eq!(
///     parser(" Potato"),
///     Err(Err::Error(Error::new(" Potato", ErrorKind::Tag)))
/// );
/// ```
pub fn terminated_ws0<T, O1, E: ParseError<T>, F>(
    mut parser: F,
) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, o1) = parser.parse(i)?;
        let (i, _) = whitespace0(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes one or more leading and trailing whitespaces,
/// returning the result of the received parser.
///
/// # Errors
/// If the received parser fails, or there are no trailing and/or leading white
/// spaces, this function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::delimited_ws1;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     delimited_ws1(tag_no_case("Potato"))(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     Potato    "), Ok(("", "Potato")));
/// assert_eq!(
///     parser(" Potato"),
///     Err(Err::Error(Error::new("", ErrorKind::MultiSpace)))
/// );
/// assert_eq!(
///     parser("Potato "),
///     Err(Err::Error(Error::new("Potato ", ErrorKind::MultiSpace)))
/// );
/// assert_eq!(
///     parser("Potato"),
///     Err(Err::Error(Error::new("Potato", ErrorKind::MultiSpace)))
/// );
/// ```
pub fn delimited_ws1<T, O1, E: ParseError<T>, F>(mut first: F) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, _) = whitespace1(i)?;
        let (i, o1) = first.parse(i)?;
        let (i, _) = whitespace1(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes one or more leading whitespaces, returning the
/// result of the received parser.
///
/// # Errors
/// If the received parser fails, or there are no leading white spaces, this
/// function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::preceded_ws1;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     preceded_ws1(tag_no_case("Potato"))(s)
/// }
///
/// assert_eq!(parser(" \t\n\r     Potato    "), Ok(("    ", "Potato")));
/// assert_eq!(parser(" Potato"), Ok(("", "Potato")));
/// assert_eq!(
///     parser("Potato "),
///     Err(Err::Error(Error::new("Potato ", ErrorKind::MultiSpace)))
/// );
/// assert_eq!(
///     parser("Potato"),
///     Err(Err::Error(Error::new("Potato", ErrorKind::MultiSpace)))
/// );
/// ```
pub fn preceded_ws1<T, O1, E: ParseError<T>, F>(mut parser: F) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, _) = whitespace1(i)?;
        let (i, o1) = parser.parse(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes one or more trailing whitespaces, returning the
/// result of the received parser.
///
/// # Errors
/// If the received parser fails, or there are no trailing white spaces, this
/// function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::Err;
/// # use nom::IResult;
/// # use sql_helper::common::parsers::terminated_ws1;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///     terminated_ws1(tag_no_case("Potato"))(s)
/// }
/// assert_eq!(parser("Potato Name"), Ok(("Name", "Potato")));
/// assert_eq!(parser("Potato \t\r"), Ok(("", "Potato")));
/// assert_eq!(
///     parser("Potato"),
///     Err(Err::Error(Error::new("", ErrorKind::MultiSpace)))
/// );
/// ```
pub fn terminated_ws1<T, O1, E: ParseError<T>, F>(
    mut parser: F,
) -> impl FnMut(T) -> IResult<T, O1, E>
where
    F: Parser<T, O1, E>,
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, o1) = parser.parse(i)?;
        let (i, _) = whitespace1(i)?;
        Ok((i, o1))
    }
}

/// A combinator that takes zero or more leading and trailing whitespaces for a
/// paren delimited parser (or group of parsers), and return the parsers result.
///
/// # Errors
/// If the received parser fails, or there aren't any delimiter parens, this
/// function call will return an error.
///
/// # Examples
/// ```rust
/// # use nom::bytes::complete::tag_no_case;
/// # use nom::error::{Error, ErrorKind};
/// # use nom::IResult;
/// # use sql_helper::common::parsers::paren_delimited;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     paren_delimited(tag_no_case("Potato"))(s)
/// }
/// assert_eq!(parser("(Potato) Name"), Ok((" Name", "Potato")));
/// assert_eq!(parser("( Potato) Name"), Ok((" Name", "Potato")));
/// assert_eq!(parser("(Potato ) Name"), Ok((" Name", "Potato")));
/// assert_eq!(parser("(Potato) Name"), Ok((" Name", "Potato")));
/// ```
pub fn paren_delimited<T, O1, E, F>(mut parser: F) -> impl FnMut(T) -> IResult<T, O1, E>
where
    E: ParseError<T>,
    F: Parser<T, O1, E>,
    for<'a> T: InputTakeAtPosition + Clone + InputTake + Compare<&'a [u8]>,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |i: T| {
        let (i, _) = terminated_ws0(left_paren)(i)?;
        let (i, o1) = parser.parse(i)?;
        let (i, _) = preceded_ws0(right_paren)(i)?;
        Ok((i, o1))
    }
}

/// Parses a SQL special character.
///
/// # Errors
/// If the next character is not a special character, this function call will
/// fail.
pub fn sql_special_character(i: &[u8]) -> IResult<&[u8], SqlSpecialCharacter> {
    alt((
        map(space, |_| SqlSpecialCharacter::Space),
        map(double_quote, |_| SqlSpecialCharacter::DoubleQuote),
        map(percent, |_| SqlSpecialCharacter::Percent),
        map(ampersand, |_| SqlSpecialCharacter::Ampersand),
        map(quote, |_| SqlSpecialCharacter::Quote),
        map(left_paren, |_| SqlSpecialCharacter::LeftParen),
        map(right_paren, |_| SqlSpecialCharacter::RightParen),
        map(asterisk, |_| SqlSpecialCharacter::Asterisk),
        map(plus_sign, |_| SqlSpecialCharacter::PlusSign),
        map(comma, |_| SqlSpecialCharacter::Comma),
        map(minus_sign, |_| SqlSpecialCharacter::MinusSign),
        map(period, |_| SqlSpecialCharacter::Period),
        map(solidus, |_| SqlSpecialCharacter::Solidus),
        map(colon, |_| SqlSpecialCharacter::Colon),
        map(semicolon, |_| SqlSpecialCharacter::Semicolon),
        map(less_than_operator, |_| {
            SqlSpecialCharacter::LessThanOperator
        }),
        map(equals_operator, |_| SqlSpecialCharacter::EqualsOperator),
        map(greater_than_operator, |_| {
            SqlSpecialCharacter::GreaterThanOperator
        }),
        map(question_mark, |_| SqlSpecialCharacter::QuestionMark),
        map(left_bracket, |_| SqlSpecialCharacter::LeftBracket),
        alt((
            map(right_bracket, |_| SqlSpecialCharacter::RightBracket),
            map(circumflex, |_| SqlSpecialCharacter::Circumflex),
            map(underscore, |_| SqlSpecialCharacter::Underscore),
            map(vertical_bar, |_| SqlSpecialCharacter::VerticalBar),
            map(left_brace, |_| SqlSpecialCharacter::LeftBrace),
            map(right_brace, |_| SqlSpecialCharacter::RightBrace),
            map(dollar_sign, |_| SqlSpecialCharacter::DollarSign),
        )),
    ))(i)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

    use crate::common::parsers::sql_special_character;

    #[test_case(" "; "space")]
    #[test_case(r#"""#; "double quote")]
    #[test_case("%"; "percentage")]
    #[test_case("&"; "ampersand")]
    #[test_case("'"; "quote")]
    #[test_case("("; "left paren")]
    #[test_case(")"; "right paren")]
    #[test_case("*"; "asterisk")]
    #[test_case("+"; "plus sign")]
    #[test_case(","; "comma")]
    #[test_case("-"; "minus sign")]
    #[test_case("."; "period")]
    #[test_case("/"; "solidus")]
    #[test_case(":"; "colon")]
    #[test_case(";"; "semicolon")]
    #[test_case("<"; "less than operator")]
    #[test_case("="; "equals operator")]
    #[test_case(">"; "greater than operator")]
    #[test_case("?"; "question mark")]
    #[test_case("["; "left bracket")]
    #[test_case("]"; "right bracket")]
    #[test_case("^"; "circumflex")]
    #[test_case("_"; "underscore")]
    #[test_case("|"; "vertical bar")]
    #[test_case("{"; "left brace")]
    #[test_case("}"; "right brace")]
    #[test_case("$"; "dollar sign")]
    pub fn parse_special_characters(input: &str) {
        assert_str_eq!(
            input,
            sql_special_character(input.as_ref()).unwrap().1.to_string()
        );
    }
}
