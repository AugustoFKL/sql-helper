use std::fmt;

use crate::ansi::ast::data_types::DataType;
use crate::common::Ident;

/// Qualified or unqualified identifier representing a schema.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SchemaName {
    /// Schema unqualified name.
    name: Ident,
    /// Optional catalog qualifier.
    opt_catalog_name: Option<Ident>,
}

/// Table name with possibly local or schema qualification (`<table name>`).
///
/// # Supported syntax
/// ```plaintext
/// [<local or schema qualifier>.]<identifier>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TableName {
    name: Ident,
    opt_local_or_schema: Option<LocalOrSchemaQualifier>,
}

/// Schema name or local qualifier (`<local or schema qualifier>`).
///
/// # Supported syntax
/// ```plaintext
/// <schema_name>
/// | <local qualifier>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LocalOrSchemaQualifier {
    Schema(SchemaName),
    LocalQualifier(LocalQualifier),
}

/// Local qualifier (`<local qualifier>`).
///
/// # Supported syntax
/// ```plaintext
/// MODULE
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum LocalQualifier {
    /// `MODULE`
    #[default]
    Module,
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

/// Possible behaviours when dropping a structure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DropBehavior {
    /// CASCADE - all dependencies are dropped.
    Cascade,
    /// RESTRICT - the drop is restricted to the specific structure.
    Restrict,
}

/// Referential action.
///
/// # Supported syntax
/// ```plaintext
///   CASCADE
/// | SET NULL
/// | SET DEFAULT
/// | RESTRICT
/// | NO ACTION
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ReferentialAction {
    /// `CASCADE`.
    Cascade,
    /// `SET NULL`.
    SetNull,
    /// `SET DEFAULT`.
    SetDefault,
    /// `RESTRICT`.
    Restrict,
    /// `NO ACTION`.
    NoAction,
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

impl TableName {
    #[must_use]
    pub fn new(name: &Ident) -> Self {
        Self {
            name: name.clone(),
            opt_local_or_schema: None,
        }
    }

    pub fn with_local_or_schema(&mut self, local_or_schema: LocalOrSchemaQualifier) -> &mut Self {
        self.opt_local_or_schema = Some(local_or_schema);
        self
    }

    #[must_use]
    pub fn name(&self) -> &Ident {
        &self.name
    }

    #[must_use]
    pub fn opt_local_or_schema(&self) -> Option<&LocalOrSchemaQualifier> {
        self.opt_local_or_schema.as_ref()
    }
}

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(local_or_schema) = self.opt_local_or_schema() {
            write!(f, "{local_or_schema}.")?;
        }
        write!(f, "{}", self.name())?;
        Ok(())
    }
}

impl fmt::Display for LocalOrSchemaQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema(schema) => {
                write!(f, "{schema}")?;
            }
            Self::LocalQualifier(local) => {
                write!(f, "{local}")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for LocalQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MODULE")?;
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

    pub fn with_data_type(&mut self, data_type: DataType) -> &mut Self {
        self.opt_data_type = Some(data_type);
        self
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

impl fmt::Display for ReferentialAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cascade => write!(f, "CASCADE")?,
            Self::SetNull => write!(f, "SET NULL")?,
            Self::SetDefault => write!(f, "SET DEFAULT")?,
            Self::Restrict => write!(f, "RESTRICT")?,
            Self::NoAction => write!(f, "NO ACTION")?,
        }

        Ok(())
    }
}
