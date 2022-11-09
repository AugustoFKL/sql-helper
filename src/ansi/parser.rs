use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{multispace0, multispace1, u32};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::ansi::{
    CharacterLength, CharacterLengthUnits, ColumnDefinition, CreateSchema, DataType, DropBehavior,
    DropSchema, SchemaName, SchemaNameClause, Statement,
};
use crate::common::parsers::{ident, parse_statement_terminator};

/// Parses `ANSI` data type [(1)].
///
/// # Errors
/// This function returns an error if the data type is not supported or not
/// exists in the current dialect.
///
/// [(1)]: crate::ansi::DataType
#[allow(unused)]
fn data_type(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((character_string,))(input)
}

/// Parses `ANSI` character string data types.
///
/// # Errors
/// This function returns an error if the data type is not supported or not
/// exists in the current dialect.
#[allow(unused)]
fn character_string(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(
            tuple((tag_no_case("CHARACTER VARYING"), character_length)),
            |(_, opt_len)| DataType::CharacterVarying(opt_len),
        ),
        map(
            tuple((tag_no_case("CHAR VARYING"), character_length)),
            |(_, opt_len)| DataType::CharVarying(opt_len),
        ),
        map(
            tuple((tag_no_case("CHARACTER"), character_length)),
            |(_, opt_len)| DataType::Character(opt_len),
        ),
        map(
            tuple((tag_no_case("VARCHAR"), character_length)),
            |(_, opt_len)| DataType::Varchar(opt_len),
        ),
        map(
            tuple((tag_no_case("CHAR"), character_length)),
            |(_, opt_len)| DataType::Char(opt_len),
        ),
    ))(input)
}

/// Parses optional `CharacterLength` information.
///
/// # Errors
/// This function will throw an error if the input string is malformed, invalid
/// or empty. As the implementation is complete, there's no possible
/// unimplemented error scenarios.
/// ```
fn character_length(i: &[u8]) -> IResult<&[u8], Option<CharacterLength>> {
    let characters_mapping = alt((
        map(tag_no_case("CHARACTERS"), |_| {
            CharacterLengthUnits::Characters
        }),
        map(tag_no_case("OCTETS"), |_| CharacterLengthUnits::Octets),
    ));

    let interior = map(
        tuple((
            u32,
            opt(map(
                tuple((multispace1, characters_mapping)),
                |(_, units)| units,
            )),
        )),
        |(length, opt_units)| {
            let mut character_length = CharacterLength::new(length);
            character_length.with_units(opt_units);
            character_length
        },
    );

    opt(delimited(
        tuple((multispace0, tag("("))),
        interior,
        tuple((tag(")"), multispace0)),
    ))(i)
}

/// Parses a `Statement` [(1)] from the give input.
///
/// # Errors
/// This method will raise an error if the input is malformed, or if the
/// statement is not supported.
///
/// [(1)]: crate::ansi::Statement
pub fn parse_statement(i: &[u8]) -> IResult<&[u8], Statement> {
    alt((
        map(create_schema, Statement::CreateSchema),
        map(drop_schema, Statement::DropSchema),
    ))(i)
}

/// Parses a `CREATE SCHEMA` [(1)] statement.
///
/// # Errors
/// This method will raise an error if the input is malformed, or if the
/// statement is not supported.
///
/// [(1)]: crate::ansi::CreateSchema
fn create_schema(i: &[u8]) -> IResult<&[u8], CreateSchema> {
    let (remaining_input, (_, _, _, schema_name_clause, _)) = tuple((
        tag_no_case("CREATE"),
        multispace0,
        tag_no_case("SCHEMA"),
        schema_name_clause,
        parse_statement_terminator,
    ))(i)?;

    let create_schema = CreateSchema { schema_name_clause };

    Ok((remaining_input, create_schema))
}

/// Parses a `DROP SCHEMA` [(1)] statement.
///
/// # Errors
/// This method will raise an error if the input is malformed, or if the
/// statement is not supported.
///
/// [(1)]: crate::ansi::DropSchema
fn drop_schema(i: &[u8]) -> IResult<&[u8], DropSchema> {
    let (remaining_input, (_, _, _, _, schema_name, _, drop_behavior, _)) = tuple((
        tag_no_case("DROP"),
        multispace0,
        tag_no_case("SCHEMA"),
        multispace0,
        schema_name,
        multispace0,
        drop_behavior,
        parse_statement_terminator,
    ))(i)?;

    let drop_schema = DropSchema {
        schema_name,
        drop_behavior,
    };

    Ok((remaining_input, drop_schema))
}

