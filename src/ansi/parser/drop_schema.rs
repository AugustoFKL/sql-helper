use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

use crate::ansi::ast::drop_schema::DropSchema;
use crate::ansi::parser::common::{drop_behavior, schema_name};
use crate::common::parsers::statement_terminator;

/// Parses a `DROP SCHEMA` statement.
///
/// # Errors
/// If the drop table statement is malformed or has unsupported features, this
/// function call will fail. Check the drop table statement documentation
/// [(1)][`DropSchema`] for supported syntax.
pub fn drop_schema(i: &[u8]) -> IResult<&[u8], DropSchema> {
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

    let drop_schema = DropSchema::new(&schema_name, drop_behavior);

    Ok((i, drop_schema))
}
