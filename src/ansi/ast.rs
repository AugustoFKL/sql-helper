use nom::branch::alt;
use nom::IResult;

use crate::ansi::Statement;
use crate::common::Ident;

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
    Simple(Ident),
    /// AUTHORIZATION <schema authorization identifier>
    Authorization(Ident),
    /// <schema name> AUTHORIZATION <schema authorization identifier
    NamedAuthorization(Ident, Ident),
}

/// Qualified or unqualified identifier representing a schema.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SchemaName {
    /// Schema unqualified name.
    name: Ident,
    /// Optional catalog qualifier.
    opt_catalog_name: Option<Ident>,
}

impl SchemaName {
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

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_create_schema,))(input)
}

fn parse_create_schema(_input: &str) -> IResult<&str, Statement> {
    unimplemented!()
}
