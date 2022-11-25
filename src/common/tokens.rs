use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::{Compare, IResult, InputTake};

/// Returns whether the input character is a ANSI whitespace or not, following
/// the unicode [white space list](1).
///
/// White space is any character in the Unicode General Category classes “Zs”,
/// “Zl”, and “Zp”, as well as any of the following characters:
///
/// — U+0009, Horizontal Tabulation
///
/// — U+000A, Line Feed
///
/// — U+000B, Vertical Tabulation
///
/// — U+000C, Form Feed
///
/// — U+000D, Carriage Return
///
/// — U+0085, Next Line
///
/// **OBS:** currently, we only consider UTF-8 characters for more easy
/// expansion. This can be reviewed later.
///
/// [1]: https://www.compart.com/en/unicode/bidiclass/WS
///
/// # Examples
/// ```rust
/// use sql_helper::common::tokens::is_whitespace;
/// let list = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x20, 0x85, 0xA0];
///
/// for item in list {
///     assert!(is_whitespace(char::from_u32(item).unwrap()));
/// }
/// ```
#[must_use]
pub fn is_whitespace(i: char) -> bool {
    let list = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x20, 0x85, 0xA0];

    list.contains(&(i as u32))
}

/// Parses a space character.
///
/// # Errors
/// If the next character is not a space, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::space;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     space(s)
/// }
/// assert_eq!(parser(" "), Ok(("", " ")));
/// ```
pub fn space<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b" "[..])(input)?;

    Ok((i, j))
}

/// Parses a double quote character.
///
/// # Errors
/// If the next character is not a double quote, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::double_quote;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     double_quote(s)
/// }
/// assert_eq!(parser("\""), Ok(("", "\"")));
/// ```
pub fn double_quote<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"\""[..])(input)?;

    Ok((i, j))
}

/// Parses a percent character.
///
/// # Errors
/// If the next character is not a percent, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::percent;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     percent(s)
/// }
/// assert_eq!(parser("%"), Ok(("", "%")));
/// ```
pub fn percent<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"%"[..])(input)?;

    Ok((i, j))
}

/// Parses an ampersand character.
///
/// # Errors
/// If the next character is not an ampersand, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::ampersand;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     ampersand(s)
/// }
/// assert_eq!(parser("&"), Ok(("", "&")));
/// ```
pub fn ampersand<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"&"[..])(input)?;

    Ok((i, j))
}

/// Parses a quote character.
///
/// # Errors
/// If the next character is not a quote, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::quote;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     quote(s)
/// }
/// assert_eq!(parser("'"), Ok(("", "'")));
/// ```
pub fn quote<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"'"[..])(input)?;

    Ok((i, j))
}

/// Parses a left paren character.
///
/// # Errors
/// If the next character is not a left paren, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::left_paren;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     left_paren(s)
/// }
/// assert_eq!(parser("("), Ok(("", "(")));
/// ```
pub fn left_paren<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"("[..])(input)?;

    Ok((i, j))
}

/// Parses a right paren character.
///
/// # Errors
/// If the next character is not a right paren, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::right_paren;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     right_paren(s)
/// }
/// assert_eq!(parser(")"), Ok(("", ")")));
/// ```
pub fn right_paren<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b")"[..])(input)?;

    Ok((i, j))
}

/// Parses an asterisk character.
///
/// # Errors
/// If the next character is not an asterisk, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::asterisk;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     asterisk(s)
/// }
/// assert_eq!(parser("*"), Ok(("", "*")));
/// ```
pub fn asterisk<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"*"[..])(input)?;

    Ok((i, j))
}

/// Parses a plus sign character.
///
/// # Errors
/// If the next character is not a plus sign, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::plus_sign;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     plus_sign(s)
/// }
/// assert_eq!(parser("+"), Ok(("", "+")));
/// ```
pub fn plus_sign<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"+"[..])(input)?;

    Ok((i, j))
}

/// Parses a comma character.
///
/// # Errors
/// If the next character is not a comma, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::comma;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     comma(s)
/// }
/// assert_eq!(parser(","), Ok(("", ",")));
/// ```
pub fn comma<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b","[..])(input)?;

    Ok((i, j))
}

/// Parses a minus sign character.
///
/// # Errors
/// If the next character is not a minus sign, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::minus_sign;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     minus_sign(s)
/// }
/// assert_eq!(parser("-"), Ok(("", "-")));
/// ```
pub fn minus_sign<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"-"[..])(input)?;

    Ok((i, j))
}

/// Parses a period character.
///
/// # Errors
/// If the next character is not a period, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::period;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     period(s)
/// }
/// assert_eq!(parser("."), Ok(("", ".")));
/// ```
pub fn period<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"."[..])(input)?;

    Ok((i, j))
}

/// Parses a solidus character.
///
/// # Errors
/// If the next character is not a solidus, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::solidus;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     solidus(s)
/// }
/// assert_eq!(parser("/"), Ok(("", "/")));
/// ```
pub fn solidus<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"/"[..])(input)?;

    Ok((i, j))
}

