use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use crate::ansi::ast::{SchemaName, SchemaNameClause};
use crate::ansi::{CreateSchema, DataType, Statement};
use crate::common::parsers::{ident, parse_statement_terminator};

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

/// Parses a `Statement` [(1)] from the give input.
///
/// # Errors
/// This method will raise an error if the input is malformed, or if the
/// statement is not supported.
///
/// [(1)]: crate::ansi::Statement
pub fn parse_statement(i: &[u8]) -> IResult<&[u8], Statement> {
    alt((map(parse_create_schema, |stmt| {
        Statement::CreateSchema(stmt)
    }),))(i)
}

/// Parses a `CREATE SCHEMA` [(1)] statement.
///
/// # Errors
/// This method will raise an error if the input is malformed, or if the
/// statement is not supported.
///
/// [(1)]: crate::ansi::CreateSchema
fn parse_create_schema(i: &[u8]) -> IResult<&[u8], CreateSchema> {
    let (remaining_input, (_, _, _, schema_name_clause, _)) = tuple((
        tag_no_case("CREATE"),
        multispace0,
        tag_no_case("SCHEMA"),
        parse_schema_name_clause,
        parse_statement_terminator,
    ))(i)?;

    let create_schema = CreateSchema { schema_name_clause };

    Ok((remaining_input, create_schema))
}

/// Parses a `<schema name clause>` [(1)].
///
/// # Errors
/// This method returns an error if the schema name is malformed or contains
/// unsupported features.
///
/// [(1)]: SchemaNameClause
fn parse_schema_name_clause(i: &[u8]) -> IResult<&[u8], SchemaNameClause> {
    let (remaining, (_, schema_name_clause)) = tuple((
        multispace0,
        (alt((
            map(
                tuple((
                    schema_name,
                    multispace0,
                    tag("AUTHORIZATION"),
                    multispace0,
                    ident,
                    multispace0,
                )),
                |(schema_name, _, _, _, authorization_name, _)| {
                    SchemaNameClause::NamedAuthorization(schema_name, authorization_name)
                },
            ),
            map(
                tuple((tag("AUTHORIZATION"), multispace0, ident)),
                |(_, _, authorization_name)| SchemaNameClause::Authorization(authorization_name),
            ),
            map(schema_name, |schema_name| {
                SchemaNameClause::Simple(schema_name)
            }),
        ))),
    ))(i)?;

    Ok((remaining, schema_name_clause))
}

/// Parses a `<schema name>` [(1)].
///
/// # Errors
/// This function returns an error if the schema name or catalog name
/// identifiers cannot be parsed or if it has more than one qualifier.
///
/// [(1)]: SchemaName
fn schema_name(i: &[u8]) -> IResult<&[u8], SchemaName> {
    let (i, idents) = separated_list1(tag("."), ident)(i)?;

    let schema_name = match &idents[..] {
        [schema_name] => SchemaName::new(None, schema_name),
        [catalog_name, schema_name] => SchemaName::new(Some(catalog_name), schema_name),
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((i, schema_name))
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