/// Parses a `<schema name clause>` [(1)].
///
/// # Errors
/// This method returns an error if the schema name is malformed or contains
/// unsupported features.
///
/// [(1)]: SchemaNameClause
fn schema_name_clause(i: &[u8]) -> IResult<&[u8], SchemaNameClause> {
    let (remaining, (_, schema_name_clause)) = tuple((
        multispace0,
        (alt((
            map(
                tuple((
                    schema_name,
                    multispace0,
                    tag_no_case("AUTHORIZATION"),
                    multispace0,
                    ident,
                    multispace0,
                )),
                |(schema_name, _, _, _, authorization_name, _)| {
                    SchemaNameClause::NamedAuthorization(schema_name, authorization_name)
                },
            ),
            map(
                tuple((tag_no_case("AUTHORIZATION"), multispace0, ident)),
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

/// Parses a `<column definition>` [(1)].
///
///
/// # Errors
/// This function will throw an error if the column definition is invalid or
/// malformed. This includes unsupported features such as data types.
///
/// [(1)]: crate::ansi::ColumnDefinition
pub fn column_definition(i: &[u8]) -> IResult<&[u8], ColumnDefinition> {
    let (remaining, (column_name, _, opt_data_type)) =
        tuple((ident, multispace0, opt(data_type)))(i)?;

    let mut column_def = ColumnDefinition::new(&column_name);

    if let Some(data_type) = opt_data_type {
        column_def.with_data_type(data_type);
    }

    Ok((remaining, column_def))
}

/// Parses a `<drop behavior>` [(1)].
///
/// # Errors
/// This function returns an error if the drop behavior is nor `RESTRICT` or
/// `CASCADE`.
///
/// [(1)]: DropBehavior
fn drop_behavior(i: &[u8]) -> IResult<&[u8], DropBehavior> {
    alt((
        map(tag_no_case("CASCADE"), |_| DropBehavior::Cascade),
        map(tag_no_case("RESTRICT"), |_| DropBehavior::Restrict),
    ))(i)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

    use crate::common::Ident;

    use super::*;

    #[test]
    fn parse_character_string() {
        macro_rules! assert_expected {
            ($input:expr, $expected:expr) => {{
                let (remaining, parsed) = data_type($input.as_ref()).unwrap();
                assert_eq!($expected, parsed);
                assert_str_eq!($input, parsed.to_string());
                assert!(remaining.is_empty());
            }};
        }

        assert_expected!("CHARACTER VARYING", DataType::CharacterVarying(None));

        assert_expected!(
            "CHARACTER VARYING(20)",
            DataType::CharacterVarying(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected!(
            "CHARACTER VARYING(20 OCTETS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected!(
            "CHARACTER VARYING(20 CHARACTERS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected!("CHAR VARYING", DataType::CharVarying(None));

        assert_expected!(
            "CHAR VARYING(20)",
            DataType::CharVarying(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected!(
            "CHAR VARYING(20 OCTETS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected!(
            "CHAR VARYING(20 CHARACTERS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected!("CHARACTER", DataType::Character(None));

        assert_expected!(
            "CHARACTER(20)",
            DataType::Character(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected!(
            "CHARACTER(20 OCTETS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected!(
            "CHARACTER(20 CHARACTERS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected!("VARCHAR", DataType::Varchar(None));

        assert_expected!(
            "VARCHAR(20)",
            DataType::Varchar(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected!(
            "VARCHAR(20 OCTETS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected!(
            "VARCHAR(20 CHARACTERS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected!("CHAR", DataType::Char(None));

        assert_expected!(
            "CHAR(20)",
            DataType::Char(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected!(
            "CHAR(20 OCTETS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected!(
            "CHAR(20 CHARACTERS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );
    }

    #[test]
    fn parse_column_definition_ast() {
        let input_1 = "name VARCHAR";
        let (_, column_def_1) = column_definition(input_1.as_ref()).unwrap();
        let expected_1 = ColumnDefinition {
            column_name: Ident::new(b"name"),
            opt_data_type: Some(DataType::Varchar(None)),
        };
        assert_eq!(column_def_1, expected_1);

        let input_2 = "name";
        let (_, column_def_2) = column_definition(input_2.as_ref()).unwrap();
        let expected_2 = ColumnDefinition {
            column_name: Ident::new(b"name"),
            opt_data_type: None,
        };
        assert_eq!(column_def_2, expected_2);
    }

    #[test_case("name")]
    #[test_case("name VARCHAR")]
    fn parse_column_definition_serialisation(input: &str) {
        assert_str_eq!(
            input,
            column_definition(input.as_ref()).unwrap().1.to_string()
        );
    }
}
