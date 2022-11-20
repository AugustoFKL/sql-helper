use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use crate::ansi::data_type_structures::parser::data_type;
use crate::ansi::{
    ColumnDefinition, CreateSchema, DropBehavior, DropSchema, SchemaName, SchemaNameClause,
    Statement,
};
use crate::common::parsers::{ident, parse_statement_terminator};

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

#[allow(dead_code)]
fn column_definition(i: &[u8]) -> IResult<&[u8], ColumnDefinition> {
    let (remaining, (column_name, _, opt_data_type)) =
        tuple((ident, multispace0, opt(data_type)))(i)?;

    let mut column_def = ColumnDefinition::new(&column_name);

    if let Some(data_type) = opt_data_type {
        column_def.with_data_type(data_type);
    }

    Ok((remaining, column_def))
}

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

    use crate::ansi::data_type_structures::ast::DataType;
    use crate::common::Ident;

    use super::*;

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
