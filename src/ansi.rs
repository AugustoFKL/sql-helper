use std::fmt;
use std::fmt::Formatter;

use crate::common::Ident;

/// `ANSI` parser methods.
pub mod parser;

/// `ANSI` data types [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_1_data_type
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DataType {
    /// CHARACTER\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Character,
    /// CHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Char,
    /// CHARACTER VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharacterVarying,
    /// CHAR VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharVarying,
    /// VARCHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Varchar,
}

/// `ANSI` statements [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#SQL-executable-statement
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Statement {
    /// `CREATE SCHEMA` statement.
    CreateSchema(CreateSchema),
}

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

/// Qualified or unqualified identifier representing a schema.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SchemaName {
    /// Schema unqualified name.
    name: Ident,
    /// Optional catalog qualifier.
    opt_catalog_name: Option<Ident>,
}

impl CreateSchema {
    /// Create a new `CreateSchema`.
    ///
    /// The fields in the new statement are the obligatory fields. Optional
    /// fields should be set via `with_...` methods.
    #[must_use]
    pub fn new(schema_name_clause: SchemaNameClause) -> Self {
        Self { schema_name_clause }
    }

    /// Returns a reference to the schema name clause.
    #[must_use]
    pub fn schema_name_clause(&self) -> &SchemaNameClause {
        &self.schema_name_clause
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateSchema(create_schema) => {
                write!(f, "{create_schema}")
            }
        }
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

impl SchemaName {
    /// Creates a new schema name.
    #[must_use]
    pub fn new(opt_catalog_name: Option<&Ident>, name: &Ident) -> Self {
        Self {
            name: name.clone(),
            opt_catalog_name: opt_catalog_name.cloned(),
        }
    }

    /// Returns a reference to the schema name identifier.
    #[must_use]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// Returns an optional reference to the schema catalog identifier.
    #[must_use]
    pub fn opt_catalog_name(&self) -> Option<&Ident> {
        self.opt_catalog_name.as_ref()
    }
}

impl fmt::Display for SchemaName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(catalog_name) = self.opt_catalog_name() {
            write!(f, "{}.", catalog_name)?;
        }

        write!(f, "{}", self.name())?;

        Ok(())
    }
}
