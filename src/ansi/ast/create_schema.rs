use std::fmt;

use crate::ansi::ast::common::SchemaName;
use crate::common::Ident;

/// `CREATE SCHEMA` statement [(1)].
///
/// # Supported syntax
/// ```doc
/// CREATE SCHEMA <schema name clause>
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#schema-definition
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CreateSchema {
    /// `<schema name clause>`
    schema_name_clause: SchemaNameClause,
    // TODO schema element
}

/// Create schema statement `<schema name clause>`.
///
/// # Supported syntax
/// ```doc
/// <schema name>
/// | AUTHORIZATION <schema authorization identifier>
/// | <schema name> AUTHORIZATION <schema authorization identifier>
///
/// <schema authorization identifier>: <identifier>
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#schema-definition
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SchemaNameClause {
    /// <schema name>
    Simple(SchemaName),
    /// AUTHORIZATION <schema authorization identifier>
    Authorization(Ident),
    /// <schema name> AUTHORIZATION <schema authorization identifier
    NamedAuthorization(SchemaName, Ident),
}

impl CreateSchema {
    #[must_use]
    pub fn new(schema_name_clause: &SchemaNameClause) -> Self {
        Self {
            schema_name_clause: schema_name_clause.clone(),
        }
    }

    #[must_use]
    pub fn schema_name_clause(&self) -> &SchemaNameClause {
        &self.schema_name_clause
    }
}

impl fmt::Display for CreateSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE SCHEMA {};", self.schema_name_clause())?;
        Ok(())
    }
}

impl fmt::Display for SchemaNameClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaNameClause::Simple(schema_name) => {
                write!(f, "{schema_name}")?;
            }
            SchemaNameClause::Authorization(authorization) => {
                write!(f, "AUTHORIZATION {authorization}")?;
            }
            SchemaNameClause::NamedAuthorization(schema_name, authorization) => {
                write!(f, "{schema_name} AUTHORIZATION {authorization}")?;
            }
        }
        Ok(())
    }
}
