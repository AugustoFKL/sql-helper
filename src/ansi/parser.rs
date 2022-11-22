use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

use crate::ansi::parser::create_schema::create_schema;
use crate::ansi::parser::create_table::create_table;
use crate::ansi::parser::drop_schema::drop_schema;
use crate::ansi::parser::drop_table::drop_table;
use crate::ansi::Statement;

pub mod common;
pub mod create_schema;
pub mod create_table;
pub mod data_types;
pub mod drop_schema;
pub mod drop_table;

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
