use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

use crate::ansi::ast::drop_table::DropTable;
use crate::ansi::parser::common::{drop_behavior, table_name};
use crate::common::parsers::statement_terminator;

/// Parses a `DROP TABLE` statement.
///
/// # Errors
/// If the drop table statement is malformed or has unsupported features, this
/// function call will fail. Check the drop table statement documentation
/// [(1)][`DropTable`] for supported syntax.
pub fn drop_table(i: &[u8]) -> IResult<&[u8], DropTable> {
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
