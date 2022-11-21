pub mod create_table;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace1;
use nom::combinator::{map, opt, peek};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::ansi::data_type_structures::parser::data_type;
use crate::ansi::{
    ColumnDefinition, CreateSchema, DropBehavior, DropSchema, DropTable, LocalOrSchemaQualifier,
    LocalQualifier, SchemaName, SchemaNameClause, Statement, TableName,
};
use crate::ansi::parser::create_table::create_table;
use crate::common::parsers::{ident, statement_terminator};

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
        map(drop_table, Statement::DropTable),
        map(create_table, Statement::CreateTable),
    ))(i)
}

fn create_schema(i: &[u8]) -> IResult<&[u8], CreateSchema> {
    let (i, schema_name_clause) = delimited(
        tuple((
            tag_no_case("CREATE"),
            multispace1,
            tag_no_case("SCHEMA"),
            multispace1,
        )),
        schema_name_clause,
        statement_terminator,
    )(i)?;

    let create_schema = CreateSchema { schema_name_clause };

    Ok((i, create_schema))
}

fn drop_schema(i: &[u8]) -> IResult<&[u8], DropSchema> {
    let (i, (schema_name, drop_behavior)) = delimited(
        tuple((
            tag_no_case("DROP"),
            multispace1,
            tag_no_case("SCHEMA"),
            multispace1,
        )),
        tuple((terminated(schema_name, multispace1), drop_behavior)),
        statement_terminator,
    )(i)?;

    let drop_schema = DropSchema {
        schema_name,
        drop_behavior,
    };

    Ok((i, drop_schema))
}

fn drop_table(i: &[u8]) -> IResult<&[u8], DropTable> {
    let (i, (table_name, drop_behavior)) = delimited(
        tuple((
            tag_no_case("DROP"),
            multispace1,
            tag_no_case("TABLE"),
            multispace1,
        )),
        tuple((terminated(table_name, multispace1), drop_behavior)),
        statement_terminator,
    )(i)?;

    let drop_table = DropTable::new(&table_name, drop_behavior);

    Ok((i, drop_table))
}

fn table_name(i: &[u8]) -> IResult<&[u8], TableName> {
    let (i, (opt_local_or_schema, name)) =
        tuple((opt(terminated(local_or_schema_qualifier, tag("."))), ident))(i)?;

    let mut table_name = TableName::new(&name);
    if let Some(local_or_schema) = opt_local_or_schema {
        table_name.with_local_or_schema(local_or_schema);
    }

    Ok((i, table_name))
}

fn local_or_schema_qualifier(i: &[u8]) -> IResult<&[u8], LocalOrSchemaQualifier> {
    alt((
        map(local_qualifier, LocalOrSchemaQualifier::LocalQualifier),
        map(
            schema_for_qualified_table_name,
            LocalOrSchemaQualifier::Schema,
        ),
    ))(i)
}

fn local_qualifier(i: &[u8]) -> IResult<&[u8], LocalQualifier> {
    map(tag_no_case("MODULE"), |_| LocalQualifier::Module)(i)
}

fn schema_name_clause(i: &[u8]) -> IResult<&[u8], SchemaNameClause> {
    let (remaining, (schema_name_clause,)) = tuple((alt((
        map(
            tuple((
                terminated(
                    schema_name,
                    tuple((multispace1, tag_no_case("AUTHORIZATION"), multispace1)),
                ),
                ident,
            )),
            |(schema_name, authorization_name)| {
                SchemaNameClause::NamedAuthorization(schema_name, authorization_name)
            },
        ),
        map(
            preceded(tuple((tag_no_case("AUTHORIZATION"), multispace1)), ident),
            SchemaNameClause::Authorization,
        ),
        map(schema_name, SchemaNameClause::Simple),
    )),))(i)?;

    Ok((remaining, schema_name_clause))
}

fn schema_name(i: &[u8]) -> IResult<&[u8], SchemaName> {
    alt((
        map(
            tuple((terminated(ident, tag(".")), ident)),
            |(catalog, schema)| SchemaName::new(Some(&catalog), &schema),
        ),
        map(ident, |schema| SchemaName::new(None, &schema)),
    ))(i)
}

fn schema_for_qualified_table_name(i: &[u8]) -> IResult<&[u8], SchemaName> {
    alt((
        map(
            terminated(
                tuple((terminated(ident, tag(".")), ident)),
                peek(tuple((tag("."), ident))),
            ),
            |(catalog, schema)| SchemaName::new(Some(&catalog), &schema),
        ),
        map(
            terminated(ident, peek(tuple((tag("."), ident)))),
            |schema| SchemaName::new(None, &schema),
        ),
    ))(i)
}

#[allow(dead_code)]
fn column_definition(i: &[u8]) -> IResult<&[u8], ColumnDefinition> {
    let (i, (column_name, opt_data_type)) =
        tuple((ident, opt(preceded(multispace1, data_type))))(i)?;

    let mut column_def = ColumnDefinition::new(&column_name);

    if let Some(data_type) = opt_data_type {
        column_def.with_data_type(data_type);
    }

    Ok((i, column_def))
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
