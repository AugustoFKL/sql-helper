use std::fmt;

use crate::ansi::ast::create_schema::CreateSchema;
use crate::ansi::ast::create_table::CreateTable;
use crate::ansi::ast::drop_schema::DropSchema;
use crate::ansi::ast::drop_table::DropTable;

pub mod ast;
pub mod parser;

/// `ANSI` ast [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#SQL-executable-statement
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Statement {
    /// `CREATE SCHEMA` statement.
    CreateSchema(CreateSchema),
    /// `DROP SCHEMA` statement.
    DropSchema(DropSchema),
    /// DROP TABLE statement
    DropTable(DropTable),
    /// CREATE TABLE statement
    CreateTable(CreateTable),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateSchema(create_schema) => write!(f, "{create_schema}")?,
            Self::DropSchema(drop_schema) => write!(f, "{drop_schema}")?,
            Self::DropTable(drop_table) => write!(f, "{drop_table}")?,
            Self::CreateTable(create_table) => write!(f, "{create_table}")?,
        }
        Ok(())
    }
}
