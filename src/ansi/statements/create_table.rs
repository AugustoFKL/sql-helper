use core::fmt;

use crate::ansi::{ColumnDefinition, TableName};
use crate::common::{display_comma_separated, if_some_string_preceded_by};

/// Create table statement.
///
/// # Supported syntax
/// ```plaintext
/// CREATE [<table scope>] TABLE <table name> <table contents source>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CreateTable {
    /// `[<table scope>]`.
    opt_table_scope: Option<TableScope>,
    /// `<table name>`.
    table_name: TableName,
    /// `<table contents source>`
    table_contents_source: TableContentsSource,
}

/// Table scope clause.
///
/// # Supported syntax
/// ```plaintext
/// <global or local> TEMPORARY
///
/// <global or local> ::=
///   GLOBAL
/// | LOCAL
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TableScope {
    /// `GLOBAL TEMPORARY`.
    Global,
    /// `LOCAL TEMPORARY`.
    Local,
}

/// Table contents source.
///
/// # Supported syntax
/// ```plaintext
/// <table element list>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TableContentsSource {
    /// `<table element list>`.
    TableElementList(TableElementList),
}

/// Table element list.
///
/// # Supported syntax
/// ```plaintext
/// (<table element> [{, <table element>}...])
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TableElementList {
    /// Element list.
    element_list: Vec<TableElement>,
}

/// Table element.
///
/// # Supported syntax
/// ```plaintext
/// <column definition>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TableElement {
    /// `<column definition>`.
    ColumnDefinition(ColumnDefinition),
}

impl CreateTable {
    #[must_use]
    pub fn new(table_name: &TableName, table_contents_source: &TableContentsSource) -> Self {
        Self {
            opt_table_scope: None,
            table_name: table_name.clone(),
            table_contents_source: table_contents_source.clone(),
        }
    }

    pub fn with_table_scope(&mut self, table_scope: TableScope) -> &mut Self {
        self.opt_table_scope = Some(table_scope);
        self
    }

    #[must_use]
    pub fn opt_table_scope(&self) -> Option<TableScope> {
        self.opt_table_scope
    }

    #[must_use]
    pub fn table_name(&self) -> &TableName {
        &self.table_name
    }

    #[must_use]
    pub fn table_contents_source(&self) -> &TableContentsSource {
        &self.table_contents_source
    }
}

impl fmt::Display for CreateTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CREATE{scope} TABLE {table_name} {table_contents_source}",
            scope = if_some_string_preceded_by(self.opt_table_scope(), " "),
            table_name = self.table_name(),
            table_contents_source = self.table_contents_source()
        )?;
        Ok(())
    }
}

impl fmt::Display for TableScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "GLOBAL TEMPORARY")?,
            Self::Local => write!(f, "LOCAL TEMPORARY")?,
        }
        Ok(())
    }
}

impl fmt::Display for TableContentsSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TableElementList(table_element_list) => write!(f, "{table_element_list}"),
        }
    }
}

impl TableElementList {
    #[must_use]
    pub fn new(element_list: &[TableElement]) -> Self {
        Self {
            element_list: element_list.to_vec(),
        }
    }

    #[must_use]
    pub fn element_list(&self) -> &[TableElement] {
        &self.element_list
    }
}
impl fmt::Display for TableElementList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_comma_separated(self.element_list()))?;
        Ok(())
    }
}

impl fmt::Display for TableElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ColumnDefinition(column_definition) => write!(f, "{column_definition}")?,
        }
        Ok(())
    }
}
