use std::fmt;

use crate::common::Ident;

/// `ANSI` parser methods.
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

/// `ANSI` data types [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_1_data_type
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DataType {
    /// CHARACTER\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Character(Option<CharacterLength>),
    /// CHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Char(Option<CharacterLength>),
    /// CHARACTER VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharacterVarying(Option<CharacterLength>),
    /// CHAR VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharVarying(Option<CharacterLength>),
    /// VARCHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Varchar(Option<CharacterLength>),
    /// BOOLEAN
    Boolean,
}

/// Character length of a string literal [(1)].
///
/// # Supported syntax
/// ```doc
/// <length> [<character length units>]
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-length
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CharacterLength {
    /// `<length>`
    length: u32,
    /// `[<character length units>]`
    opt_units: Option<CharacterLengthUnits>,
}

/// Character length units of a string literal [(1)].
///
/// # Supported syntax
/// ```doc
/// [CHARACTERS|OCTETS]
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#char-length-units
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CharacterLengthUnits {
    /// `CHARACTERS`
    Characters,
    /// `OCTETS`
    Octets,
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

impl fmt::Display for CreateSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE SCHEMA {};", self.schema_name_clause())?;
        Ok(())
    }
}

impl DropSchema {
    /// Create a new `DropSchema`.
    ///
    /// The fields in the new statement are the obligatory fields. Optional
    /// fields should be set via `with_...` methods.
    #[must_use]
    pub fn new(schema_name: SchemaName, drop_behavior: DropBehavior) -> Self {
        Self {
            schema_name,
            drop_behavior,
        }
    }

    /// Returns a reference to the schema name.
    #[must_use]
    pub fn schema_name(&self) -> &SchemaName {
        &self.schema_name
    }

    /// Returns the drop behavior.
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
    /// Creates a new `ColumnDefinition`.
    ///
    /// The fields in the new statement are the obligatory fields. Optional
    /// fields should be set via `with_...` methods.
    #[must_use]
    pub fn new(column_name: &Ident) -> Self {
        Self {
            column_name: column_name.clone(),
            opt_data_type: None,
        }
    }

    /// Sets the column data type.
    pub fn with_data_type(&mut self, data_type: DataType) {
        self.opt_data_type = Some(data_type);
    }

    /// Returns the column name identifier.
    #[must_use]
    pub fn column_name(&self) -> &Ident {
        &self.column_name
    }

    /// Returns the column data type.
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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Character(opt_len) => {
                write!(f, "CHARACTER")?;

                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::Char(opt_len) => {
                write!(f, "CHAR")?;

                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::CharacterVarying(opt_len) => {
                write!(f, "CHARACTER VARYING")?;

                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::CharVarying(opt_len) => {
                write!(f, "CHAR VARYING")?;

                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::Varchar(opt_len) => {
                write!(f, "VARCHAR")?;

                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            DataType::Boolean => {
                write!(f, "BOOLEAN")?;
            }
        }

        Ok(())
    }
}

impl CharacterLength {
    /// Creates a new `CharacterLength`.
    ///
    /// Optional fields should be set via `with_...` methods.
    #[must_use]
    pub fn new(length: u32) -> Self {
        Self {
            length,
            opt_units: None,
        }
    }

    /// Sets the units attribute value.
    pub fn with_units(&mut self, opt_units: Option<CharacterLengthUnits>) -> &mut Self {
        self.opt_units = opt_units;
        self
    }

    /// Returns the length attribute value.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Returns the length attribute value.
    #[must_use]
    pub fn opt_units(&self) -> Option<CharacterLengthUnits> {
        self.opt_units
    }
}
impl fmt::Display for CharacterLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.length)?;

        if let Some(units) = self.opt_units() {
            write!(f, " {units}")?;
        }

        Ok(())
    }
}

impl fmt::Display for CharacterLengthUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Characters => write!(f, "CHARACTERS")?,
            Self::Octets => write!(f, "OCTETS")?,
        }
        Ok(())
    }
}
