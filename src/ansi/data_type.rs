use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::map;
use nom::IResult;

use crate::ansi::DataType;

/// Parses `ANSI` data type [(1)], [(2)].
///
/// # Errors
/// This function returns an error if the data type is not supported or not
/// exists in the current dialect.
///
/// [(1)]: crate::ansi::DataType
/// [(2)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_1_data_type
fn parse_data_type(input: &str) -> IResult<&str, DataType> {
    alt((parse_character_string,))(input)
}

/// Parses `ANSI` character string data types [(1)].
///
/// # Errors
/// This function returns an error if the data type is not supported or not
/// exists in the current dialect.
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-string-type
fn parse_character_string(input: &str) -> IResult<&str, DataType> {
    alt((
        map(tag_no_case("CHARACTER VARYING"), |_| {
            DataType::CharacterVarying
        }),
        map(tag_no_case("CHAR VARYING"), |_| DataType::CharVarying),
        map(tag_no_case("CHARACTER"), |_| DataType::Character),
        map(tag_no_case("VARCHAR"), |_| DataType::Varchar),
        map(tag_no_case("CHAR"), |_| DataType::Char),
    ))(input)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("CHARACTER VARYING", DataType::CharacterVarying)]
    #[test_case("CHAR VARYING", DataType::CharVarying)]
    #[test_case("CHARACTER", DataType::Character)]
    #[test_case("VARCHAR", DataType::Varchar)]
    #[test_case("CHAR", DataType::Char)]
    fn parse_character_string(input: &str, expected: DataType) {
        let (remaining, parsed) = parse_data_type(input).unwrap();
        assert_eq!(parsed, expected);
        assert!(remaining.is_empty());
    }
}
