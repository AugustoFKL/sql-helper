use nom::bytes::complete::tag_no_case;
use nom::sequence::{delimited, pair};
use nom::IResult;

use crate::ansi::ast::drop_schema::DropSchema;
use crate::ansi::parser::common::{drop_behavior, schema_name};
use crate::common::parsers::{statement_terminator, terminated_ws1};

/// Parses a `DROP SCHEMA` statement.
///
/// # Errors
/// If the drop table statement is malformed or has unsupported features, this
/// function call will fail. Check the drop table statement documentation
/// [(1)][`DropSchema`] for supported syntax.
pub fn drop_schema(i: &[u8]) -> IResult<&[u8], DropSchema> {
    let (i, (schema_name, drop_behavior)) = delimited(
        pair(
            terminated_ws1(tag_no_case("DROP")),
            terminated_ws1(tag_no_case("SCHEMA")),
        ),
        pair(terminated_ws1(schema_name), drop_behavior),
        statement_terminator,
    )(i)?;

    let drop_schema = DropSchema::new(&schema_name, drop_behavior);

    Ok((i, drop_schema))
}
