use pretty_assertions::assert_str_eq;

use sql_helper::ansi::parser::parse_statement;
use sql_helper::ansi::Statement;

/// Tests if the parsed statement serialization is the same as the original
/// input.
#[track_caller]
pub fn verified_stmt(input: &str) -> Statement {
    let (_, stmt) = parse_statement(input.as_ref()).unwrap();
    assert_str_eq!(input, stmt.to_string());
    stmt
}
