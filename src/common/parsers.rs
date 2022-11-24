use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, multispace0};
use nom::combinator::{eof, map, peek};
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::common::ast::SqlSpecialCharacter;
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

/// Parses a u32 that is delimited by parentheses with one or more spaces in
/// both sides.
///
/// # Errors
/// If the parser fails, will return an error.
pub fn delimited_u32(i: &[u8]) -> IResult<&[u8], u32> {
    delimited(
        tuple((tag("("), multispace0)),
        complete::u32,
        tuple((multispace0, tag(")"))),
    )(i)
}

/// Parses a space character.
///
/// # Errors
/// If the next character is not a space, this function call will fail.
pub fn space(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(" ")(i)
}

/// Parses a double quote character.
///
/// # Errors
/// If the next character is not a double quote, this function call will fail.
pub fn double_quote(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("\"")(i)
}

/// Parses a percent character.
///
/// # Errors
/// If the next character is not a percent, this function call will fail.
pub fn percent(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%")(i)
}

/// Parses an ampersand character.
///
/// # Errors
/// If the next character is not an ampersand, this function call will fail.
pub fn ampersand(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("&")(i)
}

/// Parses a quote character.
///
/// # Errors
/// If the next character is not a quote, this function call will fail.
pub fn quote(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("'")(i)
}

/// Parses a left paren character.
///
/// # Errors
/// If the next character is not a left paren, this function call will fail.
pub fn left_paren(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("(")(i)
}

/// Parses a right paren character.
///
/// # Errors
/// If the next character is not a right paren, this function call will fail.
pub fn right_paren(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(")")(i)
}

/// Parses an asterisk character.
///
/// # Errors
/// If the next character is not an asterisk, this function call will fail.
pub fn asterisk(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("*")(i)
}

/// Parses a plus sign character.
///
/// # Errors
/// If the next character is not a plus sign, this function call will fail.
pub fn plus_sign(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("+")(i)
}

/// Parses a comma character.
///
/// # Errors
/// If the next character is not a comma, this function call will fail.
pub fn comma(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(",")(i)
}

/// Parses a minus sign character.
///
/// # Errors
/// If the next character is not a minus sign, this function call will fail.
pub fn minus_sign(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("-")(i)
}

/// Parses a period character.
///
/// # Errors
/// If the next character is not a period, this function call will fail.
pub fn period(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(".")(i)
}

/// Parses a solidus character.
///
/// # Errors
/// If the next character is not a solidus, this function call will fail.
pub fn solidus(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("/")(i)
}

/// Parses a colon character.
///
/// # Errors
/// If the next character is not a colon, this function call will fail.
pub fn colon(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(":")(i)
}

/// Parses a semicolon character.
///
/// # Errors
/// If the next character is not a semicolon, this function call will fail.
pub fn semicolon(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(";")(i)
}

/// Parses a less than operator character.
///
/// # Errors
/// If the next character is not a less than operator, this function call will
/// fail.
pub fn less_than_operator(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("<")(i)
}

/// Parses an equals operator character.
///
/// # Errors
/// If the next character is not an equals operator, this function call will
/// fail.
pub fn equals_operator(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("=")(i)
}

/// Parses a greater than operator character.
///
/// # Errors
/// If the next character is not a greater than operator, this function call
/// will fail.
pub fn greater_than_operator(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(">")(i)
}

/// Parses a question mark character.
///
/// # Errors
/// If the next character is not a question mark, this function call will fail.
pub fn question_mark(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("?")(i)
}

/// Parses a left bracket character.
///
/// # Errors
/// If the next character is not a left bracket, this function call will fail.
pub fn left_bracket(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("[")(i)
}

/// Parses a right bracket character.
///
/// # Errors
/// If the next character is not a right bracket, this function call will fail.
pub fn right_bracket(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("]")(i)
}

/// Parses a circumflex character.
///
/// # Errors
/// If the next character is not a circumflex, this function call will fail.
pub fn circumflex(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("^")(i)
}

/// Parses an underscore character.
///
/// # Errors
/// If the next character is not an underscore, this function call will fail.
pub fn underscore(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("_")(i)
}

/// Parses an vertical bar character.
///
/// # Errors
/// If the next character is not an vertical bar, this function call will fail.
pub fn vertical_bar(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("|")(i)
}

/// Parses an left brace character.
///
/// # Errors
/// If the next character is not a left brace, this function call will fail.
pub fn left_brace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("{")(i)
}

/// Parses a right brace character.
///
/// # Errors
/// If the next character is not a right brace, this function call will fail.
pub fn right_brace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("}")(i)
}

/// Parses a dollar sign character.
///
/// # Errors
/// If the next character is not a dollar sign, this function call will fail.
pub fn dollar_sign(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("$")(i)
}

/// Parses an apostrophe character.
///
/// # Errors
/// If the next character is not an apostrophe, this function call will fail.
pub fn apostrophe(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("'")(i)
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
            map(apostrophe, |_| SqlSpecialCharacter::Apostrophe),
        )),
    ))(i)
}
#[cfg(test)]
mod tests {
    use crate::common::parsers::sql_special_character;
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

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
