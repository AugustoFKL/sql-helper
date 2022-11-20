use std::fmt;

use crate::ansi::data_type_structures::ast::DataType;
use crate::common::Ident;

pub mod data_type_structures;
pub mod parser;

/// `ANSI` statements [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#SQL-executable-statement
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Statement {
    /// `CREATE SCHEMA` statement.
    CreateSchema(CreateSchema),
    /// `DROP SCHEMA` statement.
    DropSchema(DropSchema),
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

/// `DROP SCHEMA` statement [(1)].
///
/// # Supported syntax
/// ```doc
/// DROP SCHEMA <schema name> <drop behavior>
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_11_2_drop_schema_statement
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DropSchema {
    /// `<schema name>`
    schema_name: SchemaName,
    /// `<drop behavior>`
    drop_behavior: DropBehavior,
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

/// Possible behaviours when dropping a structure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DropBehavior {
    /// CASCADE - all dependencies are dropped.
    Cascade,
    /// RESTRICT - the drop is restricted to the specific structure.
    Restrict,
}

/// Column definition for `ANSI` columns [(1)].
///
/// # Supported syntax
/// `<column name> [<data type>]`
///
/// [1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#column-definition
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ColumnDefinition {
    /// `<column name>`
    column_name: Ident,
    /// `[<data_type>]`
    opt_data_type: Option<DataType>,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateSchema(create_schema) => {
                write!(f, "{create_schema}")
            }
            Statement::DropSchema(drop_schema) => {
                write!(f, "{drop_schema}")
            }
        }
    }
}

impl CreateSchema {
    #[must_use]
    pub fn new(schema_name_clause: SchemaNameClause) -> Self {
        Self { schema_name_clause }
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

impl DropSchema {
    #[must_use]
    pub fn new(schema_name: SchemaName, drop_behavior: DropBehavior) -> Self {
        Self {
            schema_name,
            drop_behavior,
        }
    }

    #[must_use]
    pub fn schema_name(&self) -> &SchemaName {
        &self.schema_name
    }

    #[must_use]
    pub fn drop_behavior(&self) -> DropBehavior {
        self.drop_behavior
    }
}

impl fmt::Display for DropSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DROP SCHEMA {} {};",
            self.schema_name(),
            self.drop_behavior()
        )?;
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
    #[must_use]
    pub fn new(opt_catalog_name: Option<&Ident>, name: &Ident) -> Self {
        Self {
            name: name.clone(),
            opt_catalog_name: opt_catalog_name.cloned(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    #[must_use]
    pub fn opt_catalog_name(&self) -> Option<&Ident> {
        self.opt_catalog_name.as_ref()
    }
}

impl fmt::Display for SchemaName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(catalog_name) = self.opt_catalog_name() {
            write!(f, "{catalog_name}.")?;
        }

        write!(f, "{}", self.name())?;

        Ok(())
    }
}

impl fmt::Display for DropBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cascade => {
                write!(f, "CASCADE")?;
            }
            Self::Restrict => {
                write!(f, "RESTRICT")?;
            }
        }

        Ok(())
    }
}

impl ColumnDefinition {
    #[must_use]
    pub fn new(column_name: &Ident) -> Self {
        Self {
            column_name: column_name.clone(),
            opt_data_type: None,
        }
    }

    pub fn with_data_type(&mut self, data_type: DataType) {
        self.opt_data_type = Some(data_type);
    }

    #[must_use]
    pub fn column_name(&self) -> &Ident {
        &self.column_name
    }

    #[must_use]
    pub fn opt_data_type(&self) -> Option<DataType> {
        self.opt_data_type
    }
}

impl fmt::Display for ColumnDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.column_name)?;

        if let Some(data_type) = self.opt_data_type() {
            write!(f, " {data_type}")?;
        }

        Ok(())
    }
}