/// Parses a colon character.
///
/// # Errors
/// If the next character is not a colon, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::colon;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     colon(s)
/// }
/// assert_eq!(parser(":"), Ok(("", ":")));
/// ```
pub fn colon<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b":"[..])(input)?;

    Ok((i, j))
}

/// Parses a semicolon character.
///
/// # Errors
/// If the next character is not a semicolon, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::semicolon;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     semicolon(s)
/// }
/// assert_eq!(parser(";"), Ok(("", ";")));
/// ```
pub fn semicolon<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b";"[..])(input)?;

    Ok((i, j))
}

/// Parses a less than operator character.
///
/// # Errors
/// If the next character is not a less than operator, this function call will
/// fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::less_than_operator;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     less_than_operator(s)
/// }
/// assert_eq!(parser("<"), Ok(("", "<")));
/// ```
pub fn less_than_operator<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"<"[..])(input)?;

    Ok((i, j))
}

/// Parses an equals operator character.
///
/// # Errors
/// If the next character is not an equals operator, this function call will
/// fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::equals_operator;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     equals_operator(s)
/// }
/// assert_eq!(parser("="), Ok(("", "=")));
/// ```
pub fn equals_operator<T, E>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
    E: ParseError<T>,
{
    let (i, j) = tag(&b"="[..])(input)?;

    Ok((i, j))
}

/// Parses a greater than operator character.
///
/// # Errors
/// If the next character is not a greater than operator, this function call
/// will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::greater_than_operator;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     greater_than_operator(s)
/// }
/// assert_eq!(parser(">"), Ok(("", ">")));
/// ```
pub fn greater_than_operator<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b">"[..])(input)?;

    Ok((i, j))
}

/// Parses a question mark character.
///
/// # Errors
/// If the next character is not a question mark, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::question_mark;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     question_mark(s)
/// }
/// assert_eq!(parser("?"), Ok(("", "?")));
/// ```
pub fn question_mark<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"?"[..])(input)?;

    Ok((i, j))
}

/// Parses a left bracket character.
///
/// # Errors
/// If the next character is not a left bracket, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::left_bracket;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     left_bracket(s)
/// }
/// assert_eq!(parser("["), Ok(("", "[")));
/// ```
pub fn left_bracket<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"["[..])(input)?;

    Ok((i, j))
}

/// Parses a right bracket character.
///
/// # Errors
/// If the next character is not a right bracket, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::right_bracket;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     right_bracket(s)
/// }
/// assert_eq!(parser("]"), Ok(("", "]")));
/// ```
pub fn right_bracket<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"]"[..])(input)?;

    Ok((i, j))
}

/// Parses a circumflex character.
///
/// # Errors
/// If the next character is not a circumflex, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::circumflex;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     circumflex(s)
/// }
/// assert_eq!(parser("^"), Ok(("", "^")));
/// ```
pub fn circumflex<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"^"[..])(input)?;

    Ok((i, j))
}

/// Parses an underscore character.
///
/// # Errors
/// If the next character is not an underscore, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::underscore;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     underscore(s)
/// }
/// assert_eq!(parser("_"), Ok(("", "_")));
/// ```
pub fn underscore<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"_"[..])(input)?;

    Ok((i, j))
}

/// Parses an vertical bar character.
///
/// # Errors
/// If the next character is not an vertical bar, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::vertical_bar;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     vertical_bar(s)
/// }
/// assert_eq!(parser("|"), Ok(("", "|")));
/// ```
pub fn vertical_bar<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"|"[..])(input)?;

    Ok((i, j))
}

/// Parses an left brace character.
///
/// # Errors
/// If the next character is not a left brace, this function call will fail.
///
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::left_brace;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     left_brace(s)
/// }
/// assert_eq!(parser("{"), Ok(("", "{")));
/// ```
pub fn left_brace<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"{"[..])(input)?;

    Ok((i, j))
}

/// Parses a right brace character.
///
/// # Errors
/// If the next character is not a right brace, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::right_brace;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     right_brace(s)
/// }
/// assert_eq!(parser("}"), Ok(("", "}")));
/// ```
pub fn right_brace<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"}"[..])(input)?;

    Ok((i, j))
}

/// Parses a dollar sign character.
///
/// # Errors
/// If the next character is not a dollar sign, this function call will fail.
///
/// # Examples
/// ```rust
/// # use nom::IResult;
/// # use sql_helper::common::tokens::dollar_sign;
/// fn parser(s: &str) -> IResult<&str, &str> {
///     dollar_sign(s)
/// }
/// assert_eq!(parser("$"), Ok(("", "$")));
/// ```
pub fn dollar_sign<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    for<'a> T: Clone + InputTake + Compare<&'a [u8]>,
{
    let (i, j) = tag(&b"$"[..])(input)?;

    Ok((i, j))
}
