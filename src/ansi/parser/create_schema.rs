use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::ansi::ast::create_schema::{CreateSchema, SchemaNameClause};
use crate::ansi::parser::common::schema_name;
use crate::common::parsers::{ident, statement_terminator};

/// Parses a `CREATE SCHEMA` statement [(1)](SchemaNameClause).
///
/// # Errors
/// If the drop table statement is malformed or has unsupported features, this
/// function call will fail. Check the create table statement documentation for
/// supported syntax.
pub fn create_schema(i: &[u8]) -> IResult<&[u8], CreateSchema> {
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

    let create_schema = CreateSchema::new(&schema_name_clause);

    Ok((i, create_schema))
}

/// Parses schema name clause [(1)](SchemaNameClause).
///
/// # Errors
/// If the schema name clause is invalid, this function call will fail. Check
/// the described syntax on the schema name clause structure to understand the
/// supported syntax.
pub fn schema_name_clause(i: &[u8]) -> IResult<&[u8], SchemaNameClause> {
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
